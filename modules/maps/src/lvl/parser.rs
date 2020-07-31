//! # Parsers for the data
use super::file::*;
use assembly_core::{
    nom::{
        cond, count, do_parse, length_count, map, named,
        number::complete::{le_f32, le_u16, le_u32, le_u8},
        tag, take, IResult,
    },
    parser::{
        parse_object_id, parse_object_template, parse_quat_wxyz, parse_u32_wstring, parse_vec3f,
    },
};
use std::convert::TryInto;

named!(pub parse_chunk_version<ChunkVersion>,
    do_parse!(
        header: le_u16 >>
        data: le_u16 >>
        (ChunkVersion{header, data})
    )
);

named!(pub parse_chunk_header<ChunkHeader>,
    do_parse!(
        tag!("CHNK") >>
        id: le_u32 >>
        version: parse_chunk_version >>
        size: le_u32 >>
        offset: le_u32 >>
        (ChunkHeader{id, version, size, offset})
    )
);

named!(pub parse_file_meta_chunk_data<FileMetaChunkData>,
    do_parse!(
        version: le_u32 >>
        revision: le_u32 >>
        chunk_2000_offset: le_u32 >>
        chunk_2001_offset: le_u32 >>
        chunk_2002_offset: le_u32 >>
        (FileMetaChunkData{
            version, revision,
            chunk_2000_offset, chunk_2001_offset, chunk_2002_offset
        })
    )
);

named!(
    parse_object_extra<ObjectExtra>,
    do_parse!(
        field_1a: take!(32)
            >> field_1b: take!(32)
            >> field_2: le_u32
            >> field_3: map!(le_u8, |b| b != 0)
            >> field_4: count!(le_u32, 16)
            >> field_5: take!(3)
            >> (ObjectExtra {
                field_1a: field_1a.try_into().unwrap(),
                field_1b: field_1b.try_into().unwrap(),
                field_2,
                field_3,
                field_4: field_4[..].try_into().unwrap(),
                field_5: field_5.try_into().unwrap(),
            })
    )
);

pub fn parse_objects_chunk_data<'a>(
    version: u32,
    i: &'a [u8],
) -> IResult<&'a [u8], ObjectsChunkData<String>> {
    let parse_object = move |i: &'a [u8]| -> IResult<&'a [u8], Object<String>> {
        do_parse!(
            i,
            obj_id: parse_object_id
                >> lot: parse_object_template
                >> asset_type: cond!(version >= 0x26, le_u32)
                >> value_1: cond!(version >= 0x20, le_u32)
                >> position: parse_vec3f
                >> rotation: parse_quat_wxyz
                >> scale: le_f32
                >> settings: parse_u32_wstring
                >> extra: cond!(version >= 0x07, length_count!(le_u32, parse_object_extra))
                >> (Object {
                    obj_id,
                    lot,
                    asset_type,
                    value_1,
                    position,
                    rotation,
                    scale,
                    settings,
                    extra: extra.unwrap_or_default(),
                })
        )
    };

    let _parse_objects_chunk_data =
        move |i: &'a [u8]| -> IResult<&'a [u8], ObjectsChunkData<String>> {
            map!(i, length_count!(le_u32, parse_object), |objects| {
                ObjectsChunkData { objects }
            })
        };

    _parse_objects_chunk_data(i)
}
