//! Read-Only low level access to a database file

use std::{ops::Deref, sync::Arc};

use assembly_core::buffer::{CastError, MinimallyAligned, Repr};

use self::buffer::Buffer;

use super::file::ArrayHeader;

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
            mem: Buffer::new(self.mem.as_ref().as_ref()),
            raw: self.raw,
        }
    }
}

/// Base type for a handle to an in-memory data structure
///
/// This is basic layout of a handle to an in-memory FDB database.
/// Internally, there is a pointer (`&[u8]`/`Box<[u8]>`/`Rc<[u8]>`/`Arc<Mmap>`/…)
/// the memory slice as well as a value that represents the
/// current target.
#[derive(Clone, Debug)]
pub struct BaseHandle<P: Deref, T>
where
    <P as Deref>::Target: AsRef<[u8]>,
{
    /// The memory pointer
    pub(super) mem: P,
    /// The raw value
    pub(super) raw: T,
}

impl<P, T> Copy for BaseHandle<P, T>
where
    P: Deref + Copy,
    T: Copy,
    <P as Deref>::Target: AsRef<[u8]>,
{
}

impl<P: Deref> BaseHandle<P, ()>
where
    <P as Deref>::Target: AsRef<[u8]>,
{
    /// Creates a new handle
    pub fn new(mem: P) -> Self {
        Self { mem, raw: () }
    }
}

impl<T, P: Deref> BaseHandle<P, Option<T>>
where
    <P as Deref>::Target: AsRef<[u8]>,
{
    /// Turns a handle of an option into an option of a handle
    pub fn transpose(self) -> Option<BaseHandle<P, T>> {
        if let Some(raw) = self.raw {
            Some(BaseHandle { mem: self.mem, raw })
        } else {
            None
        }
    }
}

impl<P: Deref, T> BaseHandle<P, T>
where
    <P as Deref>::Target: AsRef<[u8]>,
{
    /// Get a reference to the raw value inside
    pub fn raw(&self) -> &T {
        &self.raw
    }

    /// Get a reference to the raw value inside
    pub fn raw_mut(&mut self) -> &mut T {
        &mut self.raw
    }

    /// Get the byte slice for the whole database
    pub fn as_bytes(&self) -> &[u8] {
        self.mem.deref().as_ref()
    }

    /// Replace the value that is stored with the memory pointer
    pub fn replace<O>(self, raw: O) -> BaseHandle<P, O> {
        BaseHandle { mem: self.mem, raw }
    }
}

/// The basic handle into a byte buffer
pub type Handle<'a, T> = BaseHandle<Buffer<'a>, T>;

impl<'a, T> Handle<'a, T> {
    /// Returns a copy of the contained buffer
    pub fn buf(self) -> Buffer<'a> {
        self.mem
    }

    /// Get the raw value out of the handle
    pub fn into_raw(self) -> T {
        self.raw
    }

    /// Wrap a value as a handle
    pub(crate) fn wrap<R>(&self, raw: R) -> Handle<'a, R> {
        Handle { mem: self.mem, raw }
    }

    /// Map a cast reference
    pub(crate) fn try_map_cast<R: MinimallyAligned>(
        &self,
        offset: u32,
    ) -> Result<RefHandle<'a, R>, CastError> {
        let raw: &'a R = self.mem.try_cast(offset)?;
        Ok(self.wrap(raw))
    }

    /// Map a casted slice
    pub(crate) fn try_map_cast_slice<R: MinimallyAligned>(
        &self,
        offset: u32,
        count: u32,
    ) -> Result<RefHandle<'a, [R]>, CastError> {
        let raw: &'a [R] = self.mem.try_cast_slice(offset, count)?;
        Ok(self.wrap(raw))
    }

    /// Map a casted array
    pub(crate) fn try_map_cast_array<R: MinimallyAligned>(
        &self,
        array: ArrayHeader,
    ) -> Result<RefHandle<'a, [R]>, CastError> {
        let raw: &'a [R] = self.mem.try_cast_slice(array.base_offset, array.count)?;
        Ok(self.wrap(raw))
    }

    /// Map something with a closure
    pub fn map<X>(self, mapper: impl Fn(Buffer<'a>, T) -> X) -> Handle<'a, X> {
        let raw = mapper(self.mem, self.raw);
        Handle { mem: self.mem, raw }
    }

    /// Map the value with a closure
    pub fn map_val<X>(self, mapper: impl Fn(T) -> X) -> Handle<'a, X> {
        let raw = mapper(self.raw);
        Handle { mem: self.mem, raw }
    }

    /// Map something with a closure
    pub fn try_map<X, E>(
        self,
        mapper: impl Fn(Buffer<'a>, T) -> Result<X, E>,
    ) -> Result<Handle<'a, X>, E> {
        let raw = mapper(self.mem, self.raw)?;
        Ok(Handle { mem: self.mem, raw })
    }
}

impl<'a, T> Iterator for Handle<'a, T>
where
    T: Iterator,
{
    type Item = Handle<'a, T::Item>;

    /// Returns a copy of the contained buffer
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a, T> RefHandle<'a, [T]> {
    /// Get the reference at `index`
    pub fn get(self, index: usize) -> Option<RefHandle<'a, T>> {
        self.raw.get(index).map(|raw| self.wrap(raw))
    }
}

/// A handle that contains a reference
pub type RefHandle<'a, T> = Handle<'a, &'a T>;

impl<'a, T: Repr> RefHandle<'a, T> {
    /// Extract a value from a reference
    pub fn map_extract(self) -> Handle<'a, T::Value> {
        self.wrap(self.raw.extract())
    }
}

/// A handle that contains a slice
pub type SliceHandle<'a, T> = RefHandle<'a, [T]>;

/// A handle that contains a slice iterator
pub type SliceIterHandle<'a, T> = Handle<'a, std::slice::Iter<'a, T>>;
