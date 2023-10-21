use displaydoc::Display;
use encoding::{all::UTF_16LE, DecoderTrap, Encoding};
use ms_oforms::properties::types::{parse_position, parse_size};

use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag, take, take_until};
use nom::character::complete::one_of;
use nom::combinator::{eof, map, map_res, recognize, success};
use nom::multi::{count, fold_many0, many_till};
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

use super::core::*;

fn parse_wstring_nt(input: &[u8]) -> IResult<&[u8], String> {
    map_res(
        recognize(many_till(le_u16, tag([0x00, 0x00]))),
        |x: &[u8]| UTF_16LE.decode(&x[..(x.len() - 2)], DecoderTrap::Strict),
    )(input)
}

fn decode_utf16(input: &[u8]) -> Result<String, Cow<'static, str>> {
    UTF_16LE.decode(input, DecoderTrap::Strict)
}

fn parse_u32_bytes_wstring_nt(input: &[u8]) -> IResult<&[u8], String> {
    let (input, len) = le_u32(input)?;
    let (input, string) = map_res(take(len - 2), decode_utf16)(input)?;
    let (input, _) = tag([0x00, 0x00])(input)?;
    Ok((input, string))
}

fn parse_u32_wstring_nt(input: &[u8]) -> IResult<&[u8], String> {
    let (input, len) = le_u32(input)?;
    let (input, string) = map_res(take(len * 2 - 2), decode_utf16)(input)?;
    let (input, _) = tag([0x00, 0x00])(input)?;
    Ok((input, string))
}

pub fn parse_relationship(input: &str) -> IResult<&str, (String, String, String)> {
    let (input, _) = tag("Relationship '")(input)?;
    let (input, name) = take_until("'")(input)?;
    let (input, _) = tag("' between '")(input)?;
    let (input, from) = take_until("'")(input)?;
    let (input, _) = tag("' and '")(input)?;
    let (input, to) = take_until("'")(input)?;
    Ok((input, (name.to_string(), from.to_string(), to.to_string())))
}

fn parse_entry(input: &[u8]) -> IResult<&[u8], DSRefSchemaEntry> {
    let (input, k1) = le_u32(input)?;
    let (input, table) = parse_u32_bytes_wstring_nt(input)?;
    let (input, schema) = parse_u32_bytes_wstring_nt(input)?;
    Ok((input, DSRefSchemaEntry { k1, table, schema }))
}

fn parse_setting(input: &str) -> IResult<&str, (String, String)> {
    map(
        separated_pair(
            is_not::<_, &str, _>("="),
            tag("="),
            alt((
                delimited(
                    tag("\""),
                    escaped(is_not("\""), '\\', one_of("\"\\")),
                    tag("\""),
                ),
                is_not(";"),
            )),
        ),
        |(x, y)| (x.to_string(), y.to_string()),
    )(input)
}

//pub type StringMap = Vec<(String, String)>;
pub type StringMap = HashMap<String, String>;

fn parse_connection_setting(input: &str) -> IResult<&str, (String, String)> {
    let (input, set) = parse_setting(input)?;
    let (input, _) = alt((tag(";"), eof))(input)?;
    Ok((input, set))
}

fn parse_connection_string(input: &str) -> IResult<&str, StringMap> {
    fold_many0(
        parse_connection_setting,
        HashMap::new,
        |mut acc: StringMap, (key, value): (String, String)| {
            acc.insert(key, value);
            acc
        },
    )(input)
}

#[derive(Debug, Display)]
/// Failed to load settings from connection string
pub struct SettingsError;
impl std::error::Error for SettingsError {}

pub fn get_settings(val: String) -> Result<StringMap, SettingsError> {
    parse_connection_string(val.as_str())
        .map(|y| y.1)
        .map_err(|_| SettingsError)
}

pub fn parse_dsref_schema_contents(input: &[u8]) -> IResult<&[u8], DSRefSchemaContents> {
    let (input, _d1) = take(25usize)(input)?;
    let (input, len) = map(le_u8, usize::from)(input)?;
    let (input, _d2) = take(26usize)(input)?;
    let (input, connection) = parse_u32_bytes_wstring_nt(input)?;
    let (input, settings) = map_res(success(connection.clone()), get_settings)(input)?;
    let (input, _d3) = le_u32(input)?;
    let (input, name) = parse_u32_bytes_wstring_nt(input)?;
    let (input, tables) = count(parse_entry, len)(input)?;
    let (input, _d4) = take(22usize)(input)?;
    let (input, guid) = parse_u32_bytes_wstring_nt(input)?;
    Ok((
        input,
        DSRefSchemaContents {
            name,
            guid,
            tables,
            settings,
        },
    ))
}

pub fn parse_control1(input: &[u8]) -> IResult<&[u8], Control1> {
    let (input, pos_count) = le_u16(input)?;
    let (input, d1) = le_u16(input)?;
    let (input, positions) = count(parse_position, usize::from(pos_count))(input)?;
    let (input, _d2) = take(32usize)(input)?;
    let (input, d3) = le_u32(input)?;
    let (input, d4) = le_u32(input)?;
    let (input, pos) = parse_position(input)?;
    let (input, d5) = le_u32(input)?;
    let (input, d6) = le_u32(input)?;
    let (input, d7) = le_u32(input)?;
    let (input, _d8) = take(6usize)(input)?;
    let (input, d9) = le_u32(input)?;
    Ok((
        input,
        Control1 {
            positions,
            pos,
            d1,
            /*d2,*/ d3,
            d4,
            d5,
            d6,
            d7,
            /*d8,*/ d9,
        },
    ))
}

pub fn parse_sch_grid(input: &[u8]) -> IResult<&[u8], SchGrid> {
    let (input, d1) = le_u32(input)?;
    let (input, d2) = le_u32(input)?;
    let (input, size1) = parse_size(input)?;
    let (input, d3) = le_u32(input)?;
    let (input, d4) = le_u32(input)?;
    let (input, buf_len) = map_res(le_u32, usize::try_from)(input)?;
    let (input, name) = parse_wstring_nt(input)?;
    let (input, _) = take(buf_len - name.len() * 2 - 2)(input)?;
    let (input, _d5) = take(6usize * 4)(input)?;
    let (input, d6) = le_u32(input)?;
    let (input, _d7) = take(16usize * 4)(input)?;
    let (input, size2) = parse_size(input)?;
    let (input, _d8) = take(16usize * 4)(input)?;
    let (input, d9) = le_u32(input)?;
    let (input, _d10) = take(16usize * 4)(input)?;
    let (input, _d11) = take(11usize * 4)(input)?;
    let (input, d12) = le_u32(input)?;
    let (input, _d13) = take(2usize * 4)(input)?;
    let (input, some_len) = map_res(le_u32, usize::try_from)(input)?;
    let (input, d14) = count(le_u32, some_len)(input)?;
    let (input, schema) = parse_u32_wstring_nt(input)?;
    let (input, table) = parse_u32_wstring_nt(input)?;
    Ok((
        input,
        SchGrid {
            d1,
            d2,
            d3,
            d4,
            /*d5,*/ d6,
            /*d7, d8,*/ d9,
            /*d10, d11,*/ d12,
            /*d13,*/ d14,
            size1,
            name,
            size2,
            schema,
            table,
        },
    ))
}
