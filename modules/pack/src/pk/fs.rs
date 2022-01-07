//! # Interact with PK files in the file system

use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{common::CRCTree, txt::FileMeta};

use super::{
    file::{PKEntryData, PKTrailer, MAGIC_SEP, MAGIC_START},
    reader::PackFile,
    writer::{write_pk_directory_tree, write_pk_trailer},
};

/// Handle to a PK file
///
/// This is a handle to an open PK file. Open means that the dictionary is in memory, but it
/// holds a handle to the underlying file and can add files as needed.
pub struct PKHandle {
    /// The file handle
    file: File,
    /// The last write position & num compressed
    trailer: PKTrailer,
    /// The directory
    directory: CRCTree<PKEntryData>,
}

/// Inversion of control to put bytes into PK
pub trait PKWriter {
    /// Write the bytes into the file
    fn write<W: Write>(&mut self, writer: &mut W) -> io::Result<()>;
}

impl PKHandle {
    /// Open a PK file
    pub fn open(path: &Path) -> io::Result<PKHandle> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;
        let meta = file.metadata()?;
        let new = meta.len() == 0;

        let (directory, trailer) = if new {
            file.write_all(&MAGIC_START)?;

            let file_list_base_addr = MAGIC_START.len() as u32;
            (
                CRCTree::new(),
                PKTrailer {
                    file_list_base_addr,
                    num_compressed: 0,
                },
            )
        } else {
            let buf = BufReader::new(&mut file);
            let mut pk = PackFile::open(buf);

            pk.check_magic()?;

            let trailer = pk.get_header()?;
            let mut acc = pk.get_entry_accessor(trailer.file_list_base_addr)?;

            (acc.read_all()?, trailer)
        };

        Ok(PKHandle {
            file,
            directory,
            trailer,
        })
    }

    /// Put a file into the PK
    pub fn put_file<W: PKWriter>(
        &mut self,
        crc: u32,
        writer: &mut W,
        raw: FileMeta,
        compressed: FileMeta,
        is_compressed: bool,
    ) -> io::Result<()> {
        let mut buf = BufWriter::new(&mut self.file);
        let start = buf.seek(SeekFrom::Current(0))?;
        assert!(start <= u32::MAX.into());

        writer.write(&mut buf)?;
        buf.write_all(&MAGIC_SEP)?;
        let end = buf.seek(SeekFrom::Current(0))?;
        assert!(end <= u32::MAX.into());

        let is_compressed = if is_compressed { 0x01 } else { 0x00 };
        self.directory.insert(
            crc,
            PKEntryData {
                orig_file_size: raw.size,
                orig_file_hash: raw.hash,
                compr_file_size: compressed.size,
                compr_file_hash: compressed.hash,
                file_data_addr: start as u32,
                is_compressed,
            },
        );

        self.trailer.file_list_base_addr = end as u32;
        self.trailer.num_compressed += is_compressed;

        Ok(())
    }

    /// Finish the file by writing the directory
    pub fn finish(&mut self) -> io::Result<()> {
        let mut buf = BufWriter::new(&mut self.file);
        write_pk_directory_tree(&mut buf, &self.directory)?;
        write_pk_trailer(&mut buf, &self.trailer)?;
        // FIXME: truncate file if necessary
        Ok(())
    }
}
