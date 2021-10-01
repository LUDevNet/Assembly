use super::core::*;
use assembly_core::nom::{
    combinator::{cond, map_opt, map_res},
    multi::{fold_many_m_n, length_count},
    number::complete::{le_f32, le_u32, le_u8},
    sequence::tuple,
    IResult,
};
use assembly_core::parser::{
    parse_object_id, parse_object_template, parse_quat, parse_quat_wxyz, parse_u32_wstring,
    parse_u8_bool, parse_u8_wstring, parse_vec3f, parse_world_id,
};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn parse_zone_paths_version(input: &[u8]) -> IResult<&[u8], ZonePathsVersion> {
    map_opt(le_u32, ZonePathsVersion::from_u32)(input)
}

pub fn parse_path_version(input: &[u8]) -> IResult<&[u8], PathVersion> {
    map_opt(le_u32, PathVersion::from_u32)(input)
}

pub fn parse_path_type(input: &[u8]) -> IResult<&[u8], PathType> {
    map_opt(le_u32, PathType::from_u32)(input)
}

pub fn parse_path_composition(input: &[u8]) -> IResult<&[u8], PathComposition> {
    map_opt(le_u32, PathComposition::from_u32)(input)
}

pub fn parse_path_data_movement(input: &[u8]) -> IResult<&[u8], PathDataMovement> {
    Ok((input, PathDataMovement {}))
}

pub fn parse_path_data_moving_platform(
    version: PathVersion,
) -> fn(&[u8]) -> IResult<&[u8], PathDataMovingPlatform> {
    fn pre_v13(i: &[u8]) -> IResult<&[u8], PathDataMovingPlatform> {
        Ok((i, PathDataMovingPlatform::PreV13))
    }
    fn v13_to_17(i: &[u8]) -> IResult<&[u8], PathDataMovingPlatform> {
        let (i, platform_travel_sound) = parse_u8_wstring(i)?;
        Ok((
            i,
            PathDataMovingPlatform::V13ToV17 {
                platform_travel_sound,
            },
        ))
    }
    fn post_v18(i: &[u8]) -> IResult<&[u8], PathDataMovingPlatform> {
        let (i, something) = le_u8(i)?;
        Ok((i, PathDataMovingPlatform::PostV18 { something }))
    }
    match version.id() {
        0..=12 => pre_v13,
        13..=17 => v13_to_17,
        _ => post_v18,
    }
}

pub fn parse_property_rental_time_unit(input: &[u8]) -> IResult<&[u8], PropertyRentalTimeUnit> {
    map_opt(le_u32, PropertyRentalTimeUnit::from_u32)(input)
}

pub fn parse_property_achievement_required(
    input: &[u8],
) -> IResult<&[u8], PropertyAchievementRequired> {
    map_opt(le_u32, PropertyAchievementRequired::from_u32)(input)
}

pub fn parse_path_data_property(input: &[u8]) -> IResult<&[u8], PathDataProperty> {
    let (input, value_1) = le_u32(input)?;
    let (input, price) = le_u32(input)?;
    let (input, rental_time) = le_u32(input)?;
    let (input, associated_map) = parse_world_id(input)?;
    let (input, value_2) = le_u32(input)?;
    let (input, display_name) = parse_u8_wstring(input)?;
    let (input, display_description) = parse_u32_wstring(input)?;
    let (input, value_3) = le_u32(input)?;
    let (input, clone_limit) = le_u32(input)?;
    let (input, reputation_multiplier) = le_f32(input)?;
    let (input, rental_time_unit) = parse_property_rental_time_unit(input)?;
    let (input, achievement_required) = parse_property_achievement_required(input)?;
    let (input, player_zone_coordinate) = parse_vec3f(input)?;
    let (input, max_build_height) = le_f32(input)?;
    Ok((
        input,
        PathDataProperty {
            value_1,
            price,
            rental_time,
            associated_map,
            value_2,
            display_name,
            display_description,
            value_3,
            clone_limit,
            reputation_multiplier,
            rental_time_unit,
            achievement_required,
            player_zone_coordinate,
            max_build_height,
        },
    ))
}

