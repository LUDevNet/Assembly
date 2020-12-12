//! Read-Only low level access to a database file

use assembly_core::buffer::{CastError, MinimallyAligned, Repr};

use self::buffer::Buffer;

use super::file::ArrayHeader;

pub mod buffer;
pub mod handle;
pub mod slice;

#[derive(Copy, Clone, Debug)]
/// The basic handle into a byte buffer
pub struct Handle<'a, T> {
    pub(super) buffer: Buffer<'a>,
    pub(super) raw: T,
}

impl<'a, T> Handle<'a, T> {
    /// Get a reference to the raw value inside
    pub fn raw(&self) -> &T {
        &self.raw
    }

    /// Get a reference to the raw value inside
    pub fn raw_mut(&mut self) -> &mut T {
        &mut self.raw
    }

    /// Returns a copy of the contained buffer
    pub fn buf(self) -> Buffer<'a> {
        self.buffer
    }

    /// Get the raw value out of the handle
    pub fn into_raw(self) -> T {
        self.raw
    }

    /// Wrap a value as a handle
    pub(crate) fn wrap<R>(&self, raw: R) -> Handle<'a, R> {
        Handle {
            buffer: self.buffer,
            raw,
        }
    }

    /// Map a cast reference
    pub(crate) fn try_map_cast<R: MinimallyAligned>(&self, offset: u32) -> Result<RefHandle<'a, R>, CastError> {
        let raw: &'a R = self.buffer.try_cast(offset)?;
        Ok(self.wrap(raw))
    }

    /// Map a casted slice
    pub(crate) fn try_map_cast_slice<R: MinimallyAligned>(&self, offset: u32, count: u32) -> Result<RefHandle<'a, [R]>, CastError> {
        let raw: &'a [R] = self.buffer.try_cast_slice(offset, count)?;
        Ok(self.wrap(raw))
    }

    /// Map a casted array
    pub(crate) fn try_map_cast_array<R: MinimallyAligned>(&self, array: ArrayHeader) -> Result<RefHandle<'a, [R]>, CastError> {
        let raw: &'a [R] = self.buffer.try_cast_slice(array.base_offset, array.count)?;
        Ok(self.wrap(raw))
    }

    /// Map something with a closure
    pub fn map<X>(self, mapper: impl Fn(Buffer<'a>, T) -> X) -> Handle<'a, X> {
        let raw = mapper(self.buffer, self.raw);
        Handle { buffer: self.buffer, raw }
    }

    /// Map the value with a closure
    pub fn map_val<X>(self, mapper: impl Fn(T) -> X) -> Handle<'a, X> {
        let raw = mapper(self.raw);
        Handle { buffer: self.buffer, raw }
    }

    /// Map something with a closure
    pub fn try_map<X, E>(self, mapper: impl Fn(Buffer<'a>, T) -> Result<X, E>) -> Result<Handle<'a, X>, E> {
        let raw = mapper(self.buffer, self.raw)?;
        Ok(Handle { buffer: self.buffer, raw })
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