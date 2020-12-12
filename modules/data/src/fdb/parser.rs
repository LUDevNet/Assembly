//! # Parse structures from a byte buffer

use super::file::*;
use assembly_core::nom::{
    combinator::map, number::complete::le_u32, sequence::tuple, take, IResult,
};
use std::convert::TryInto;

fn u8_4(i: &[u8]) -> IResult<&[u8], [u8; 4]> {
    let (i, slice) = take!(i, 4)?;
    Ok((i, slice.try_into().unwrap()))
}

/// Marker trait that implies that `Self` can be parsed in little-endian mode
pub trait ParseLE: Sized + Copy {
    /// Same as `std::mem::size_of::<Self>()`
    const BYTE_COUNT: usize;
    /// A byte array of the same length that can be parsed as `Self`
    type Buf: AsMut<[u8]> + Default;
    /// Function to parse the buffer into self
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

/// Trait that implements parsing from a FDB file
pub trait ParseFDB: Sized + Copy {
    /// The [`ParseLE`] compatible type that is equivalent to `Self`
    type IO: ParseLE;
    /// Create `Self` from an instance of IO
    fn new(i: Self::IO) -> Self;

    /// Parse an FDB structure from a input slice
    ///
    /// This function chains [`ParseLE::parse`] with [`ParseFDB::new`]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(Self::IO::parse, Self::new)(input)
    }
}

impl ParseFDB for ArrayHeader {
    type IO = (u32, u32);

    fn new((a, b): Self::IO) -> Self {
        ArrayHeader {
            count: a,
            base_offset: b,
        }
    }
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
            buckets: ArrayHeader::new((a, b)),
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

    fn new(io: Self::IO) -> Self {
        FDBRowHeader {
            fields: ArrayHeader::from(io),
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
            tables: ArrayHeader::from((a, b)),
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
