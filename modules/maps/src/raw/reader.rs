use super::file::*;
use super::parser;
use assembly_core::byteorder::{ReadBytesExt, LE};
use assembly_core::reader::{FileResult, ParseError};

use std::io::prelude::*;

pub trait TerrainReader: Read {
    fn read_terrain_header(&mut self) -> FileResult<TerrainHeader> {
        let mut header_bytes = [0 as u8; 15];
        self.read_exact(&mut header_bytes)?;
        let (_, header) = parser::parse_terrain_header(&header_bytes).map_err(ParseError::from)?;
        Ok(header)
    }

    fn read_terrain_chunk(&mut self) -> FileResult<TerrainChunk> {
        let index = self.read_u32::<LE>()?;
        Ok(TerrainChunk { index })
    }

    fn read_height_map_header(&mut self) -> FileResult<HeightMapHeader> {
        let mut header_bytes = [0 as u8; 36];
        self.read_exact(&mut header_bytes)?;
        let (_, header) =
            parser::parse_height_map_header(&header_bytes).map_err(ParseError::from)?;
        Ok(header)
    }

    fn read_height_map_data(&mut self, width: u32, height: u32) -> FileResult<Vec<f32>> {
        let len = (width * height) as usize;
        let mut bytes = Vec::with_capacity(len);
        bytes.resize(len, 0.0);
        self.read_f32_into::<LE>(bytes.as_mut_slice())?;
        Ok(bytes)
    }

    fn read_color_map_data(&mut self) -> FileResult<Vec<u32>> {
        let len = self.read_u32::<LE>()? as usize;
        let len = len * len;
        let mut bytes = vec![0; len];
        self.read_u32_into::<LE>(bytes.as_mut_slice())?;
        Ok(bytes)
    }

    fn read_embedded_file(&mut self) -> FileResult<Vec<u8>> {
        let len = self.read_u32::<LE>()? as usize;
        let mut bytes = vec![0; len];
        self.read_exact(bytes.as_mut_slice())?;
        Ok(bytes)
    }
}

impl<T> TerrainReader for T where T: Read {}
