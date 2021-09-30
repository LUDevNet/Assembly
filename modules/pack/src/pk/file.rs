//! # The structures as the appear in the file
use serde::{Deserialize, Serialize};

use crate::{common::CRCTreeNode, md5::MD5Sum};

/// The header of a pack file#
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PKHeader {
    pub file_list_base_addr: u32,
    pub value_1: u32,
}

/// An entry for a single file
pub type PKEntry = CRCTreeNode<PKEntryData>;

/// Payload of the [`PKEntry`]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PKEntryData {
    /// Size of the decompressed file
    pub orig_file_size: u32,
    /// MD5sum of the decompressed file
    #[serde(with = "crate::md5::padded")]
    pub orig_file_hash: MD5Sum,

    /// Size of the compressed file
    pub compr_file_size: u32,
    /// MD5sum of the compressed file
    #[serde(with = "crate::md5::padded")]
    pub compr_file_hash: MD5Sum,

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
        let h: PKHeader = bincode::deserialize(&[1, 0, 0, 0, 2, 0, 0, 0]).unwrap();
        assert_eq!(h.file_list_base_addr, 1);
        assert_eq!(h.value_1, 2);
    }
}
