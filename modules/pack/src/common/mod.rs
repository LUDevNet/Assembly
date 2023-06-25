//! # Common elements
//!
//! Both PK files and the PKI files are organised as

use std::{
    borrow::{Borrow, BorrowMut},
    collections::BTreeMap,
    fmt,
    ops::{ControlFlow, Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::md5::MD5Sum;

pub mod fs;
#[cfg(feature = "common-parser")]
pub mod parser;
pub mod writer;

/// Node in a CRC tree
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CRCTreeNode<D> {
    /// The [CRC][`crate::crc`] value of this file
    pub crc: u32,
    /// Binary tree node to the left
    pub left: i32,
    /// Binary tree node to the right
    pub right: i32,
    /// The data in this node
    pub data: D,
}

impl<D> Borrow<D> for CRCTreeNode<D> {
    fn borrow(&self) -> &D {
        &self.data
    }
}

impl<D> BorrowMut<D> for CRCTreeNode<D> {
    fn borrow_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

impl<D> Deref for CRCTreeNode<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D> DerefMut for CRCTreeNode<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Datastructure to hold a CRC tree.
///
/// Within the file, the trees are sorted by CRC value and organised in
/// binary tree. This is not necessarily the same as the Rust B-Tree, but
/// the ordering is good enough for what we need.
pub type CRCTree<T> = BTreeMap<u32, T>;

/// A trait to visit a CRC tree from a reader
pub trait CRCTreeVisitor<T> {
    /// The type of data to return on a premature break
    type Break;

    /// Called once for every
    fn visit(&mut self, crc: u32, data: T) -> ControlFlow<Self::Break>;
}

/// Simple visitor that collects a CRC tree to an instance of []
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CRCTreeCollector<T> {
    inner: CRCTree<T>,
}

impl<T> CRCTreeCollector<T> {
    /// Create a new collector
    pub fn new() -> Self {
        Self {
            inner: CRCTree::new(),
        }
    }

    /// Return the contained map
    pub fn into_inner(self) -> CRCTree<T> {
        self.inner
    }
}

impl<T> CRCTreeVisitor<T> for CRCTreeCollector<T> {
    type Break = ();

    fn visit(&mut self, crc: u32, data: T) -> ControlFlow<Self::Break> {
        self.inner.insert(crc, data);
        ControlFlow::Continue(())
    }
}

/// Metadata for a single file
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMeta {
    /// Size of the file
    pub size: u32,
    /// md5sum of the file
    #[serde(with = "crate::md5::padded")]
    pub hash: MD5Sum,
}

impl fmt::Display for FileMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.size, self.hash)
    }
}

/// Metadata for a file, raw and compressed
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetaPair {
    /// The raw metadata
    pub raw: FileMeta,
    /// The compressed metadata
    pub compressed: FileMeta,
}

impl FileMetaPair {
    /// Create a new File-Meta pair
    pub fn new(raw: FileMeta, compressed: FileMeta) -> Self {
        Self { raw, compressed }
    }

    /// Get the (relative) patcher URL for this file
    pub fn to_path(&self) -> String {
        let hash = format!("{:?}", self.raw.hash);
        let mut chars = hash.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        format!("{}/{}/{}.sd0", c1, c2, hash)
    }
}

impl fmt::Display for FileMetaPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.raw, self.compressed)
    }
}
