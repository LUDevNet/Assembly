use crate::core::types::{
    Placement3D,
    WorldID,
    Vector3f,
};

/// Version of the zone file
#[derive(Debug, Clone, Copy)]
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
pub struct SceneRef {
    /// Name of the scene file
    pub file_name: String,
    /// ID of the scene
    pub id: u32,
    /// Whether this is an audio scene
    pub is_audio: bool,
    /// Name of the scene
    pub name: String,
}

/// Scene Transition at a single point
#[derive(Copy, Clone, Debug)]
pub struct SceneTransitionPoint {
    /// ID of the scene
    pub scene_id: u64,
    /// Position of the transition
    pub point: Vector3f,
}

/// Transition Points
#[derive(Debug)]
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
pub struct SceneTransition {
    /// Name of the transition
    pub name: Option<String>,
    /// Points of the transition
    pub points: SceneTransitionInfo,
}

/// The data in a luz file
#[derive(Debug)]
pub struct ZoneFile {
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
    pub path_data: Option<Vec<u8>>,
}
