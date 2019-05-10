//! # Parsing functions

use super::file::*;
use nom::{le_u32, le_u8};

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

named!(pub parse_pk_entry<PKEntry>,
    do_parse!(
        crc: le_u32 >>
        left: le_u32 >>
        right: le_u32 >>
        orig_file_size: le_u32 >>
        orig_file_hash: map!(count_fixed!(u8, le_u8, 32), ascii_from_bytes) >>
        take!(4) >>
        compr_file_size: le_u32 >>
        compr_file_hash: map!(count_fixed!(u8, le_u8, 32), ascii_from_bytes) >>
        take!(4) >>
        file_data_addr: le_u32 >>
        is_compressed: le_u32 >>
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
