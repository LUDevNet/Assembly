//! The parsing of structures

use super::file::*;
use assembly_core::nom::{
    combinator::map, number::complete::le_u32, sequence::tuple, take, IResult,
};
use std::convert::TryInto;

fn u8_4(i: &[u8]) -> IResult<&[u8], [u8; 4]> {
    let (i, slice) = take!(i, 4)?;
    Ok((i, slice.try_into().unwrap()))
}

pub trait ParseLE: Sized {
    const BYTE_COUNT: usize;
    type Buf: AsMut<[u8]> + Default;
    fn parse(i: &[u8]) -> IResult<&[u8], Self>;
}

impl ParseLE for u32 {
    const BYTE_COUNT: usize = 4;
    type Buf = [u8; 4];
    fn parse(input: &[u8]) -> IResult<&[u8], u32> {
        le_u32(input)
    }
}

impl ParseLE for (u32, u32) {
    const BYTE_COUNT: usize = 8;
    type Buf = [u8; 8];
    fn parse(input: &[u8]) -> IResult<&[u8], (u32, u32)> {
        tuple((le_u32, le_u32))(input)
    }
}

impl ParseLE for (u32, [u8; 4]) {
    const BYTE_COUNT: usize = 8;
    type Buf = [u8; 8];
    fn parse(input: &[u8]) -> IResult<&[u8], (u32, [u8; 4])> {
        tuple((le_u32, u8_4))(input)
    }
}

impl ParseLE for (u32, u32, u32) {
    const BYTE_COUNT: usize = 12;
    type Buf = [u8; 12];
    fn parse(input: &[u8]) -> IResult<&[u8], (u32, u32, u32)> {
        tuple((le_u32, le_u32, le_u32))(input)
    }
}

pub trait ParseFDB {
    type IO: ParseLE;
    fn new(i: Self::IO) -> Self;
}

pub fn parse<T: ParseFDB>(input: &[u8]) -> IResult<&[u8], T> {
    map(T::IO::parse, T::new)(input)
}

impl ParseFDB for FDBTableDefHeader {
    type IO = (u32, u32, u32);

    fn new((a, b, c): Self::IO) -> Self {
        FDBTableDefHeader {
            column_count: a,
            table_name_addr: b,
            column_header_list_addr: c,
        }
    }
}

impl ParseFDB for FDBTableDataHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBTableDataHeader {
            bucket_count: a,
            bucket_header_list_addr: b,
        }
    }
}

impl ParseFDB for FDBColumnHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBColumnHeader {
            column_data_type: a,
            column_name_addr: b,
        }
    }
}

impl ParseFDB for FDBRowHeaderListEntry {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBRowHeaderListEntry {
            row_header_addr: a,
            row_header_list_next_addr: b,
        }
    }
}

impl ParseFDB for FDBRowHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBRowHeader {
            field_count: a,
            field_data_list_addr: b,
        }
    }
}

impl ParseFDB for FDBTableHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBTableHeader {
            table_def_header_addr: a,
            table_data_header_addr: b,
        }
    }
}

impl ParseFDB for FDBHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        FDBHeader {
            table_count: a,
            table_header_list_addr: b,
        }
    }
}

impl ParseFDB for FDBBucketHeader {
    type IO = u32;

    fn new(a: Self::IO) -> Self {
        FDBBucketHeader {
            row_header_list_head_addr: a,
        }
    }
}

impl ParseFDB for FDBFieldData {
    type IO = (u32, [u8; 4]);

    fn new((data_type, value): Self::IO) -> Self {
        FDBFieldData { data_type, value }
    }
}
