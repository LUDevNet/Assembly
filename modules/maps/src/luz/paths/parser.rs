use std::collections::HashMap;
use std::convert::TryFrom;
use assembly_core::nom::{
    number::complete::{le_f32, le_u32, le_u8},
    IResult,
    named, named_args, map_opt, do_parse, value,
    length_count, call, cond, map_res, pair, fold_many_m_n,
};
use num_traits::FromPrimitive;
use assembly_core::parser::{
    parse_u8_wstring, parse_u32_wstring,
    parse_u8_bool,
    parse_world_id, parse_object_id, parse_object_template,
    parse_vec3f, parse_quat,
};
use super::core::*;

named!(pub parse_zone_paths_version<ZonePathsVersion>,
    map_opt!(le_u32, ZonePathsVersion::from_u32)
);

named!(pub parse_path_version<PathVersion>,
    map_opt!(le_u32, PathVersion::from_u32)
);

named!(pub parse_path_type<PathType>,
    map_opt!(le_u32, PathType::from_u32)
);

named!(pub parse_path_composition<PathComposition>,
    map_opt!(le_u32, PathComposition::from_u32)
);

named!(pub parse_path_data_movement<PathDataMovement>,
    value!(PathDataMovement{})
);

named_args!(pub parse_path_data_moving_platform(version: PathVersion)<PathDataMovingPlatform>,
    do_parse!(
        a: cond!(version.min(18), call!(le_u8)) >>
        b: cond!(version.min(13) && !version.min(18), call!(parse_u8_wstring)) >>
        (PathDataMovingPlatform{
            something: a,
            platform_travel_sound: b,
        })
    )
);

named!(pub parse_property_rental_time_unit<PropertyRentalTimeUnit>,
    map_opt!(le_u32, PropertyRentalTimeUnit::from_u32)
);

named!(pub parse_property_achievement_required<PropertyAchievementRequired>,
    map_opt!(le_u32, PropertyAchievementRequired::from_u32)
);

named!(pub parse_path_data_property<PathDataProperty>,
    do_parse!(
        value_1: le_u32 >>
        price: le_u32 >>
        rental_time: le_u32 >>
        associated_map: parse_world_id >>
        value_2: le_u32 >>
        display_name: parse_u8_wstring >>
        display_description: parse_u32_wstring >>
        value_3: le_u32 >>
        clone_limit: le_u32 >>
        reputation_multiplier: le_f32 >>
        rental_time_unit: parse_property_rental_time_unit >>
        achievement_required: parse_property_achievement_required >>
        player_zone_coordinate: parse_vec3f >>
        max_build_height: le_f32 >>
        (PathDataProperty {
            value_1,
            price, rental_time, associated_map,
            value_2,
            display_name, display_description,
            value_3,
            clone_limit, reputation_multiplier,
            rental_time_unit, achievement_required,
            player_zone_coordinate, max_build_height,
        })
    )
);

named_args!(pub parse_path_data_camera(version: PathVersion)<PathDataCamera>,
    do_parse!(
        next_path: parse_u8_wstring >>
        value_1: cond!(version.min(14), le_u8) >>
        (PathDataCamera{next_path, value_1})
    )
);

named!(pub parse_path_data_spawner<PathDataSpawner>,
    do_parse!(
        spawned_lot: parse_object_template >>
        respawn_time: le_u32 >>
        max_to_spawn: le_u32 >>
        min_to_spawn: le_u32 >>
        spawner_obj_id: parse_object_id >>
        activate_network_on_load: parse_u8_bool >>
        (PathDataSpawner{
            spawned_lot,
            respawn_time,
            max_to_spawn,
            min_to_spawn,
            spawner_obj_id,
            activate_network_on_load,
        })
    )
);

named!(pub parse_path_data_showcase<PathDataShowcase>,
    value!(PathDataShowcase{})
);

named!(pub parse_path_data_race<PathDataRace>,
    value!(PathDataRace{})
);

named!(pub parse_path_data_rail<PathDataRail>,
    value!(PathDataRail{})
);

named!(pub parse_path_waypoint_data_movement<PathWaypointDataMovement>,
    do_parse!(
        config: parse_waypoint_config >>
        (PathWaypointDataMovement{config})
    )
);

