//! # The structures as the appear in the file
use serde::{Deserialize, Serialize};

use crate::common::{CRCTreeNode, FileMetaPair};

/// Magic bytes at the start of a PK file
pub const MAGIC_START: [u8; 7] = [b'n', b'd', b'p', b'k', 0x01, 0xff, 0x00];
/// Magic bytes after files in a PK file
pub const MAGIC_SEP: [u8; 5] = [0xff, 0x00, 0x00, 0xdd, 0x00];

/// The header of a pack file#
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PKTrailer {
    /// The base addr of the file list
    pub file_list_base_addr: u32,
    /// Number of compressed files in this archive
    pub num_compressed: u32,
}

/// An entry for a single file
pub type PKEntry = CRCTreeNode<PKEntryData>;

/// Payload of the [`PKEntry`]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PKEntryData {
    /// File Metadata
    pub meta: FileMetaPair,
    /// Offset of the file data within the PK archive
    pub file_data_addr: u32,
    /// TODO: figure out
    pub is_compressed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<PKEntry>(), 60);
    }

    #[test]
    fn test() {
        let h: PKTrailer = bincode::deserialize(&[1, 0, 0, 0, 2, 0, 0, 0]).unwrap();
        assert_eq!(h.file_list_base_addr, 1);
        assert_eq!(h.num_compressed, 2);
    }
}
