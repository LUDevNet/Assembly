//! # Parsing functions

use std::convert::TryInto;

use crate::{common::parser::parse_crc_node, md5::MD5Sum};

use super::file::*;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    multi::length_count,
    number::complete::le_u32,
    sequence::tuple,
    IResult,
};

/// Parse the magic bytes
pub fn parse_pk_magic(input: &[u8]) -> IResult<&[u8], ()> {
    let (rest, _) = tag("ndpk")(input)?;
    Ok((rest, ()))
}

/// Parse the trailer
pub fn parse_pk_trailer(input: &[u8]) -> IResult<&[u8], PKTrailer> {
    map(tuple((le_u32, le_u32)), |(a, b)| PKTrailer {
        file_list_base_addr: a,
        num_compressed: b,
    })(input)
}

fn parse_hash(i: &[u8]) -> IResult<&[u8], MD5Sum> {
    map_res(take(32usize), MD5Sum::from_hex_bytes)(i)
}

fn parse_compressed(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, byte_slice) = take(4usize)(i)?;
    // This cannot fail
    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
    Ok((i, u32::from_le_bytes(bytes)))
}

/// Parse a file list entry
pub fn parse_pk_entry_data(input: &[u8]) -> IResult<&[u8], PKEntryData> {
    let (input, orig_file_size) = le_u32(input)?;
    let (input, orig_file_hash) = parse_hash(input)?;
    let (input, _ofh_padding) = take(4usize)(input)?;
    let (input, compr_file_size) = le_u32(input)?;
    let (input, compr_file_hash) = parse_hash(input)?;
    let (input, _cfh_padding) = take(4usize)(input)?;
    let (input, file_data_addr) = le_u32(input)?;
    let (input, is_compressed) = parse_compressed(input)?;
    Ok((
        input,
        PKEntryData {
            orig_file_size,
            orig_file_hash,
            compr_file_size,
            compr_file_hash,
            file_data_addr,
            is_compressed,
        },
    ))
}

/// Parse a file list entry
pub fn parse_pk_entry(input: &[u8]) -> IResult<&[u8], PKEntry> {
    parse_crc_node(parse_pk_entry_data)(input)
}

/// Parse the file list
pub fn parse_pk_entry_list(input: &[u8]) -> IResult<&[u8], Vec<PKEntry>> {
    length_count(le_u32, parse_pk_entry)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_magic() {
        assert_eq!(parse_pk_magic(b"ndpk"), Ok((EMPTY, ())));
    }

    #[test]
    fn test_trailer() {
        assert_eq!(
            parse_pk_trailer(&[0x20, 0x10, 0, 0, 0, 0, 0, 0]),
            Ok((
                EMPTY,
                PKTrailer {
                    file_list_base_addr: 0x1020,
                    num_compressed: 0,
                }
            ))
        )
    }

    const ARR: [u8; 16] = [
        0x33, 0x7e, 0x24, 0xd7, 0x26, 0xfd, 0x72, 0x8f, //
        0x92, 0x95, 0x7a, 0x2c, 0x90, 0x8d, 0xde, 0xe6,
    ];

    const ARR2: [u8; 16] = [
        0x44, 0x7e, 0x24, 0xd7, 0x26, 0xfd, 0x72, 0x8f, //
        0x92, 0x95, 0x7a, 0x2c, 0x90, 0x8d, 0xde, 0xe6,
    ];

    #[test]
    fn test_parse_hash() {
        assert_eq!(
            parse_hash(b"337e24d726fd728f92957a2c908ddee6"),
            Ok((EMPTY, MD5Sum(ARR)))
        );
    }

    #[test]
    fn test_entry() {
        let mut data = Vec::new();
        data.extend_from_slice(&u32::to_le_bytes(1234)); // CRC
        data.extend_from_slice(&i32::to_le_bytes(100)); // left
        data.extend_from_slice(&i32::to_le_bytes(200)); // right

        data.extend_from_slice(&u32::to_le_bytes(4444)); // file size
        data.extend_from_slice(b"337e24d726fd728f92957a2c908ddee6");
        data.extend_from_slice(&[0, 0, 0, 0]);

        data.extend_from_slice(&u32::to_le_bytes(8888)); // file size
        data.extend_from_slice(b"447e24d726fd728f92957a2c908ddee6");
        data.extend_from_slice(&[0, 0, 0, 0]);

        data.extend_from_slice(&u32::to_le_bytes(5678)); // file offset
        data.extend_from_slice(&[1, 0, 0, 0]); // compressed

        assert_eq!(
            parse_pk_entry(&data),
            Ok((
                EMPTY,
                PKEntry {
                    crc: 1234,
                    left: 100,
                    right: 200,
                    data: PKEntryData {
                        orig_file_size: 4444,
                        orig_file_hash: MD5Sum(ARR),
                        compr_file_size: 8888,
                        compr_file_hash: MD5Sum(ARR2),
                        file_data_addr: 5678,
                        is_compressed: 1,
                    }
                }
            ))
        );
    }
}