named!(pub parse_path_waypoint_data_moving_platform_sounds<PathWaypointDataMovingPlatformSounds>,
    do_parse!(
        depart_sound: parse_u8_wstring >>
        arrive_sound: parse_u8_wstring >>
        (PathWaypointDataMovingPlatformSounds{ depart_sound, arrive_sound })
    )
);

named_args!(pub parse_path_waypoint_data_moving_platform(version: PathVersion)<PathWaypointDataMovingPlatform>,
    do_parse!(
        rotation: parse_quat >>
        lock_player: parse_u8_bool >>
        speed: le_f32 >>
        wait: le_f32 >>
        sounds: cond!(version.min(13), call!(parse_path_waypoint_data_moving_platform_sounds)) >>
        (PathWaypointDataMovingPlatform {
            rotation, lock_player, speed, wait, sounds
        })
    )
);

named!(pub parse_path_waypoint_data_property<PathWaypointDataProperty>,
    value!(PathWaypointDataProperty{})
);

named!(pub parse_path_waypoint_data_camera<PathWaypointDataCamera>,
    do_parse!(
        a: le_f32 >>
        b: le_f32 >>
        c: le_f32 >>
        d: le_f32 >>
        e: le_f32 >>
        f: le_f32 >>
        g: le_f32 >>
        h: le_f32 >>
        i: le_f32 >>
        (PathWaypointDataCamera{
            value_1: a, value_2: b, value_3: c, value_4: d,
            time: e,
            value_5: f,
            tension: g, continuity: h, bias: i,
        })
    )
);

named!(pub parse_path_waypoint_data_spawner<PathWaypointDataSpawner>,
    do_parse!(
        rotation: parse_quat >>
        config: parse_waypoint_config >>
        (PathWaypointDataSpawner{ rotation, config })
    )
);

named!(pub parse_path_waypoint_data_showcase<PathWaypointDataShowcase>,
    value!(PathWaypointDataShowcase{})
);

named!(pub parse_path_waypoint_data_race<PathWaypointDataRace>,
    do_parse!(
        rotation: parse_quat >>
        value_1: le_u8 >>
        value_2: le_u8 >>
        value_3: le_f32 >>
        value_4: le_f32 >>
        value_5: le_f32 >>
        (PathWaypointDataRace{ rotation, value_1, value_2, value_3, value_4, value_5 })
    )
);

named!(pub parse_path_waypoint_data_rail<PathWaypointDataRail>,
    do_parse!(
        value_1: le_f32 >>
        value_2: le_f32 >>
        value_3: le_f32 >>
        value_4: le_f32 >>
        config: parse_waypoint_config >>
        (PathWaypointDataRail{ value_1, value_2, value_3, value_4, config })
    )
);

named!(parse_path_waypoint_variant_movement<PathWaypointVariantMovement>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_movement >>
        (PathWaypointVariantMovement{position, data})
    )
);

named_args!(parse_path_variant_movement(header: PathHeader)<PathVariantMovement>,
    do_parse!(
        path_data: parse_path_data_movement >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_movement) >>
        (PathVariantMovement {
            header, path_data, waypoints,
        })
    )
);

named_args!(parse_path_waypoint_variant_moving_platform(version: PathVersion)<PathWaypointVariantMovingPlatform>,
    do_parse!(
        position: parse_vec3f >>
        data: call!(parse_path_waypoint_data_moving_platform, version) >>
        (PathWaypointVariantMovingPlatform{position, data})
    )
);

