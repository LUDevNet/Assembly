//! Public data structures for pack index files
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The data for a single pack file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackFileRef {
    /// The path to the pack file relative to the installation
    pub path: String,
}

/// The data associated with each file
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct FileRef {
    /// The category of this file. The least significant byte indicates whether
    /// the file should be compressed.
    pub category: u32,
    /// The index of the pack file in [`PackIndexFile::archives`]
    pub pack_file: u32,
}

/// The entire data in a PKI file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackIndexFile {
    /// The list of PK archive paths
    pub archives: Vec<PackFileRef>,
    /// The map from CRC to file metadata
    pub files: BTreeMap<u32, FileRef>,
}
