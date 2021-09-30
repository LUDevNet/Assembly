//! # Nom parsers

use assembly_core::nom::{
    error::ParseError,
    number::complete::{le_i32, le_u32},
    IResult, Parser,
};

use super::CRCTreeNode;

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
