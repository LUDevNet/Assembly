//! # Low level reading
use super::file::*;
use super::parser;

use assembly_core::reader::{FileResult, ParseError};

use std::io::prelude::*;
use std::io::SeekFrom;

/// A low level reader class
pub struct LevelReader<T> {
    inner: T,
}

impl<T> LevelReader<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> LevelReader<T>
where
    T: Read + Seek,
{
    /// Load a chunk header
    pub fn get_chunk_header(&mut self) -> FileResult<ChunkHeader> {
        let mut header_bytes = [0 as u8; 20];
        self.inner.read_exact(&mut header_bytes)?;
        let (_rest, header) =
            parser::parse_chunk_header(&header_bytes).map_err(ParseError::from)?;
        Ok(header)
    }

    /// Get the chunk meta data
    pub fn get_meta_chunk_data(&mut self) -> FileResult<FileMetaChunkData> {
        let mut meta_chunk_data_bytes = [0 as u8; 20];
        self.inner.read_exact(&mut meta_chunk_data_bytes)?;
        let (_rest, meta_chunk_data) =
            parser::parse_file_meta_chunk_data(&meta_chunk_data_bytes).map_err(ParseError::from)?;
        Ok(meta_chunk_data)
    }

    /// Get the meta chunk
    pub fn get_meta_chunk(&mut self) -> FileResult<FileMetaChunk> {
        let header = self.get_chunk_header()?;
        self.inner.seek(SeekFrom::Start(header.offset.into()))?;
        let data = self.get_meta_chunk_data()?;
        Ok(FileMetaChunk { header, data })
    }
}
