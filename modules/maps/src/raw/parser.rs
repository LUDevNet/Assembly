//! ### Parsers for the data
use super::file::*;
use assembly_core::nom::{
    number::complete::{le_f32, le_u32, le_u8},
    IResult,
};

pub fn parse_terrain_header(input: &[u8]) -> IResult<&[u8], TerrainHeader> {
    let (input, version) = le_u8(input)?;
    let (input, value_1) = le_u8(input)?;
    let (input, value_2) = le_u8(input)?;
    let (input, chunk_count) = le_u32(input)?;
    let (input, width_in_chunks) = le_u32(input)?;
    let (input, height_in_chunks) = le_u32(input)?;
    Ok((
        input,
        TerrainHeader {
            version,
            value_1,
            value_2,
            chunk_count,
            width_in_chunks,
            height_in_chunks,
        },
    ))
}

#[allow(clippy::just_underscores_and_digits)]
pub fn parse_height_map_header(input: &[u8]) -> IResult<&[u8], HeightMapHeader> {
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    let (input, pos_x) = le_f32(input)?;
    let (input, pos_z) = le_f32(input)?;
    let (input, _1) = le_u32(input)?;
    let (input, _2) = le_u32(input)?;
    let (input, _3) = le_u32(input)?;
    let (input, _4) = le_u32(input)?;
    let (input, _5) = le_f32(input)?;
    Ok((
        input,
        HeightMapHeader {
            width,
            height,
            pos_x,
            pos_z,
            _1,
            _2,
            _3,
            _4,
            _5,
        },
    ))
}
