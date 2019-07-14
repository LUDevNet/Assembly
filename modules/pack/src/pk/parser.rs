//! # Parsing functions

use super::file::*;
use assembly_core::nom::{
    number::complete::le_u32,
    named, do_parse, take, length_count, tag,
    IResult,
};
use std::convert::TryInto;

named!(pub parse_pk_magic<&[u8]>,
    tag!("ndpk")
);

named!(pub parse_pk_header<PKHeader>,
    do_parse!(
        file_list_base_addr: le_u32 >>
        value_1: le_u32 >>
        (PKHeader{file_list_base_addr, value_1})
    )
);

fn ascii_from_bytes(b: [u8; 32]) -> String {
    String::from_utf8_lossy(&b).to_string()
}

fn parse_hash(i: &[u8]) -> IResult<&[u8], String> {
    let (i, byte_slice) = take!(i, 32)?;
    // This cannot fail
    let bytes: [u8; 32] = byte_slice.try_into().unwrap();
    let hash = ascii_from_bytes(bytes);
    Ok((i, hash))
}

fn parse_compressed(i: &[u8]) -> IResult<&[u8], [u8;4]> {
    let (i, byte_slice) = take!(i, 4)?;
    // This cannot fail
    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
    Ok((i, bytes))
}

named!(pub parse_pk_entry<PKEntry>,
    do_parse!(
        crc: le_u32 >>
        left: le_u32 >>
        right: le_u32 >>
        orig_file_size: le_u32 >>
        orig_file_hash: parse_hash >>
        take!(4) >>
        compr_file_size: le_u32 >>
        compr_file_hash: parse_hash >>
        take!(4) >>
        file_data_addr: le_u32 >>
        is_compressed: parse_compressed >>
        (PKEntry{
            crc, left, right,
            orig_file_size, orig_file_hash,
            compr_file_size, compr_file_hash,
            file_data_addr, is_compressed,
        })
    )
);

named!(pub parse_pk_entry_list<Vec<PKEntry>>,
    length_count!(le_u32, parse_pk_entry)
);
