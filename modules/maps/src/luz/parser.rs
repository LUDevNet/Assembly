use assembly_core::nom::{
    call,
    combinator::map,
    cond, count, do_parse, length_count, map, map_res, named, named_args,
    number::complete::{le_u32, le_u64, le_u8},
    switch, take, value, IResult,
};
use std::convert::TryFrom;

use super::core::{
    FileVersion, SceneRef, SceneTransition, SceneTransitionInfo, SceneTransitionPoint, ZoneFile,
};
use assembly_core::nom_ext::{count_2, count_5};
use assembly_core::parser::{parse_quat, parse_u8_string, parse_vec3f, parse_world_id};
use assembly_core::types::Placement3D;

named!(pub parse_file_version<FileVersion>,
    map!(le_u32, FileVersion::from)
);

named_args!(pub parse_file_revision(version: FileVersion)<Option<u32>>,
    cond!(version.id() >= 0x24, call!(le_u32))
);

named_args!(pub parse_spawn_point(version: FileVersion)<Option<Placement3D>>,
    cond!(version.id() >= 0x26,
        do_parse!(a: parse_vec3f >> b: parse_quat >> (Placement3D{pos: a, rot: b}))
    )
);

named_args!(pub parse_scene_count(version: FileVersion)<usize>,
    switch!(value!(version.id() >= 0x25),
        true => map_res!(le_u32, usize::try_from) |
        false => map_res!(le_u8, usize::try_from)
    )
);

named!(
    parse_scene_ref<SceneRef>,
    do_parse!(
        file_name: parse_u8_string
            >> id: le_u32
            >> layer: le_u32
            >> name: parse_u8_string
            >> take!(3)
            >> (SceneRef {
                file_name,
                id,
                layer,
                name
            })
    )
);

named!(
    parse_scene_transition_point<SceneTransitionPoint>,
    do_parse!(
        a: le_u64
            >> b: parse_vec3f
            >> (SceneTransitionPoint {
                scene_id: a,
                point: b
            })
    )
);

fn parse_scene_transition_info(
    i: &[u8],
    version: FileVersion,
) -> IResult<&[u8], SceneTransitionInfo> {
    if version.id() <= 0x21 || version.id() >= 0x27 {
        map(
            count_2(parse_scene_transition_point),
            SceneTransitionInfo::from,
        )(i)
    } else {
        map(
            count_5(parse_scene_transition_point),
            SceneTransitionInfo::from,
        )(i)
    }
}

named_args!(parse_scene_transition(version: FileVersion)<SceneTransition>,
    do_parse!(
        name: cond!(version.id() < 0x25, parse_u8_string) >>
        points: call!(parse_scene_transition_info, version) >>
        (SceneTransition{ name, points })
    )
);

named_args!(parse_scene_transitions(version: FileVersion)<Option<Vec<SceneTransition>>>,
    cond!(version.id() >= 0x20,
        length_count!(le_u32, call!(parse_scene_transition, version))
    )
);

named!(pub parse_zone_file<ZoneFile<Vec<u8>>>,
    do_parse!(
        file_version: parse_file_version >>
        b: call!(parse_file_revision, file_version) >>
        c: parse_world_id >>
        d: call!(parse_spawn_point, file_version) >>
        e: call!(parse_scene_count, file_version) >>
        f: count!(parse_scene_ref, e) >>
        g: parse_u8_string >>
        h: parse_u8_string >>
        i: parse_u8_string >>
        j: parse_u8_string >>
        k: call!(parse_scene_transitions, file_version) >>
        l: cond!(file_version.min(0x23), length_count!(le_u32, le_u8)) >>
        (ZoneFile{
            file_version,
            file_revision: b,
            world_id: c,
            spawn_point: d,
            scene_refs: f,

            something: g,
            map_filename: h,
            map_name: i,
            map_description: j,

            scene_transitions: k,
            path_data: l,
        })
    )
);

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
        parse_scene_count(&[20], FileVersion::from(0x24)),
        Ok((&[][..], 20))
    );
    assert_eq!(
        parse_scene_count(&[20, 0, 0, 0], FileVersion::from(0x25)),
        Ok((&[][..], 20))
    );
    assert_eq!(
        parse_u8_string::<(&[u8], ErrorKind)>(&[2, 65, 66]),
        Ok((&[][..], String::from("AB")))
    );
}
