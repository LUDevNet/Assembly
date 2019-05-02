use crate::core::types::{WorldID, Vector3f, Quaternion, ObjectTemplate, ObjectID};
use std::collections::HashMap;

/// Version / first field of path data
#[derive(Clone, Debug, FromPrimitive, ToPrimitive)]
pub struct ZonePathsVersion(u32);

/// Version of this path data
#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub struct PathVersion(u32);

impl PathVersion {
    pub fn min(self, val: u32) -> bool {
        return self.0 >= val;
    }
}

/// Type of this path
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PathType {
    Movement,
    MovingPlatform,
    Property,
    Camera,
    Spawner,
    Showcase,
    Race,
    Rail,
}

/// Interpretation of this path
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PathComposition {
    /// A closed polygon
    Polygon,
    /// A collection of single points
    Points,
    /// A sequence of points
    Line,
}

/// General data for a movement path
#[derive(Debug)]
pub struct PathDataMovement {}

/// General data for a moving platform path
#[derive(Debug)]
pub struct PathDataMovingPlatform {
    /// Unknown field
    pub something: Option<u8>,
    /// Travel sound?
    pub platform_travel_sound: Option<String>,
}

/// Time units for rental time
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PropertyRentalTimeUnit {
    Forever,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

/// Achievement required to rent a property
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PropertyAchievementRequired {
    None,
    Builder,
    Craftsman,
    SeniorBuilder,
    Journeyman,
    MasterBuilder,
    Architect,
    SeniorArchitect,
    MasterArchitect,
    Visionary,
    Exemplar,
}

/// General data for a property (border) path
#[derive(Debug)]
pub struct PathDataProperty {
    /// Unknown value
    pub value_1: u32,
    /// Rental price
    pub price: u32,
    /// Rental time
    pub rental_time: u32,
    /// World that this property is attached to
    pub associated_map: WorldID,
    /// Unknown value
    pub value_2: u32,
    /// Display name of the property
    pub display_name: String,
    /// Display description
    pub display_description: String,
    /// Unknown value
    pub value_3: u32,
    /// Limit to the number of clones in one instance
    pub clone_limit: u32,
    /// Multiplier for reputation
    pub reputation_multiplier: f32,
    /// Unit for rental time
    pub rental_time_unit: PropertyRentalTimeUnit,
    /// Required achievement
    pub achievement_required: PropertyAchievementRequired,
    /// Coordinate of the player
    pub player_zone_coordinate: Vector3f,
    /// Maximum building height
    pub max_build_height: f32,
}

/// General data for camera path
#[derive(Debug)]
pub struct PathDataCamera {
    /// Following path
    pub next_path: String,
    /// Unknown
    pub value_1: Option<u8>,
}

/// General data for a spawner path
#[derive(Debug)]
pub struct PathDataSpawner {
    /// The object to be spawned
    pub spawned_lot: ObjectTemplate,
    /// Time until respawn
    pub respawn_time: u32,
    /// max to spawn (MAX_VALUE for infinity)
    pub max_to_spawn: u32,
    /// number to maintain spawned
    pub min_to_spawn: u32,
    /// Spawner object ID without flags
    pub spawner_obj_id: ObjectID,
    /// Activate network on load
    pub activate_network_on_load: bool,
}

/// General data for a showcase path
#[derive(Debug)]
pub struct PathDataShowcase {}

/// General data for a race path
#[derive(Debug)]
pub struct PathDataRace {}

/// General data for a rail path
#[derive(Debug)]
pub struct PathDataRail {}

/// Data for a movement path waypoint
#[derive(Debug)]
pub struct PathWaypointMovement {
    pub config: WaypointConfig,
}

/// Sounds for a moving platform
#[derive(Debug)]
pub struct PathWaypointMovingPlatformSounds {
    pub arrive_sound: String,
    pub depart_sound: String,
}

/// Data for a moving platform path waypoint
#[derive(Debug)]
pub struct PathWaypointMovingPlatform {
    pub rotation: Quaternion,
    pub lock_player: bool,
    pub speed: f32,
    pub wait: f32,
    pub sounds: Option<PathWaypointMovingPlatformSounds>,
}

/// Data for a property (border) path waypoint
#[derive(Debug)]
pub struct PathWaypointProperty {}

/// Data for a camera path waypoint
#[derive(Debug)]
pub struct PathWaypointCamera {
    pub value_1: f32,
    pub value_2: f32,
    pub value_3: f32,
    pub value_4: f32,
    pub time: f32,
    pub value_5: f32,
    pub tension: f32,
    pub continuity: f32,
    pub bias: f32,
}

/// Data for a spawner network waypoint
#[derive(Debug)]
pub struct PathWaypointSpawner {
    pub rotation: Quaternion,
    pub config: WaypointConfig,
}

/// Data for a showcase path waypoint
#[derive(Debug)]
pub struct PathWaypointShowcase {}

/// Data for a race path waypoint
#[derive(Debug)]
pub struct PathWaypointRace {
    pub rotation: Quaternion,
    pub value_1: u8,
    pub value_2: u8,
    pub value_3: f32,
    pub value_4: f32,
    pub value_5: f32,
}

/// Data for a rail path waypoint
#[derive(Debug)]
pub struct PathWaypointRail {
    pub value_1: f32,
    pub value_2: f32,
    pub value_3: f32,
    pub value_4: f32,
    pub config: WaypointConfig,
}

/// Config for a waypoint
pub type WaypointConfig = HashMap<String, String>;

/// Path Waypoint
#[derive(Debug)]
pub struct PathWaypoint<WaypointType> {
    pub position: Vector3f,
    pub data: WaypointType,
}

/// Wrapper for all general path data
#[derive(Debug)]
pub struct PathVariant<DataType, WaypointType> {
    pub version: PathVersion,
    pub path_name: String,
    pub value_1: u32,
    pub path_composition: PathComposition,
    pub path_data: DataType,
    pub waypoints: Vec<PathWaypoint<WaypointType>>
}

pub type PathVariantMovement = PathVariant<PathDataMovement,PathWaypointMovement>;
pub type PathVariantMovingPlatform = PathVariant<PathDataMovingPlatform, PathWaypointMovingPlatform>;
pub type PathVariantProperty = PathVariant<PathDataProperty, PathWaypointProperty>;
pub type PathVariantCamera = PathVariant<PathDataCamera, PathWaypointCamera>;
pub type PathVariantSpawner = PathVariant<PathDataSpawner, PathWaypointSpawner>;
pub type PathVariantShowcase = PathVariant<PathDataShowcase, PathWaypointShowcase>;
pub type PathVariantRace = PathVariant<PathDataRace, PathWaypointRace>;
pub type PathVariantRail = PathVariant<PathDataRail, PathWaypointRail>;

/// Enum of all path variants
#[derive(Debug)]
pub enum Path {
    Movement(PathVariantMovement),
    MovingPlatform(PathVariantMovingPlatform),
    Property(PathVariantProperty),
    Camera(PathVariantCamera),
    Spawner(PathVariantSpawner),
    Showcase(PathVariantShowcase),
    Race(PathVariantRace),
    Rail(PathVariantRail),
}

/// All paths in a zone
#[derive(Debug)]
pub struct ZonePaths {
    pub version: ZonePathsVersion,
    pub paths: Vec<Path>,
}
