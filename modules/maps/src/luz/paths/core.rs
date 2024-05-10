use assembly_core::types::{ObjectID, ObjectTemplate, Quaternion, Vector3f, WorldID};
use std::{collections::HashMap, fmt};

#[cfg(feature = "serde-derives")]
use serde::Serialize;

enum ErrorKind {
    /// Invalid argument parsed (e.g. undefined integer in enum)
    ArgumentError,
}

pub struct Error {
    kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::ArgumentError => write!(f, "Argument Error"),
        }
    }
}

impl Error {
    fn argument_error() -> Self {
        Self {
            kind: ErrorKind::ArgumentError,
        }
    }
}

/// Version / first field of path data
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct ZonePathsVersion(u32);

impl ZonePathsVersion {
    pub fn id(&self) -> u32 {
        self.0
    }
}

impl From<u32> for ZonePathsVersion {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

/// Version of this path data
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct PathVersion(u32);

impl From<u32> for PathVersion {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl PathVersion {
    pub fn min(self, val: u32) -> bool {
        self.0 >= val
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

/// Type of this path
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
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

impl PathType {
    const VALUES: [PathType; 8] = [
        Self::Movement,
        Self::MovingPlatform,
        Self::Property,
        Self::Camera,
        Self::Spawner,
        Self::Showcase,
        Self::Race,
        Self::Rail,
    ];

    pub fn from_u32(v: u32) -> Option<Self> {
        Self::VALUES.get(v as usize).copied()
    }
}

/// Interpretation of this path
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[derive(Debug)]
pub enum PathComposition {
    /// A closed polygon
    Polygon,
    /// A collection of single points
    Points,
    /// A sequence of points
    Line,
}

impl PathComposition {
    /// Turn a u32 into a value
    pub fn from_u32(v: u32) -> Result<Self, Error> {
        match v {
            0 => Ok(Self::Polygon),
            1 => Ok(Self::Points),
            2 => Ok(Self::Line),
            _ => Err(Error {
                kind: ErrorKind::ArgumentError,
            }),
        }
    }
}

/// General data for a movement path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathDataMovement {}

/// General data for a moving platform path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub enum PathDataMovingPlatform {
    PreV13,
    V13ToV17 {
        /// Travel sound?
        platform_travel_sound: String,
    },
    PostV18 {
        /// Unknown field
        something: u8,
    },
}

/// Time units for rental time
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
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

impl PropertyRentalTimeUnit {
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Forever),
            1 => Some(Self::Seconds),
            2 => Some(Self::Minutes),
            3 => Some(Self::Hours),
            4 => Some(Self::Days),
            5 => Some(Self::Weeks),
            6 => Some(Self::Months),
            7 => Some(Self::Years),
            _ => None,
        }
    }
}

/// Achievement required to rent a property
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
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

impl PropertyAchievementRequired {
    pub fn from_u32(v: u32) -> Result<Option<PropertyAchievementRequired>, Error> {
        match v {
            0 => Ok(None),
            1 => Ok(Some(Self::Builder)),
            2 => Ok(Some(Self::Craftsman)),
            3 => Ok(Some(Self::SeniorBuilder)),
            4 => Ok(Some(Self::Journeyman)),
            5 => Ok(Some(Self::MasterBuilder)),
            6 => Ok(Some(Self::Architect)),
            7 => Ok(Some(Self::SeniorArchitect)),
            8 => Ok(Some(Self::MasterArchitect)),
            9 => Ok(Some(Self::Visionary)),
            10 => Ok(Some(Self::Exemplar)),
            _ => Err(Error::argument_error()),
        }
    }
}

/// General data for a property (border) path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
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
    pub achievement_required: Option<PropertyAchievementRequired>,
    /// Coordinate of the player
    pub player_zone_coordinate: Vector3f,
    /// Maximum building height
    pub max_build_height: f32,
}

/// General data for camera path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathDataCamera {
    /// Following path
    pub next_path: String,
    /// Unknown
    pub value_1: Option<u8>,
}