pub fn parse_path_data_camera<'a>(
    version: PathVersion,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], PathDataCamera> {
    let mut v1_parser = cond(version.min(14), le_u8);
    move |i: &'a [u8]| {
        let (i, next_path) = parse_u8_wstring(i)?;
        let (i, value_1) = v1_parser(i)?;
        Ok((i, PathDataCamera { next_path, value_1 }))
    }
}

pub fn parse_path_data_spawner(input: &[u8]) -> IResult<&[u8], PathDataSpawner> {
    let (input, spawned_lot) = parse_object_template(input)?;
    let (input, respawn_time) = le_u32(input)?;
    let (input, max_to_spawn) = le_u32(input)?;
    let (input, min_to_spawn) = le_u32(input)?;
    let (input, spawner_obj_id) = parse_object_id(input)?;
    let (input, activate_network_on_load) = parse_u8_bool(input)?;
    Ok((
        input,
        PathDataSpawner {
            spawned_lot,
            respawn_time,
            max_to_spawn,
            min_to_spawn,
            spawner_obj_id,
            activate_network_on_load,
        },
    ))
}

pub fn parse_path_data_showcase(input: &[u8]) -> IResult<&[u8], PathDataShowcase> {
    Ok((input, PathDataShowcase {}))
}

pub fn parse_path_data_race(input: &[u8]) -> IResult<&[u8], PathDataRace> {
    Ok((input, PathDataRace {}))
}

pub fn parse_path_data_rail(input: &[u8]) -> IResult<&[u8], PathDataRail> {
    Ok((input, PathDataRail {}))
}

pub fn parse_path_waypoint_data_movement(input: &[u8]) -> IResult<&[u8], PathWaypointDataMovement> {
    let (input, config) = parse_waypoint_config(input)?;
    Ok((input, PathWaypointDataMovement { config }))
}

pub fn parse_path_waypoint_data_moving_platform_sounds(
    input: &[u8],
) -> IResult<&[u8], PathWaypointDataMovingPlatformSounds> {
    let (input, depart_sound) = parse_u8_wstring(input)?;
    let (input, arrive_sound) = parse_u8_wstring(input)?;
    Ok((
        input,
        PathWaypointDataMovingPlatformSounds {
            depart_sound,
            arrive_sound,
        },
    ))
}

pub fn parse_path_waypoint_data_moving_platform<'a>(
    version: PathVersion,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], PathWaypointDataMovingPlatform> {
    let mut sounds_parser = cond(
        version.min(13),
        parse_path_waypoint_data_moving_platform_sounds,
    );
    move |input: &'a [u8]| {
        let (input, rotation) = parse_quat(input)?;
        let (input, lock_player) = parse_u8_bool(input)?;
        let (input, speed) = le_f32(input)?;
        let (input, wait) = le_f32(input)?;
        let (input, sounds) = sounds_parser(input)?;
        Ok((
            input,
            PathWaypointDataMovingPlatform {
                rotation,
                lock_player,
                speed,
                wait,
                sounds,
            },
        ))
    }
}

pub fn parse_path_waypoint_data_property(input: &[u8]) -> IResult<&[u8], PathWaypointDataProperty> {
    Ok((input, PathWaypointDataProperty {}))
}

pub fn parse_path_waypoint_data_camera(input: &[u8]) -> IResult<&[u8], PathWaypointDataCamera> {
    let (input, rotation) = parse_quat(input)?;
    let (input, time) = le_f32(input)?;
    let (input, value_5) = le_f32(input)?;
    let (input, tension) = le_f32(input)?;
    let (input, continuity) = le_f32(input)?;
    let (input, bias) = le_f32(input)?;
    Ok((
        input,
        PathWaypointDataCamera {
            rotation,
            time,
            value_5,
            tension,
            continuity,
            bias,
        },
    ))
}

pub fn parse_path_waypoint_data_spawner(input: &[u8]) -> IResult<&[u8], PathWaypointDataSpawner> {
    let (input, rotation) = parse_quat(input)?;
    let (input, config) = parse_waypoint_config(input)?;
    Ok((input, PathWaypointDataSpawner { rotation, config }))
}

pub fn parse_path_waypoint_data_showcase(input: &[u8]) -> IResult<&[u8], PathWaypointDataShowcase> {
    Ok((input, PathWaypointDataShowcase {}))
}

