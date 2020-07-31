//! # General structs and data

use assembly_core::{
    ldf::{LDFError, LDF},
    types::{ObjectID, ObjectTemplate, Quaternion, Vector3f},
};

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
pub struct ObjectExtra {
    pub field_1a: [u8; 32],
    pub field_1b: [u8; 32],
    pub field_2: u32,
    pub field_3: bool,
    pub field_4: [u32; 16],
    pub field_5: [u8; 3],
}

#[derive(Debug)]
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