/// General data for a spawner path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
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
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathDataShowcase {}

/// General data for a race path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathDataRace {}

/// General data for a rail path
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathDataRail {}

/// Data for a movement path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataMovement {
    pub config: WaypointConfig,
}

/// Sounds for a moving platform
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataMovingPlatformSounds {
    pub arrive_sound: String,
    pub depart_sound: String,
}

/// Data for a moving platform path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataMovingPlatform {
    pub rotation: Quaternion,
    pub lock_player: bool,
    pub speed: f32,
    pub wait: f32,
    pub sounds: Option<PathWaypointDataMovingPlatformSounds>,
}

/// Data for a property (border) path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataProperty {}

/// Data for a camera path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataCamera {
    pub rotation: Quaternion,
    pub time: f32,
    pub value_5: f32,
    pub tension: f32,
    pub continuity: f32,
    pub bias: f32,
}

/// Data for a spawner network waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataSpawner {
    pub rotation: Quaternion,
    pub config: WaypointConfig,
}

/// Data for a showcase path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataShowcase {}

/// Data for a race path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataRace {
    pub rotation: Quaternion,
    pub value_1: u8,
    pub value_2: u8,
    pub value_3: f32,
    pub value_4: f32,
    pub value_5: f32,
}

/// Data for a rail path waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointDataRail {
    pub rotation: Quaternion,
    pub speed: Option<f32>,
    pub config: WaypointConfig,
}

/// Config for a waypoint
pub type WaypointConfig = HashMap<String, String>;

/// Path Waypoint
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathWaypointVariant<WaypointType> {
    pub position: Vector3f,
    #[cfg_attr(feature = "serde-derives", serde(flatten))]
    pub data: WaypointType,
}

pub type PathWaypointVariantMovement = PathWaypointVariant<PathWaypointDataMovement>;
pub type PathWaypointVariantMovingPlatform = PathWaypointVariant<PathWaypointDataMovingPlatform>;
pub type PathWaypointVariantProperty = PathWaypointVariant<PathWaypointDataProperty>;
pub type PathWaypointVariantCamera = PathWaypointVariant<PathWaypointDataCamera>;
pub type PathWaypointVariantSpawner = PathWaypointVariant<PathWaypointDataSpawner>;
pub type PathWaypointVariantShowcase = PathWaypointVariant<PathWaypointDataShowcase>;
pub type PathWaypointVariantRace = PathWaypointVariant<PathWaypointDataRace>;
pub type PathWaypointVariantRail = PathWaypointVariant<PathWaypointDataRail>;

/// Common header for all paths
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathHeader {
    pub version: PathVersion,
    pub path_name: String,
    pub value_1: u32,
    pub path_composition: PathComposition,
}

/// Wrapper for all general path data
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct PathVariant<DataType, WaypointDataType> {
    pub header: PathHeader,
    pub path_data: DataType,
    pub waypoints: Vec<PathWaypointVariant<WaypointDataType>>,
}

pub type PathVariantMovement = PathVariant<PathDataMovement, PathWaypointDataMovement>;
pub type PathVariantMovingPlatform =
    PathVariant<PathDataMovingPlatform, PathWaypointDataMovingPlatform>;
pub type PathVariantProperty = PathVariant<PathDataProperty, PathWaypointDataProperty>;
pub type PathVariantCamera = PathVariant<PathDataCamera, PathWaypointDataCamera>;
pub type PathVariantSpawner = PathVariant<PathDataSpawner, PathWaypointDataSpawner>;
pub type PathVariantShowcase = PathVariant<PathDataShowcase, PathWaypointDataShowcase>;
pub type PathVariantRace = PathVariant<PathDataRace, PathWaypointDataRace>;
pub type PathVariantRail = PathVariant<PathDataRail, PathWaypointDataRail>;

/// Enum of all path variants
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[cfg_attr(feature = "serde-derives", serde(tag = "type"))]
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
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct ZonePaths {
    pub version: ZonePathsVersion,
    pub paths: Vec<Path>,
}
