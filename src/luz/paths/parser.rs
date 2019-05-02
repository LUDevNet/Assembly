use std::collections::HashMap;
use std::convert::TryFrom;
use nom::{le_f32, le_u32, le_u8};
use num_traits::FromPrimitive;
use crate::core::parser::{
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

named!(pub parse_path_waypoint_movement<PathWaypointMovement>,
    do_parse!(
        config: parse_waypoint_config >>
        (PathWaypointMovement{config})
    )
);

named!(pub parse_path_waypoint_moving_platform_sounds<PathWaypointMovingPlatformSounds>,
    do_parse!(
        depart_sound: parse_u8_wstring >>
        arrive_sound: parse_u8_wstring >>
        (PathWaypointMovingPlatformSounds{ depart_sound, arrive_sound })
    )
);

named_args!(pub parse_path_waypoint_moving_platform(version: PathVersion)<PathWaypointMovingPlatform>,
    do_parse!(
        rotation: parse_quat >>
        lock_player: parse_u8_bool >>
        speed: le_f32 >>
        wait: le_f32 >>
        sounds: cond!(version.min(13), call!(parse_path_waypoint_moving_platform_sounds)) >>
        (PathWaypointMovingPlatform {
            rotation, lock_player, speed, wait, sounds
        })
    )
);

named!(pub parse_path_waypoint_property<PathWaypointProperty>,
    value!(PathWaypointProperty{})
);

named!(pub parse_path_waypoint_camera<PathWaypointCamera>,
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
        (PathWaypointCamera{
            value_1: a, value_2: b, value_3: c, value_4: d,
            time: e,
            value_5: f,
            tension: g, continuity: h, bias: i,
        })
    )
);

named!(pub parse_path_waypoint_spawner<PathWaypointSpawner>,
    do_parse!(
        rotation: parse_quat >>
        config: parse_waypoint_config >>
        (PathWaypointSpawner{ rotation, config })
    )
);

named!(pub parse_path_waypoint_showcase<PathWaypointShowcase>,
    value!(PathWaypointShowcase{})
);

named!(pub parse_path_waypoint_race<PathWaypointRace>,
    do_parse!(
        rotation: parse_quat >>
        value_1: le_u8 >>
        value_2: le_u8 >>
        value_3: le_f32 >>
        value_4: le_f32 >>
        value_5: le_f32 >>
        (PathWaypointRace{ rotation, value_1, value_2, value_3, value_4, value_5 })
    )
);

named!(pub parse_path_waypoint_rail<PathWaypointRail>,
    do_parse!(
        value_1: le_f32 >>
        value_2: le_f32 >>
        value_3: le_f32 >>
        value_4: le_f32 >>
        config: parse_waypoint_config >>
        (PathWaypointRail{ value_1, value_2, value_3, value_4, config })
    )
);

named!(pub parse_path<Path>,
    do_parse!(
        version: parse_path_version >>
        path_name: parse_u8_wstring >>
        path_type: parse_path_type >>
        value_1: le_u32 >>
        path_composition: parse_path_composition >>
        path: switch!(value!(path_type),
            PathType::Movement => do_parse!(
                path_data: parse_path_data_movement >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_movement >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Movement(PathVariantMovement {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::MovingPlatform => do_parse!(
                path_data: call!(parse_path_data_moving_platform, version) >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: call!(parse_path_waypoint_moving_platform, version) >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::MovingPlatform(PathVariantMovingPlatform {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Property => do_parse!(
                path_data: parse_path_data_property >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_property >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Property(PathVariantProperty {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Camera => do_parse!(
                path_data: call!(parse_path_data_camera, version) >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_camera >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Camera(PathVariantCamera {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Spawner => do_parse!(
                path_data: parse_path_data_spawner >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_spawner >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Spawner(PathVariantSpawner {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Showcase => do_parse!(
                path_data: parse_path_data_showcase >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_showcase >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Showcase(PathVariantShowcase {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Race => do_parse!(
                path_data: parse_path_data_race >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_race >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Race(PathVariantRace {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            ) |
            PathType::Rail => do_parse!(
                path_data: parse_path_data_rail >>
                waypoints: length_count!(le_u32, do_parse!(
                    position: parse_vec3f >>
                    data: parse_path_waypoint_rail >>
                    (PathWaypoint{position, data})
                )) >>
                (Path::Rail(PathVariantRail {
                    version, path_name, value_1, path_composition,
                    path_data, waypoints,
                }))
            )
        ) >>
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
