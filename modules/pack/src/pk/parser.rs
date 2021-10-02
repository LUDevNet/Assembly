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
        value_1: b,
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
