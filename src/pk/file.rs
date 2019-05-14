//! # The structures as the appear in the file

/// The header of a pack file
pub struct PKHeader {
    pub file_list_base_addr: u32,
    pub value_1: u32,
}

/// An entry for a single file
#[derive(Debug)]
pub struct PKEntry {
    pub crc: u32,
    #[allow(dead_code)]
    pub left: u32,
    #[allow(dead_code)]
    pub right: u32,

    pub orig_file_size: u32,
    pub orig_file_hash: String,

    pub compr_file_size: u32,
    pub compr_file_hash: String,

    pub file_data_addr: u32,
    pub is_compressed: [u8;4],
}