pub fn parse_path_waypoint_data_race(input: &[u8]) -> IResult<&[u8], PathWaypointDataRace> {
    let (input, rotation) = parse_quat(input)?;
    let (input, value_1) = le_u8(input)?;
    let (input, value_2) = le_u8(input)?;
    let (input, value_3) = le_f32(input)?;
    let (input, value_4) = le_f32(input)?;
    let (input, value_5) = le_f32(input)?;
    Ok((
        input,
        PathWaypointDataRace {
            rotation,
            value_1,
            value_2,
            value_3,
            value_4,
            value_5,
        },
    ))
}

fn parse_path_waypoint_variant_movement(
    input: &[u8],
) -> IResult<&[u8], PathWaypointVariantMovement> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_movement(input)?;
    Ok((input, PathWaypointVariantMovement { position, data }))
}

fn parse_path_variant_movement(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantMovement> {
    let (input, path_data) = parse_path_data_movement(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_movement)(input)?;
    Ok((
        input,
        PathVariantMovement {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_moving_platform<'a>(
    version: PathVersion,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], PathWaypointVariantMovingPlatform> {
    let mut inner = parse_path_waypoint_data_moving_platform(version);
    move |i: &'a [u8]| {
        let (i, position) = parse_vec3f(i)?;
        let (i, data) = inner(i)?;
        Ok((i, PathWaypointVariantMovingPlatform { position, data }))
    }
}

fn parse_path_variant_moving_platform(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantMovingPlatform> {
    let (input, path_data) = parse_path_data_moving_platform(header.version)(input)?;
    let (input, waypoints) = length_count(
        le_u32,
        parse_path_waypoint_variant_moving_platform(header.version),
    )(input)?;
    Ok((
        input,
        PathVariantMovingPlatform {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_property(
    input: &[u8],
) -> IResult<&[u8], PathWaypointVariantProperty> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_property(input)?;
    Ok((input, PathWaypointVariantProperty { position, data }))
}

fn parse_path_variant_property(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantProperty> {
    let (input, path_data) = parse_path_data_property(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_property)(input)?;
    Ok((
        input,
        PathVariantProperty {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_camera(input: &[u8]) -> IResult<&[u8], PathWaypointVariantCamera> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_camera(input)?;
    Ok((input, PathWaypointVariantCamera { position, data }))
}

fn parse_path_variant_camera(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantCamera> {
    let (input, path_data) = parse_path_data_camera(header.version)(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_camera)(input)?;
    Ok((
        input,
        PathVariantCamera {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_spawner(input: &[u8]) -> IResult<&[u8], PathWaypointVariantSpawner> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_spawner(input)?;
    Ok((input, PathWaypointVariantSpawner { position, data }))
}

fn parse_path_variant_spawner(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantSpawner> {
    let (input, path_data) = parse_path_data_spawner(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_spawner)(input)?;
    Ok((
        input,
        PathVariantSpawner {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_showcase(
    input: &[u8],
) -> IResult<&[u8], PathWaypointVariantShowcase> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_showcase(input)?;
    Ok((input, PathWaypointVariantShowcase { position, data }))
}

fn parse_path_variant_showcase(
    input: &[u8],
    header: PathHeader,
) -> IResult<&[u8], PathVariantShowcase> {
    let (input, path_data) = parse_path_data_showcase(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_showcase)(input)?;
    Ok((
        input,
        PathVariantShowcase {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_race(input: &[u8]) -> IResult<&[u8], PathWaypointVariantRace> {
    let (input, position) = parse_vec3f(input)?;
    let (input, data) = parse_path_waypoint_data_race(input)?;
    Ok((input, PathWaypointVariantRace { position, data }))
}

fn parse_path_variant_race(input: &[u8], header: PathHeader) -> IResult<&[u8], PathVariantRace> {
    let (input, path_data) = parse_path_data_race(input)?;
    let (input, waypoints) = length_count(le_u32, parse_path_waypoint_variant_race)(input)?;
    Ok((
        input,
        PathVariantRace {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_waypoint_variant_rail(input: &[u8]) -> IResult<&[u8], PathWaypointVariantRail> {
    let (input, position) = parse_vec3f(input)?;
    let (input, rotation) = parse_quat_wxyz(input)?;
    let (input, config) = parse_waypoint_config(input)?;
    Ok((
        input,
        PathWaypointVariantRail {
            position,
            data: PathWaypointDataRail {
                rotation,
                speed: None,
                config,
            },
        },
    ))
}

fn parse_path_waypoint_variant_rail_17(input: &[u8]) -> IResult<&[u8], PathWaypointVariantRail> {
    let (input, position) = parse_vec3f(input)?;
    let (input, rotation) = parse_quat_wxyz(input)?;
    let (input, speed) = le_f32(input)?;
    let (input, config) = parse_waypoint_config(input)?;
    Ok((
        input,
        PathWaypointVariantRail {
            position,
            data: PathWaypointDataRail {
                rotation,
                speed: Some(speed),
                config,
            },
        },
    ))
}

fn parse_path_variant_rail(input: &[u8], header: PathHeader) -> IResult<&[u8], PathVariantRail> {
    let version = header.version;
    let waypoint_parser = if version.min(17) {
        parse_path_waypoint_variant_rail_17
    } else {
        parse_path_waypoint_variant_rail
    };
    let (input, path_data) = parse_path_data_rail(input)?;
    let (input, waypoints) = length_count(le_u32, waypoint_parser)(input)?;
    Ok((
        input,
        PathVariantRail {
            header,
            path_data,
            waypoints,
        },
    ))
}

fn parse_path_data(inp: &[u8], path_type: PathType, header: PathHeader) -> IResult<&[u8], Path> {
    match path_type {
        PathType::Movement => {
            let (inp, var) = parse_path_variant_movement(inp, header)?;
            Ok((inp, Path::Movement(var)))
        }
        PathType::MovingPlatform => {
            let (inp, var) = parse_path_variant_moving_platform(inp, header)?;
            Ok((inp, Path::MovingPlatform(var)))
        }
        PathType::Property => {
            let (inp, var) = parse_path_variant_property(inp, header)?;
            Ok((inp, Path::Property(var)))
        }
        PathType::Camera => {
            let (inp, var) = parse_path_variant_camera(inp, header)?;
            Ok((inp, Path::Camera(var)))
        }
        PathType::Spawner => {
            let (inp, var) = parse_path_variant_spawner(inp, header)?;
            Ok((inp, Path::Spawner(var)))
        }
        PathType::Showcase => {
            let (inp, var) = parse_path_variant_showcase(inp, header)?;
            Ok((inp, Path::Showcase(var)))
        }
        PathType::Race => {
            let (inp, var) = parse_path_variant_race(inp, header)?;
            Ok((inp, Path::Race(var)))
        }
        PathType::Rail => {
            let (inp, var) = parse_path_variant_rail(inp, header)?;
            Ok((inp, Path::Rail(var)))
        }
    }
}

pub fn parse_path(input: &[u8]) -> IResult<&[u8], Path> {
    let (input, version) = parse_path_version(input)?;
    let (input, path_name) = parse_u8_wstring(input)?;
    let (input, path_type) = parse_path_type(input)?;
    let (input, value_1) = le_u32(input)?;
    let (input, path_composition) = parse_path_composition(input)?;
    let header = PathHeader {
        version,
        path_name,
        value_1,
        path_composition,
    };
    parse_path_data(input, path_type, header)
}

pub fn parse_waypoint_config_entry(input: &[u8]) -> IResult<&[u8], (String, String)> {
    tuple((parse_u8_wstring, parse_u8_wstring))(input)
}

fn extend_config_map(
    mut map: HashMap<String, String>,
    entry: (String, String),
) -> HashMap<String, String> {
    map.insert(entry.0, entry.1);
    map
}

pub fn parse_waypoint_config(input: &[u8]) -> IResult<&[u8], WaypointConfig> {
    let (input, count) = map_res(le_u32, usize::try_from)(input)?;
    fold_many_m_n(
        count,
        count,
        parse_waypoint_config_entry,
        HashMap::new,
        extend_config_map,
    )(input)
}

pub fn parse_zone_paths(input: &[u8]) -> IResult<&[u8], ZonePaths> {
    let (input, version) = parse_zone_paths_version(input)?;
    let (input, paths) = length_count(le_u32, parse_path)(input)?;
    Ok((input, ZonePaths { version, paths }))
}