named_args!(parse_path_variant_moving_platform(header: PathHeader)<PathVariantMovingPlatform>,
    do_parse!(
        path_data: call!(parse_path_data_moving_platform, header.version) >>
        waypoints: length_count!(le_u32, call!(parse_path_waypoint_variant_moving_platform, header.version)) >>
        (PathVariantMovingPlatform {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_property<PathWaypointVariantProperty>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_property >>
        (PathWaypointVariantProperty{position, data})
    )
);

named_args!(parse_path_variant_property(header: PathHeader)<PathVariantProperty>,
    do_parse!(
        path_data: parse_path_data_property >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_property) >>
        (PathVariantProperty {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_camera<PathWaypointVariantCamera>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_camera >>
        (PathWaypointVariantCamera{position, data})
    )
);

named_args!(parse_path_variant_camera(header: PathHeader)<PathVariantCamera>,
    do_parse!(
        path_data: call!(parse_path_data_camera, header.version) >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_camera) >>
        (PathVariantCamera {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_spawner<PathWaypointVariantSpawner>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_spawner >>
        (PathWaypointVariantSpawner{position, data})
    )
);

named_args!(parse_path_variant_spawner(header: PathHeader)<PathVariantSpawner>,
    do_parse!(
        path_data: parse_path_data_spawner >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_spawner) >>
        (PathVariantSpawner {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_showcase<PathWaypointVariantShowcase>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_showcase >>
        (PathWaypointVariantShowcase{position, data})
    )
);

named_args!(parse_path_variant_showcase(header: PathHeader)<PathVariantShowcase>,
    do_parse!(
        path_data: parse_path_data_showcase >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_showcase) >>
        (PathVariantShowcase {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_race<PathWaypointVariantRace>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_race >>
        (PathWaypointVariantRace{position, data})
    )
);

named_args!(parse_path_variant_race(header: PathHeader)<PathVariantRace>,
    do_parse!(
        path_data: parse_path_data_race >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_race) >>
        (PathVariantRace {
            header, path_data, waypoints,
        })
    )
);

named!(parse_path_waypoint_variant_rail<PathWaypointVariantRail>,
    do_parse!(
        position: parse_vec3f >>
        data: parse_path_waypoint_data_rail >>
        (PathWaypointVariantRail{position, data})
    )
);

named_args!(parse_path_variant_rail(header: PathHeader)<PathVariantRail>,
    do_parse!(
        path_data: parse_path_data_rail >>
        waypoints: length_count!(le_u32, parse_path_waypoint_variant_rail) >>
        (PathVariantRail {
            header, path_data, waypoints,
        })
    )
);

fn parse_path_data(inp: &[u8], path_type: PathType, header: PathHeader) -> IResult<&[u8], Path> {
    match path_type {
        PathType::Movement => {
            let (inp, var) = parse_path_variant_movement(inp, header)?;
            Ok((inp, Path::Movement(var)))
        },
        PathType::MovingPlatform => {
            let (inp, var) = parse_path_variant_moving_platform(inp, header)?;
            Ok((inp, Path::MovingPlatform(var)))
        },
        PathType::Property => {
            let (inp, var) = parse_path_variant_property(inp, header)?;
            Ok((inp, Path::Property(var)))
        },
        PathType::Camera => {
            let (inp, var) = parse_path_variant_camera(inp, header)?;
            Ok((inp, Path::Camera(var)))
        },
        PathType::Spawner => {
            let (inp, var) = parse_path_variant_spawner(inp, header)?;
            Ok((inp, Path::Spawner(var)))
        },
        PathType::Showcase => {
            let (inp, var) = parse_path_variant_showcase(inp, header)?;
            Ok((inp, Path::Showcase(var)))
        },
        PathType::Race => {
            let (inp, var) = parse_path_variant_race(inp, header)?;
            Ok((inp, Path::Race(var)))
        },
        PathType::Rail => {
            let (inp, var) = parse_path_variant_rail(inp, header)?;
            Ok((inp, Path::Rail(var)))
        },
    }
}

named!(pub parse_path<Path>,
    do_parse!(
        version: parse_path_version >>
        path_name: parse_u8_wstring >>
        path_type: parse_path_type >>
        value_1: le_u32 >>
        path_composition: parse_path_composition >>
        header: value!(PathHeader{version, path_name, value_1, path_composition}) >>
        path: call!(parse_path_data, path_type, header) >>
        (path)
    )
);

named!(pub parse_waypoint_config_entry<(String, String)>,
    pair!(parse_u8_wstring, parse_u8_wstring)
);

fn extend_config_map(mut map: HashMap<String,String>, entry: (String,String)) -> HashMap<String,String> {
    map.insert(entry.0, entry.1);
    map
}

named!(pub parse_waypoint_config<WaypointConfig>,
    do_parse!(
        count: map_res!(le_u32, usize::try_from) >>
        map: fold_many_m_n!(count, count, parse_waypoint_config_entry, HashMap::new(), extend_config_map) >>
        (map)
    )
);

named!(pub parse_zone_paths<ZonePaths>,
    do_parse!(
        version: parse_zone_paths_version >>
        paths: length_count!(le_u32, parse_path) >>
        (ZonePaths{version,paths})
    )
);
