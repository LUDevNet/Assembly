//! ### Parsers for the data
use super::file::*;
use assembly_core::nom::{
    number::complete::{le_u8, le_u32, le_f32},
    named, do_parse
};

named!(pub parse_terrain_header<TerrainHeader>,
    do_parse!(
        version: le_u8 >>
        value_1: le_u8 >>
        value_2: le_u8 >>
        chunk_count: le_u32 >>
        width_in_chunks: le_u32 >>
        height_in_chunks: le_u32 >>
        (TerrainHeader{
            version, value_1, value_2,
            chunk_count, width_in_chunks, height_in_chunks
        })
    )
);

named!(pub parse_height_map_header<HeightMapHeader>,
    do_parse!(
        width: le_u32 >>
        height: le_u32 >>
        pos_x: le_f32 >>
        pos_z: le_f32 >>
        _1: le_u32 >>
        _2: le_u32 >>
        _3: le_u32 >>
        _4: le_u32 >>
        _5: le_f32 >>
        (HeightMapHeader{
            width, height, pos_x, pos_z, _1, _2, _3, _4, _5
        })
    )
);
