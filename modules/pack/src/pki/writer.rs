//! # Code to write out a PKI file

use std::io::{self, Write};

use crate::common::writer::write_crc_tree;

use super::core::{FileRef, PackIndexFile};

const VERSION: u32 = 3;

/// Write out a PKI file
pub fn write_pki_file<W: Write>(writer: &mut W, value: &PackIndexFile) -> io::Result<()> {
    writer.write_all(&VERSION.to_le_bytes())?;
    writer.write_all(&(value.archives.len() as u32).to_le_bytes())?;
    for archive in &value.archives {
        writer.write_all(&(archive.path.len() as u32).to_le_bytes())?;
        writer.write_all(archive.path.as_bytes())?;
    }

    write_crc_tree(writer, &value.files, write_pki_entry)
}

fn write_pki_entry<W: Write>(writer: &mut W, value: &FileRef) -> io::Result<()> {
    writer.write_all(&value.pack_file.to_le_bytes())?;
    writer.write_all(&value.category.to_le_bytes())?;
    Ok(())
}
