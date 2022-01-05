//! # Write PK files

use std::io::{self, Seek, SeekFrom, Write};

use crate::common::{writer::write_crc_tree, CRCTree};

use super::file::{PKEntryData, PKTrailer};

/// Write the directory of a PK file.
///
/// This function takes a [Write] implementation and a CRCTree<PKEntryData>
/// and writes the tree part of the PK directory to disk
fn write_pk_directory_tree<W: Write>(
    writer: &mut W,
    tree: &CRCTree<PKEntryData>,
) -> io::Result<()> {
    write_crc_tree(writer, tree, write_pk_entry_data)
}

/// Write the trailer of a PK file
fn write_pk_trailer<W: Write>(writer: &mut W, trailer: &PKTrailer) -> io::Result<()> {
    writer.write_all(&trailer.file_list_base_addr.to_le_bytes())?;
    writer.write_all(&trailer.num_compressed.to_le_bytes())?;
    Ok(())
}

/// Write the full directory to disk
///
/// For a "complete" PK file, this function takes the dictionary as a sorted tree
/// and writes the PK directory as well as the trailer to disk.
pub fn write_pk_directory<W: Write + Seek>(
    writer: &mut W,
    tree: &CRCTree<PKEntryData>,
) -> io::Result<()> {
    let file_list_base_addr = writer.seek(SeekFrom::Current(0))? as u32;
    let num_compressed = tree
        .iter()
        .filter(|(_, &x)| x.is_compressed & 0xFF > 0)
        .count() as u32;
    let trailer = PKTrailer {
        file_list_base_addr,
        num_compressed,
    };
    write_pk_directory_tree(writer, tree)?;
    write_pk_trailer(writer, &trailer)?;
    Ok(())
}

/// Write out a [PKEntryData]
fn write_pk_entry_data<W: Write>(writer: &mut W, entry: &PKEntryData) -> io::Result<()> {
    writer.write_all(&entry.orig_file_size.to_le_bytes())?;
    write!(writer, "{}\0\0\0\0", &entry.orig_file_hash)?;
    writer.write_all(&entry.compr_file_size.to_le_bytes())?;
    write!(writer, "{}\0\0\0\0", &entry.compr_file_hash)?;
    writer.write_all(&entry.file_data_addr.to_le_bytes())?;
    writer.write_all(&entry.is_compressed.to_le_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        md5::MD5Sum,
        pk::{file::PKEntryData, parser::parse_pk_entry_data, writer::write_pk_entry_data},
    };

    #[test]
    fn test_write_pk_entry() {
        let mut out: Vec<u8> = vec![];
        let orig_file_hash = MD5Sum([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let compr_file_hash = MD5Sum([
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        ]);
        let pke = PKEntryData {
            orig_file_size: 100,
            orig_file_hash,
            compr_file_size: 101,
            compr_file_hash,
            file_data_addr: 50,
            is_compressed: 256,
        };
        write_pk_entry_data(&mut out, &pke).unwrap();
        assert_eq!(
            out.as_slice(),
            &[
                100, 0, 0, 0, //
                b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6',
                b'0', b'7', b'0', b'8', b'0', b'9', b'0', b'a', b'0', b'b', b'0', b'c', b'0', b'd',
                b'0', b'e', b'0', b'f', 0, 0, 0, 0, //
                101, 0, 0, 0, //
                b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6',
                b'2', b'7', b'2', b'8', b'2', b'9', b'2', b'a', b'2', b'b', b'2', b'c', b'2', b'd',
                b'2', b'e', b'2', b'f', 0, 0, 0, 0, //
                50, 0, 0, 0, //
                0, 1, 0, 0
            ]
        );
        let (r, data) = parse_pk_entry_data(&out).unwrap();
        assert_eq!(r, &[] as &[u8]);
        assert_eq!(data, pke);
    }
}
