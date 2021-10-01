//! # Common elements
//!
//! Both PK files and the PKI files are organised as

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

pub mod parser;

/// Node in a CRC tree
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
