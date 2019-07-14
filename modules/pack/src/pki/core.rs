//! Public data structures for pack index files
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PackFileRef {
    pub path: String,
}

#[derive(Debug, Copy, Clone)]
pub struct FileRef {
    pub category: u32,
    pub pack_file: u32,
}

pub struct PackIndexFile {
    pub archives: Vec<PackFileRef>,
    pub files: BTreeMap<u32, FileRef>,
}
