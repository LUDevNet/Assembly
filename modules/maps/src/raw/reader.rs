use assembly_core::reader::{FileError, FileResult};
use assembly_core::byteorder::{ReadBytesExt,LE};
use super::parser;
use super::file::*;

use std::io::prelude::*;

pub trait TerrainReader: Read {
    fn read_terrain_header(&mut self) -> FileResult<TerrainHeader> {
        let mut header_bytes = [0 as u8; 15];
        self.read_exact(&mut header_bytes).map_err(FileError::Read)?;
        let (_, header) = parser::parse_terrain_header(&mut header_bytes)?;
        Ok(header)
    }

    fn read_terrain_chunk(&mut self) -> FileResult<TerrainChunk> {
        let index = self.read_u32::<LE>().map_err(FileError::Read)?;
        return Ok(TerrainChunk{index})
    }

    fn read_height_map_header(&mut self) -> FileResult<HeightMapHeader> {
        let mut header_bytes = [0 as u8; 36];
        self.read_exact(&mut header_bytes).map_err(FileError::Read)?;
        let (_, header) = parser::parse_height_map_header(&mut header_bytes)?;
        Ok(header)
    }

    fn read_height_map_data(&mut self, width: u32, height: u32) -> FileResult<Vec<f32>> {
        let len = (width * height) as usize;
        let mut bytes = Vec::with_capacity(len);
        bytes.resize(len, 0.0);
        self.read_f32_into::<LE>(bytes.as_mut_slice()).map_err(FileError::Read)?;
        Ok(bytes)
    }

    fn read_color_map_data(&mut self) -> FileResult<Vec<u32>> {
        let len = self.read_u32::<LE>().map_err(FileError::Read)? as usize;
        let len = len * len;
        let mut bytes = Vec::with_capacity(len);
        bytes.resize(len, 0);
        self.read_u32_into::<LE>(bytes.as_mut_slice()).map_err(FileError::Read)?;
        Ok(bytes)
    }

    fn read_embedded_file(&mut self) -> FileResult<Vec<u8>> {
        let len = self.read_u32::<LE>().map_err(FileError::Read)? as usize;
        let mut bytes = Vec::with_capacity(len);
        bytes.resize(len, 0);
        self.read_exact(bytes.as_mut_slice()).map_err(FileError::Read)?;
        Ok(bytes)
    }
}

impl<T> TerrainReader for T where T: Read {}
