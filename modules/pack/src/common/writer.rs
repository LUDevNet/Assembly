//! # Writer for CRC trees

use std::{
    collections::BTreeMap,
    io::{self, Write},
    ops::Range,
};

/// Write a CRC tree to file
///
/// This function recursively subdivides the given `range` to implement a depth first tree-order traversal
///
/// For every leaf it calls [`Iterator::next`] and writes out the CRC key, left and right subindices
/// and then the payload using `write_value`.
fn write_crc_tree_recursive<V, W: Write, I: Iterator<Item = (u32, V)>>(
    writer: &mut W,
    iterator: &mut I,
    range: Range<u32>,
    write_value: fn(&mut W, V) -> io::Result<()>,
) -> io::Result<()> {
    let len = range.end - range.start;
    match len {
        0 => { /* Ignore */ }
        _ => {
            let mid = range.start + len / 2;

            let left_range = range.start..mid;
            let left_len = left_range.end - left_range.start;
            let left_ptr = if left_len == 0 {
                u32::MAX
            } else {
                left_range.start + left_len / 2
            };

            let right_range = (mid + 1)..range.end;
            let right_len = right_range.end - right_range.start;
            let right_ptr = if right_len == 0 {
                u32::MAX
            } else {
                right_range.start + right_len / 2
            };

            write_crc_tree_recursive(writer, iterator, left_range, write_value)?;

            let (crc, data) = iterator.next().unwrap();
            writer.write_all(&crc.to_le_bytes())?;
            writer.write_all(&left_ptr.to_le_bytes())?;
            writer.write_all(&right_ptr.to_le_bytes())?;
            write_value(writer, data)?;

            write_crc_tree_recursive(writer, iterator, right_range, write_value)?;
        }
    }

    Ok(())
}

/// Write a CRC tree to a writer
pub fn write_crc_tree<V, W: Write>(
    writer: &mut W,
    tree: &BTreeMap<u32, V>,
    write_value: fn(&mut W, &V) -> io::Result<()>,
) -> io::Result<()> {
    let len = tree.len() as u32;
    writer.write_all(&len.to_le_bytes())?;
    write_crc_tree_recursive(
        writer,
        &mut tree.iter().map(|(a, b)| (*a, b)),
        0..len,
        write_value,
    )
}
