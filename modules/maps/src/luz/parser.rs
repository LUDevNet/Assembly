use assembly_core::nom::{
    bytes::complete::take,
    combinator::{cond, map, map_res},
    multi::length_count,
    number::complete::{le_u32, le_u64, le_u8},
    IResult,
};
use std::convert::TryFrom;

use super::core::{
    FileVersion, SceneRef, SceneTransition, SceneTransitionInfo, SceneTransitionPoint, ZoneFile,
};
use assembly_core::nom_ext::{count_2, count_5};
use assembly_core::parser::{parse_quat, parse_u8_string, parse_vec3f, parse_world_id};
use assembly_core::types::Placement3D;

pub fn parse_file_version(input: &[u8]) -> IResult<&[u8], FileVersion> {
    map(le_u32, FileVersion::from)(input)
}

pub fn parse_file_revision(input: &[u8], version: FileVersion) -> IResult<&[u8], Option<u32>> {
    cond(version.id() >= 0x24, le_u32)(input)
}

pub fn parse_spawn_point<'a>(
    input: &'a [u8],
    version: FileVersion,
) -> IResult<&'a [u8], Option<Placement3D>> {
    let inner = |i: &'a [u8]| {
        let (i, a) = parse_vec3f(i)?;
        let (i, b) = parse_quat(i)?;
        Ok((i, Placement3D { pos: a, rot: b }))
    };

    cond(version.id() >= 0x26, inner)(input)
}

pub fn parse_scene_count(version: FileVersion) -> fn(input: &[u8]) -> IResult<&[u8], usize> {
    fn pre_x25(input: &[u8]) -> IResult<&[u8], usize> {
        map_res(le_u8, usize::try_from)(input)
    }

    fn post_x25(input: &[u8]) -> IResult<&[u8], usize> {
        map_res(le_u32, usize::try_from)(input)
    }

    if version.id() >= 0x25 {
        post_x25
    } else {
        pre_x25
    }
}

fn parse_scene_ref(input: &[u8]) -> IResult<&[u8], SceneRef> {
    let (input, file_name) = parse_u8_string(input)?;
    let (input, id) = le_u32(input)?;
    let (input, layer) = le_u32(input)?;
    let (input, name) = parse_u8_string(input)?;
    let (input, _) = take(3usize)(input)?;
    Ok((
        input,
        SceneRef {
            file_name,
            id,
            layer,
            name,
        },
    ))
}

fn parse_scene_transition_point(input: &[u8]) -> IResult<&[u8], SceneTransitionPoint> {
    let (input, a) = le_u64(input)?;
    let (input, b) = parse_vec3f(input)?;
    Ok((
        input,
        SceneTransitionPoint {
            scene_id: a,
            point: b,
        },
    ))
}

fn parse_scene_transition_info(
    version: FileVersion,
) -> fn(&[u8]) -> IResult<&[u8], SceneTransitionInfo> {
    fn x22_to_x26(i: &[u8]) -> IResult<&[u8], SceneTransitionInfo> {
        map(
            count_5(parse_scene_transition_point),
            SceneTransitionInfo::from,
        )(i)
    }

    fn post_x27(i: &[u8]) -> IResult<&[u8], SceneTransitionInfo> {
        map(
            count_2(parse_scene_transition_point),
            SceneTransitionInfo::from,
        )(i)
    }

    if version.id() <= 0x21 || version.id() >= 0x27 {
        post_x27
    } else {
        x22_to_x26
    }
}

fn parse_scene_transition(
    version: FileVersion,
) -> impl Fn(&[u8]) -> IResult<&[u8], SceneTransition> + Copy {
    let sti_parser = parse_scene_transition_info(version);
    move |i: &[u8]| {
        let (i, name) = cond(version.id() < 0x25, parse_u8_string)(i)?;
        let (i, points) = sti_parser(i)?;
        Ok((i, SceneTransition { name, points }))
    }
}

fn parse_scene_transitions(
    version: FileVersion,
) -> impl Fn(&[u8]) -> IResult<&[u8], Option<Vec<SceneTransition>>> {
    let st_parser = parse_scene_transition(version);
    move |i: &[u8]| cond(version.id() >= 0x20, length_count(le_u32, st_parser))(i)
}

#[allow(clippy::many_single_char_names)]
pub fn parse_zone_file(input: &[u8]) -> IResult<&[u8], ZoneFile<Vec<u8>>> {
    let (input, file_version) = parse_file_version(input)?;
    let sc_parser = parse_scene_count(file_version);
    let st_parser = parse_scene_transitions(file_version);

    let (input, file_revision) = parse_file_revision(input, file_version)?;
    let (input, world_id) = parse_world_id(input)?;
    let (input, spawn_point) = parse_spawn_point(input, file_version)?;
    let (input, scene_refs) = length_count(sc_parser, parse_scene_ref)(input)?;
    let (input, g) = parse_u8_string(input)?;
    let (input, map_filename) = parse_u8_string(input)?;
    let (input, map_name) = parse_u8_string(input)?;
    let (input, map_description) = parse_u8_string(input)?;
    let (input, scene_transitions) = st_parser(input)?;
    let (input, path_data) = cond(file_version.min(0x23), length_count(le_u32, le_u8))(input)?;
    Ok((
        input,
        ZoneFile {
            file_version,
            file_revision,
            world_id,
            spawn_point,
            scene_refs,

            something: g,
            map_filename,
            map_name,
            map_description,

            scene_transitions,
            path_data,
        },
    ))
}

#[test]
fn test_parse() {
    use assembly_core::nom::error::ErrorKind;

    assert_eq!(
        parse_file_revision(&[20, 0, 0, 0], FileVersion::from(0x24)),
        Ok((&[][..], Some(20)))
    );
    assert_eq!(
        parse_file_revision(&[20, 0, 0, 0], FileVersion::from(0x23)),
        Ok((&[20, 0, 0, 0][..], None))
    );
    assert_eq!(
        parse_scene_count(FileVersion::from(0x24))(&[20]),
        Ok((&[][..], 20))
    );
    assert_eq!(
        parse_scene_count(FileVersion::from(0x25))(&[20, 0, 0, 0]),
        Ok((&[][..], 20))
    );
    assert_eq!(
        parse_u8_string::<(&[u8], ErrorKind)>(&[2, 65, 66]),
        Ok((&[][..], String::from("AB")))
    );
}
