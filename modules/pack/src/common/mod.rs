//! # Common elements
//!
//! Both PK files and the PKI files are organised as

use std::{
    borrow::{Borrow, BorrowMut},
    collections::BTreeMap,
    ops::{ControlFlow, Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

pub mod fs;
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
