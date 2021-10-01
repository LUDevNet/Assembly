//! # Parsers for the data
use super::file::*;
use assembly_core::{
    nom::{
        bytes::complete::{tag, take},
        combinator::{cond, map},
        multi::{count, fill, length_count},
        number::complete::{le_f32, le_u16, le_u32, le_u8},
        sequence::tuple,
        IResult,
    },
    parser::{
        parse_object_id, parse_object_template, parse_quat, parse_quat_wxyz, parse_u32_string,
        parse_u32_wstring, parse_vec3f,
    },
};
use std::convert::TryInto;

pub fn parse_chunk_version(input: &[u8]) -> IResult<&[u8], ChunkVersion> {
    map(tuple((le_u16, le_u16)), |(header, data)| ChunkVersion {
        header,
        data,
    })(input)
}

pub fn parse_chunk_header(input: &[u8]) -> IResult<&[u8], ChunkHeader> {
    let (input, _) = tag("CHNK")(input)?;
    let (input, id) = le_u32(input)?;
    let (input, version) = parse_chunk_version(input)?;
    let (input, size) = le_u32(input)?;
    let (input, offset) = le_u32(input)?;
    Ok((
        input,
        ChunkHeader {
            id,
            version,
            size,
            offset,
        },
    ))
}

pub fn parse_file_meta_chunk_data(input: &[u8]) -> IResult<&[u8], FileMetaChunkData> {
    let (input, version) = le_u32(input)?;
    let (input, revision) = le_u32(input)?;
    let (input, chunk_2000_offset) = le_u32(input)?;
    let (input, chunk_2001_offset) = le_u32(input)?;
    let (input, chunk_2002_offset) = le_u32(input)?;
    Ok((
        input,
        FileMetaChunkData {
            version,
            revision,
            chunk_2000_offset,
            chunk_2001_offset,
            chunk_2002_offset,
        },
    ))
}

fn parse_object_extra(input: &[u8]) -> IResult<&[u8], ObjectExtra> {
    let (input, field_1a) = take(32usize)(input)?;
    let (input, field_1b) = take(32usize)(input)?;
    let (input, field_2) = le_u32(input)?;
    let (input, field_3) = map(le_u8, |b| b != 0)(input)?;
    let (input, field_4) = count(le_u32, 16)(input)?;
    let (input, field_5) = take(3usize)(input)?;
    Ok((
        input,
        ObjectExtra {
            field_1a: field_1a.try_into().unwrap(),
            field_1b: field_1b.try_into().unwrap(),
            field_2,
            field_3,
            field_4: field_4[..].try_into().unwrap(),
            field_5: field_5.try_into().unwrap(),
        },
    ))
}

pub fn parse_objects_chunk_data<'a>(
    version: u32,
    i: &'a [u8],
) -> IResult<&'a [u8], ObjectsChunkData<String>> {
    let parse_object = move |i: &'a [u8]| -> IResult<&'a [u8], Object<String>> {
        let (i, obj_id) = parse_object_id(i)?;
        let (i, lot) = parse_object_template(i)?;
        let (i, asset_type) = cond(version >= 0x26, le_u32)(i)?;
        let (i, value_1) = cond(version >= 0x20, le_u32)(i)?;
        let (i, position) = parse_vec3f(i)?;
        let (i, rotation) = parse_quat_wxyz(i)?;
        let (i, scale) = le_f32(i)?;
        let (i, settings) = parse_u32_wstring(i)?;
        let (i, extra) = cond(version >= 0x07, length_count(le_u32, parse_object_extra))(i)?;
        Ok((
            i,
            Object {
                obj_id,
                lot,
                asset_type,
                value_1,
                position,
                rotation,
                scale,
                settings,
                extra: extra.unwrap_or_default(),
            },
        ))
    };

    let mut _parse_objects_chunk_data = map(length_count(le_u32, parse_object), |objects| {
        ObjectsChunkData { objects }
    });

    _parse_objects_chunk_data(i)
}

pub fn parse_env_chunk_data(input: &[u8]) -> IResult<&[u8], EnvironmentChunkData> {
    let (input, section1_address) = le_u32(input)?;
    let (input, sky_address) = le_u32(input)?;
    let (input, section3_address) = le_u32(input)?;
    Ok((
        input,
        EnvironmentChunkData {
            section1_address,
            sky_address,
            section3_address,
        },
    ))
}

fn parse_color(input: &[u8]) -> IResult<&[u8], Color> {
    map(tuple((le_f32, le_f32, le_f32)), Color::from)(input)
}

fn parse_section1_40(input: &[u8]) -> IResult<&[u8], Section1_40> {
    let (input, id) = le_u32(input)?;
    let (input, float1) = le_f32(input)?;
    let (input, float2) = le_f32(input)?;
    Ok((input, Section1_40 { id, float1, float2 }))
}

pub fn parse_section1<'a>(version: u32, input: &'a [u8]) -> IResult<&'a [u8], Section1> {
    let parse_section1_31e = |i: &'a [u8]| {
        if version >= 39 {
            let mut arr: [f32; 12] = Default::default();
            let (i, ()) = fill(le_f32, &mut arr)(i)?;
            let (i, array) = map(
                cond(version >= 40, length_count(le_u32, parse_section1_40)),
                Option::unwrap_or_default,
            )(i)?;
            Ok((
                i,
                Section1_39::After {
                    values: Box::new(arr),
                    array,
                },
            ))
        } else {
            let (i, value1) = le_f32(i)?;
            let (i, value2) = le_f32(i)?;
            Ok((i, Section1_39::Before { value1, value2 }))
        }
    };

    let parse_section1_31 = |i: &'a [u8]| {
        let (i, value1) = parse_section1_31e(i)?;
        let (i, value2) = parse_color(i)?;
        Ok((i, Section1_31 { value1, value2 }))
    };

    let parse_section1_43 = |i: &'a [u8]| {
        let (i, pos) = parse_vec3f(i)?;
        let (i, rot) = cond(version >= 33, parse_quat)(i)?;
        Ok((i, Section1_43 { pos, rot }))
    };

    let (input, value1) = cond(version >= 45, le_f32)(input)?;
    let (input, value2) = parse_color(input)?;
    let (input, value3) = parse_color(input)?;
    let (input, value4) = parse_color(input)?;
    let (input, value5) = parse_vec3f(input)?;
    let (input, value6) = cond(version >= 31, parse_section1_31)(input)?;
    let (input, value7) = cond(version >= 36, parse_color)(input)?;
    let (input, value8) = cond(version < 42, parse_section1_43)(input)?;
    Ok((
        input,
        Section1 {
            value1,
            value2,
            value3,
            value4,
            value5,
            value6,
            value7,
            value8,
        },
    ))
}

pub fn parse_sky_section(input: &[u8]) -> IResult<&[u8], SkySection> {
    let mut files: [String; 6] = Default::default();
    let (input, _) = fill(parse_u32_string, &mut files)(input)?;
    Ok((input, SkySection { files }))
}
