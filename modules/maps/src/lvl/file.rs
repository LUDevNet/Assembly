//! # General structs and data

/// The version of a chunk
#[derive(Debug)]
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
