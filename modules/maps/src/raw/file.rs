//! ### General structs and data

#[derive(Debug)]
pub struct TerrainHeader {
    pub version: u8,
    pub value_1: u8,
    pub value_2: u8,
    pub chunk_count: u32,
    pub width_in_chunks: u32,
    pub height_in_chunks: u32,
}

#[derive(Debug)]
pub struct TerrainChunk {
    pub index: u32,
}

#[derive(Debug)]
pub struct HeightMapHeader {
    pub width: u32,
    pub height: u32,
    pub pos_x: f32,
    /// (or y in 2D)
    pub pos_z: f32,
    /// these 4 ints seem to stay mostly constant, but sometimes change
    pub _1: u32,
    pub _2: u32,
    pub _3: u32,
    pub _4: u32,
    /// this might sound silly, but is it y?
    pub _5: f32,
}
