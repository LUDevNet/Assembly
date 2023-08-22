//! Public data structures for pack index files
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::crc::CRC;

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
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PackIndexFile {
    /// The list of PK archive paths
    pub archives: Vec<PackFileRef>,
    /// The map from CRC to file metadata
    pub files: BTreeMap<CRC, FileRef>,
}

impl PackIndexFile {
    /// Add a new pack to this PKI
    pub fn add_pack(&mut self, path: String, compressed: bool) -> PackIndexHandle<'_> {
        let index = self.archives.len() as u32;
        self.archives.push(PackFileRef { path });
        PackIndexHandle {
            pki: self,
            index,
            compressed,
        }
    }
}

/// Handle to a specific pack file
pub struct PackIndexHandle<'a> {
    pki: &'a mut PackIndexFile,
    index: u32,
    compressed: bool,
}

impl<'a> PackIndexHandle<'a> {
    /// add more files to this pack file
    pub fn add_files<I: Iterator<Item = CRC>>(&mut self, crcs: I) {
        for crc in crcs {
            self.add_file(crc)
        }
    }

    /// add one file to this pack file
    pub fn add_file(&mut self, crc: CRC) {
        self.pki.files.entry(crc).or_insert(FileRef {
            category: u32::from(self.compressed),
            pack_file: self.index,
        });
    }
}
