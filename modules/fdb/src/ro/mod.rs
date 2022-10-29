//! Read-Only low level access to a database file

use std::sync::Arc;

pub use crate::handle::{BaseHandle, Handle, RefHandle};

pub mod buffer;
pub mod handle;
pub mod slice;

/// An owned, atomically-reference counted handle to a database
pub type ArcHandle<B, T> = BaseHandle<Arc<B>, T>;

impl<B: AsRef<[u8]>> ArcHandle<B, ()> {
    /// Create a new atomically-reference counted handle
    pub fn new_arc(inner: B) -> Self {
        Self::new(Arc::new(inner))
    }
}

impl<B: AsRef<[u8]>, T: Copy> ArcHandle<B, T> {
    /// Borrow the atomically-reference counted handle as a byte handle
    ///
    /// You can use this function to make cloning cheaper
    pub fn as_bytes_handle(&self) -> Handle<T> {
        BaseHandle {
            mem: self.mem.as_ref().as_ref(),
            raw: self.raw,
        }
    }
}

/// A handle that contains a slice
pub type SliceHandle<'a, T> = RefHandle<'a, [T]>;

/// A handle that contains a slice iterator
pub type SliceIterHandle<'a, T> = Handle<'a, std::slice::Iter<'a, T>>;
