//! # General structs and data
use assembly_core::{
    ldf::{LDFError, LDF},
    types::{ObjectID, ObjectTemplate, Quaternion, Vector3f},
};

#[cfg(feature = "serde-derives")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Level {
    pub env: Option<Environment>,
    pub objects: Vec<Object<LDF>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Environment {
    pub sec1: Section1,
    pub sky: SkySection,
}

/// The version of a chunk
#[derive(Debug, Copy, Clone)]
pub struct ChunkVersion {
    /// The version of the chunk header format
    pub header: u16,
    /// The version of the chunk data format
    pub data: u16,
}

/// The header for a single chunk
#[derive(Debug)]
pub struct ChunkHeader {
    /// The ID of this chunk
    pub id: u32,
    /// The version of this chunk
    pub version: ChunkVersion,
    /// The chunk size
    pub size: u32,
    /// The chunk data offset
    pub offset: u32,
}

/// A chunk (header + data)
#[derive(Debug)]
pub struct Chunk<T> {
    /// The chunk header
    pub header: ChunkHeader,
    /// The chunk data
    pub data: T,
}

/// The chunk containing the offsets of the other chunks
#[derive(Debug)]
pub struct FileMetaChunkData {
    /// The version of this file
    pub version: u32,
    /// The revision of this file
    pub revision: u32,
    /// The pointer to the chunk #2000
    pub chunk_2000_offset: u32,
    /// The pointer to the chunk #2001
    pub chunk_2001_offset: u32,
    /// The pointer to the chunk #2002
    pub chunk_2002_offset: u32,
}

/// The file meta chunk
pub type FileMetaChunk = Chunk<FileMetaChunkData>;

#[derive(Debug)]
pub struct Chunk2000Data {}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct ObjectExtra {
    pub field_1a: [u8; 32],
    pub field_1b: [u8; 32],
    pub field_2: u32,
    pub field_3: bool,
    pub field_4: [u32; 16],
    pub field_5: [u8; 3],
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Object<S> {
    pub obj_id: ObjectID,
    pub lot: ObjectTemplate,
    pub asset_type: Option<u32>,
    pub value_1: Option<u32>,
    pub position: Vector3f,
    pub rotation: Quaternion,
    pub scale: f32,
    pub settings: S,
    pub extra: Vec<ObjectExtra>,
}

impl Object<String> {
    pub fn parse_settings(self) -> Result<Object<LDF>, LDFError> {
        let settings = self.settings.parse()?;
        Ok(Object {
            obj_id: self.obj_id,
            lot: self.lot,
            asset_type: self.asset_type,
            value_1: self.value_1,
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            settings,
            extra: self.extra,
        })
    }
}

#[derive(Debug)]
pub struct ObjectsChunkData<S> {
    pub objects: Vec<Object<S>>,
}

impl ObjectsChunkData<String> {
    pub fn parse_settings(mut self) -> Result<ObjectsChunkData<LDF>, LDFError> {
        let objects = self
            .objects
            .drain(..)
            .map(Object::parse_settings)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ObjectsChunkData { objects })
    }
}

#[derive(Debug)]
pub struct EnvironmentChunkData {
    pub section1_address: u32,
    pub sky_address: u32,
    pub section3_address: u32,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl From<(f32, f32, f32)> for Color {
    fn from((red, green, blue): (f32, f32, f32)) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Section1 {
    pub value1: Option<f32>,
    pub value2: Color,
    pub value3: Color,
    pub value4: Color,
    pub value5: Vector3f,
    pub value6: Option<Section1_31>,
    pub value7: Option<Color>,
    pub value8: Option<Section1_43>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Section1_31 {
    pub value1: Section1_39,
    pub value2: Color,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub enum Section1_39 {
    Before {
        value1: f32,
        value2: f32,
    },
    After {
        values: Box<[f32; 12]>,
        array: Vec<Section1_40>,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Section1_40 {
    pub id: u32,
    pub float1: f32,
    pub float2: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Section1_43 {
    pub pos: Vector3f,
    pub rot: Option<Quaternion>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
#[cfg_attr(feature = "serde-derives", serde(transparent))]
pub struct SkySection {
    pub files: [String; 6],
}
