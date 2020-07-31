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
    /// Seek to the chunk data
    pub fn seek_to(&mut self, header: &ChunkHeader) -> FileResult<()> {
        self.inner.seek(SeekFrom::Start(header.offset.into()))?;
        Ok(())
    }

    pub fn load_buf(&mut self, base: u32, header: &ChunkHeader) -> FileResult<Vec<u8>> {
        self.seek_to(header)?;
        let len = header.size - (header.offset - base);
        let mut buf = vec![0; len as usize];
        self.inner.read_exact(&mut buf[..])?;
        Ok(buf)
    }

    /// Seek meta
    pub fn seek_meta(&mut self, header: &FileMetaChunkData, id: u32) -> Option<FileResult<()>> {
        let offset = match id {
            2000 => header.chunk_2000_offset,
            2001 => header.chunk_2001_offset,
            2002 => header.chunk_2002_offset,
            _ => return None,
        };
        Some(match self.inner.seek(SeekFrom::Start(offset.into())) {
            Err(e) => Err(e.into()),
            Ok(_) => Ok(()),
        })
    }

    /// Load a chunk header
    pub fn get_chunk_header(&mut self) -> FileResult<ChunkHeader> {
        let mut header_bytes = [0; 20];
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
