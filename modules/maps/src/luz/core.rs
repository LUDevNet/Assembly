use super::paths::core::ZonePaths;
use crate::luz::paths::parser::parse_zone_paths;
use assembly_core::{
    nom::{error::ErrorKind, Finish, Offset},
    types::{Placement3D, Vector3f, WorldID},
};

#[cfg(feature = "serde-derives")]
use serde::Serialize;

/// Version of the zone file
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct FileVersion(u32);

impl FileVersion {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn min(&self, val: u32) -> bool {
        self.0 >= val
    }
}

impl From<u32> for FileVersion {
    fn from(val: u32) -> Self {
        FileVersion(val)
    }
}

/// Reference to a scene file
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct SceneRef {
    /// Name of the scene file
    pub file_name: String,
    /// ID of the scene
    pub id: u32,
    /// 0: default, 1: audio
    pub layer: u32,
    /// Name of the scene
    pub name: String,
}

/// Scene Transition at a single point
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct SceneTransitionPoint {
    /// ID of the scene
    pub scene_id: u64,
    /// Position of the transition
    pub point: Vector3f,
}

/// Transition Points
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub enum SceneTransitionInfo {
    Point2([SceneTransitionPoint; 2]),
    Point5([SceneTransitionPoint; 5]),
}

impl From<[SceneTransitionPoint; 2]> for SceneTransitionInfo {
    fn from(val: [SceneTransitionPoint; 2]) -> Self {
        SceneTransitionInfo::Point2(val)
    }
}

impl From<[SceneTransitionPoint; 5]> for SceneTransitionInfo {
    fn from(val: [SceneTransitionPoint; 5]) -> Self {
        SceneTransitionInfo::Point5(val)
    }
}

/// Transitions between scenes
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct SceneTransition {
    /// Name of the transition
    pub name: Option<String>,
    /// Points of the transition
    pub points: SceneTransitionInfo,
}

/// A type that can represent path data
pub trait PathData {}

impl PathData for Vec<u8> {}
impl PathData for ZonePaths {}

/// The data in a luz file
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct ZoneFile<P: PathData> {
    /// Version of this file
    pub file_version: FileVersion,
    /// Revision of this file
    pub file_revision: Option<u32>,
    /// ID of the world described
    pub world_id: WorldID,
    /// Spawining placement of the player
    pub spawn_point: Option<Placement3D>,
    /// List of scenes
    pub scene_refs: Vec<SceneRef>,

    /// Unknown
    pub something: String,
    /// Relative filename of the map
    pub map_filename: String,
    /// Internal name of the map
    pub map_name: String,
    /// Internal description of the map
    pub map_description: String,

    /// List of transitions
    pub scene_transitions: Option<Vec<SceneTransition>>,
    /// Path data
    pub path_data: Option<P>,
}

impl<P: PathData> ZoneFile<P> {
    fn set_path_data<N: PathData>(self, new: Option<N>) -> ZoneFile<N> {
        ZoneFile {
            file_version: self.file_version,
            file_revision: self.file_revision,
            world_id: self.world_id,
            spawn_point: self.spawn_point,
            scene_refs: self.scene_refs,
            something: self.something,
            map_filename: self.map_filename,
            map_name: self.map_name,
            map_description: self.map_description,
            scene_transitions: self.scene_transitions,
            path_data: new,
        }
    }
}

pub type ParsePathErr = (ZoneFile<Vec<u8>>, (usize, ErrorKind));

impl ZoneFile<Vec<u8>> {
    pub fn parse_paths(self) -> Result<ZoneFile<ZonePaths>, ParsePathErr> {
        if let Some(path_data) = &self.path_data {
            match parse_zone_paths(path_data).finish() {
                Ok((_rest, path_data)) => Ok(self.set_path_data(Some(path_data))),
                Err(e) => {
                    let len = path_data.offset(e.input);
                    let code = e.code;
                    Err((self, (len, code)))
                }
            }
        } else {
            Ok(self.set_path_data(None))
        }
    }
}
