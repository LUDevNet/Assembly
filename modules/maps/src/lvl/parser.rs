//! # Parsers for the data
use super::file::*;
use assembly_core::{
    nom::{
        cond, count, do_parse, length_count, map, named,
        number::complete::{le_f32, le_u16, le_u32, le_u8},
        tag, take, IResult,
    },
    parser::{
        parse_object_id, parse_object_template, parse_quat, parse_quat_wxyz, parse_u32_string,
        parse_u32_wstring, parse_vec3f,
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

named!(pub parse_env_chunk_data<EnvironmentChunkData>,
    do_parse!(
        section1_address: le_u32 >>
        sky_address: le_u32 >>
        section3_address: le_u32 >>
        (EnvironmentChunkData {
            section1_address,
            sky_address,
            section3_address,
        })
    )
);

named!(
    parse_color<Color>,
    do_parse!(red: le_f32 >> green: le_f32 >> blue: le_f32 >> (Color { red, green, blue }))
);

named!(
    parse_section1_40<Section1_40>,
    do_parse!(
        id: le_u32 >> float1: le_f32 >> float2: le_f32 >> (Section1_40 { id, float1, float2 })
    )
);

pub fn parse_section1<'a>(version: u32, i: &'a [u8]) -> IResult<&'a [u8], Section1> {
    let parse_section1_31e = |i: &'a [u8]| {
        if version >= 39 {
            do_parse!(
                i,
                value1: le_f32
                    >> value2: le_f32
                    >> value3: le_f32
                    >> value4: le_f32
                    >> value5: le_f32
                    >> value6: le_f32
                    >> value7: le_f32
                    >> value8: le_f32
                    >> value9: le_f32
                    >> value10: le_f32
                    >> value11: le_f32
                    >> value12: le_f32
                    >> array:
                        map!(
                            cond!(version >= 40, length_count!(le_u32, parse_section1_40)),
                            Option::unwrap_or_default
                        )
                    >> (Section1_39::After {
                        values: Box::new([
                            value1, value2, value3, value4, value5, value6, value7, value8, value9,
                            value10, value11, value12,
                        ]),
                        array,
                    })
            )
        } else {
            do_parse!(
                i,
                value1: le_f32 >> value2: le_f32 >> (Section1_39::Before { value1, value2 })
            )
        }
    };

    let parse_section1_31 = |i: &'a [u8]| {
        do_parse!(
            i,
            value1: parse_section1_31e >> value2: parse_color >> (Section1_31 { value1, value2 })
        )
    };

    let parse_section1_43 = |i: &'a [u8]| {
        do_parse!(
            i,
            pos: parse_vec3f >> rot: cond!(version >= 33, parse_quat) >> (Section1_43 { pos, rot })
        )
    };

    do_parse!(
        i,
        value1: cond!(version >= 45, le_f32)
            >> value2: parse_color
            >> value3: parse_color
            >> value4: parse_color
            >> value5: parse_vec3f
            >> value6: cond!(version >= 31, parse_section1_31)
            >> value7: cond!(version >= 36, parse_color)
            >> value8: cond!(version < 42, parse_section1_43)
            >> (Section1 {
                value1,
                value2,
                value3,
                value4,
                value5,
                value6,
                value7,
                value8,
            })
    )
}

named!(pub parse_sky_section<SkySection>,
    do_parse!(
        file1: parse_u32_string >>
        file2: parse_u32_string >>
        file3: parse_u32_string >>
        file4: parse_u32_string >>
        file5: parse_u32_string >>
        file6: parse_u32_string >>
        (SkySection {
            files: [file1, file2, file3, file4, file5, file6]
        })
    )
);
