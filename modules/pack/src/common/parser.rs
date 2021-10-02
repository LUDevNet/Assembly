//! # Nom parsers

use std::string::FromUtf8Error;

use nom::{
    combinator::{map, map_res},
    error::{FromExternalError, ParseError},
    multi::length_data,
    number::complete::{le_i32, le_u32},
    IResult, Parser,
};

use super::CRCTreeNode;

/// Parse a CRC node
pub fn parse_crc_node<'r, D, P, E>(
    mut parser: P,
) -> impl FnMut(&'r [u8]) -> IResult<&'r [u8], CRCTreeNode<D>, E>
where
    P: Parser<&'r [u8], D, E>,
    E: ParseError<&'r [u8]>,
{
    move |input: &'r [u8]| -> IResult<&[u8], CRCTreeNode<D>, E> {
        let (input, crc) = le_u32(input)?;
        let (input, left) = le_i32(input)?;
        let (input, right) = le_i32(input)?;
        let (input, data) = parser.parse(input)?;
        Ok((
            input,
            CRCTreeNode {
                crc,
                left,
                right,
                data,
            },
        ))
    }
}

/// Parse a string after an u32 length specifier
pub fn parse_u32_string<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], FromUtf8Error>,
{
    map_res(map(length_data(le_u32), Vec::from), String::from_utf8)(input)
}
