//! # Low level reading
use super::file::*;
use super::parser;

use assembly_core::reader::{FileError, FileResult};
use assembly_core::{nom::Finish, reader::ParseAt};

use std::io::prelude::*;
use std::{io::SeekFrom, num::NonZeroU32};

/// A low level reader class
pub struct LevelReader<T> {
    inner: T,
}

impl<T> LevelReader<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

fn get_offset(header: &FileMetaChunkData, id: u32) -> Option<NonZeroU32> {
    match id {
        2000 => NonZeroU32::new(header.chunk_2000_offset),
        2001 => NonZeroU32::new(header.chunk_2001_offset),
        2002 => NonZeroU32::new(header.chunk_2002_offset),
        _ => None,
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
    pub fn get_chunk(
        &mut self,
        header: &FileMetaChunkData,
        id: u32,
    ) -> Option<FileResult<ChunkHeader>> {
        get_offset(header, id).map(|offset| {
            self.inner.seek(SeekFrom::Start(u32::from(offset).into()))?;
            self.get_chunk_header()
        })
    }

    /// Load a chunk header
    pub fn get_chunk_header(&mut self) -> FileResult<ChunkHeader> {
        let mut header_bytes = [0; 20];
        self.inner.read_exact(&mut header_bytes)?;
        let (_rest, header) = parser::parse_chunk_header(&header_bytes)
            .finish()
            .at(0xbeef, &header_bytes)?;
        Ok(header)
    }

    /// Get the chunk meta data
    pub fn get_meta_chunk_data(&mut self) -> FileResult<FileMetaChunkData> {
        let mut meta_chunk_data_bytes = [0u8; 20];
        self.inner.read_exact(&mut meta_chunk_data_bytes)?;
        let (_rest, meta_chunk_data) = parser::parse_file_meta_chunk_data(&meta_chunk_data_bytes)
            .finish()
            .at(0xbeef, &meta_chunk_data_bytes)?;
        Ok(meta_chunk_data)
    }

    /// Get the meta chunk
    pub fn get_meta_chunk(&mut self) -> FileResult<FileMetaChunk> {
        let header = self.get_chunk_header()?;
        self.inner.seek(SeekFrom::Start(header.offset.into()))?;
        let data = self.get_meta_chunk_data()?;
        Ok(FileMetaChunk { header, data })
    }

    pub fn read_level_file(&mut self) -> FileResult<Level> {
        let header_1000 = self.get_chunk_header()?;

        if !header_1000.id == 1000 {
            return Err(FileError::Custom("Expected first chunk to be of type 1000"));
        }

        self.seek_to(&header_1000)?;
        let meta = self.get_meta_chunk_data()?;

        let env = self
            .get_chunk(&meta, 2000)
            .map(|res| {
                let header_2000 = res?;

                if header_2000.id != 2000 {
                    return Err(FileError::Custom("Expected 2000 chunk to be of type 2000"));
                }

                let buf = self.load_buf(meta.chunk_2000_offset, &header_2000)?;
                let env = parser::parse_env_chunk_data(&buf)
                    .finish()
                    .at(meta.chunk_2000_offset.into(), &buf)?
                    .1;

                // first section
                let sec1_base = (env.section1_address - header_2000.offset) as usize;
                let sec1 = parser::parse_section1(meta.version, &buf[sec1_base..])
                    .finish()
                    .at(env.section1_address.into(), &buf[sec1_base..])?
                    .1;

                // sky section
                let sky_base = (env.sky_address - header_2000.offset) as usize;
                let sky = parser::parse_sky_section(&buf[sky_base..])
                    .finish()
                    .at(env.sky_address.into(), &buf[sky_base..])?
                    .1;

                // TODO: third section
                Ok(Environment { sec1, sky })
            })
            .transpose()?;

        let objects = self
            .get_chunk(&meta, 2001)
            .map(|res| {
                let header_2001 = res?;

                if header_2001.id != 2001 {
                    return Err(FileError::Custom("Expected 2001 chunk to be of type 2001"));
                }

                let buf = self.load_buf(meta.chunk_2001_offset, &header_2001)?;
                let obj = parser::parse_objects_chunk_data(meta.version, &buf)
                    .finish()
                    .at(meta.chunk_2001_offset.into(), &buf)?
                    .1;

                let obj = obj
                    .parse_settings()
                    .map_err(|_| FileError::Custom("Failed to parse object settings"))?;

                Ok(obj.objects)
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Level { env, objects })
    }
}
