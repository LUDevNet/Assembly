// use super::core::*;
// use crate::core::parser::{parse_string_u16};
use ms_oforms::properties::types::parser::{parse_size, parse_position};
use encoding::{Encoding, DecoderTrap, all::UTF_16LE};
use nom::{le_u32, le_u16, le_u8, IResult};
use super::core::{SchGrid, Control1};
use std::convert::TryFrom;

named!(parse_wstring_nt<String>,
    map_res!(
        recognize!(many_till!(le_u16, tag!([0x00, 0x00]))),
        |x: &[u8]| UTF_16LE.decode(&x[..(x.len() - 2)], DecoderTrap::Strict)
    )
);

named!(parse_u32_wstring_nt<String>,
    do_parse!(
        len: le_u32 >>
        string: map_res!(
            take!(len * 2 - 2),
            |x| UTF_16LE.decode(x, DecoderTrap::Strict)
        ) >>
        tag!([0x00, 0x00]) >>
        (string)
    )
);

named!(pub parse_relationship<&str, (String, String, String)>,
    do_parse!(
        tag!("Relationship '") >>
        name: take_until!("'") >>
        tag!("' between '") >>
        from: take_until!("'") >>
        tag!("' and '") >>
        to: take_until!("'") >>
        (name.to_string(), from.to_string(), to.to_string())
    )
);

pub fn parse_control1(input: &[u8]) -> IResult<&[u8], Control1> {
    do_parse!(input,
        pos_count: le_u16 >>
        d1: le_u16 >>
        positions: count!(parse_position, usize::from(pos_count)) >>
        d2: count_fixed!(u8, le_u8, 32) >>
        d3: le_u32 >>
        d4: le_u32 >>
        pos: parse_position >>
        d5: le_u32 >>
        d6: le_u32 >>
        d7: le_u32 >>
        d8: count_fixed!(u8, le_u8, 6) >>
        d9: le_u32 >>
        (Control1{
            positions, pos,
            d1, d2, d3, d4, d5, d6, d7, d8, d9,
        })
    )
}

pub fn parse_sch_grid(input: &[u8]) -> IResult<&[u8], SchGrid> {
    do_parse!(input,
        d1: le_u32 >>
        d2: le_u32 >>
        size1: parse_size >>
        d3: le_u32 >>
        d4: le_u32 >>
        buf_len: map_res!(le_u32, usize::try_from) >>
        name: parse_wstring_nt >>
        take!(buf_len - name.len() * 2 - 2) >>
        d5: count_fixed!(u32, le_u32, 6) >>
        d6: le_u32 >>
        d7: count_fixed!(u32, le_u32, 16) >>
        size2: parse_size >>
        d8: count_fixed!(u32, le_u32, 16) >>
        d9: le_u32 >>
        d10: count_fixed!(u32, le_u32, 16) >>
        d11: count_fixed!(u32, le_u32, 11) >>
        d12: le_u32 >>
        d13: count_fixed!(u32, le_u32, 2) >>
        some_len: map_res!(le_u32, usize::try_from) >>
        d14: count!(le_u32, some_len) >>
        schema: parse_u32_wstring_nt >>
        table: parse_u32_wstring_nt >>
        (SchGrid{
            d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14,
            size1, name, size2,
            schema, table,
        })
    )
}
