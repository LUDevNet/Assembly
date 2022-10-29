use assembly_core::buffer::{CastError, MinimallyAligned, Repr, Buffer};
use assembly_fdb_core::file::ArrayHeader;
use latin1str::Latin1Str;
use std::{convert::TryFrom, mem::size_of, ops::Deref, result::Result};

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
pub type Handle<'a, T> = BaseHandle<&'a [u8], T>;

impl<'a, T> Handle<'a, T> {
    /// Returns a copy of the contained buffer
    pub fn buf(self) -> &'a [u8] {
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
    pub fn map<X>(self, mapper: impl Fn(&'a [u8], T) -> X) -> Handle<'a, X> {
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
        mapper: impl Fn(&'a [u8], T) -> Result<X, E>,
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

/// Try from a handle
pub trait TryFromHandle<'a, T>: Sized {
    /// Error type
    type Error;
    /// Conversion function
    fn try_from(h: Handle<'a, T>) -> Result<Self, Self::Error>;
}

/// A handle that contains a reference
pub type RefHandle<'a, T> = Handle<'a, &'a T>;

impl<'a, T> RefHandle<'a, [T]> {
    /// Get the reference at `index`
    pub fn get(self, index: usize) -> Option<RefHandle<'a, T>> {
        self.raw.get(index).map(|raw| self.wrap(raw))
    }
}

impl<'a, T: Repr> RefHandle<'a, T> {
    /// Extract a value from a reference
    pub fn map_extract(self) -> Handle<'a, T::Value> {
        self.wrap(self.raw.extract())
    }
}

/// A handle that contains a slice iterator
pub type SliceIterHandle<'a, T> = Handle<'a, std::slice::Iter<'a, T>>;

/// Get a buffer as a latin1 string
pub fn get_string(buf: &[u8], offset: u32) -> Result<&Latin1Str, CastError> {
    let start = offset as usize;
    let buf = buf.get(start..).ok_or(CastError::OutOfBounds { offset })?;
    Ok(Latin1Str::from_bytes_until_nul(buf))
}

/// Get i64
pub fn get_i64(buf: &[u8], addr: u32) -> Result<i64, CastError> {
    let start = addr as usize;
    let end = start + size_of::<u64>();
    if end > buf.len() {
        Err(CastError::OutOfBounds { offset: addr })
    } else {
        let (_, base) = buf.split_at(start);
        let (bytes, _) = base.split_at(size_of::<u64>());
        let val = i64::from_le_bytes(<[u8; 8]>::try_from(bytes).unwrap());
        Ok(val)
    }
}
