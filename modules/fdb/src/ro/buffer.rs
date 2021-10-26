//! # General methods for aligned access to a byte buffer

use crate::{
    common::Latin1Str,
    file::{
        FDBHeader, FDBRowHeader, FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader,
        FDBTableHeader,
    },
};
use assembly_core::buffer::{CastError, MinimallyAligned};
use bytemuck::from_bytes;
use displaydoc::Display;
use std::{
    cmp::Ordering,
    convert::TryInto,
    fmt,
    mem::size_of,
    ops::{Deref, Range},
};
use thiserror::Error;

#[derive(Error, Debug, Display, Clone, PartialEq, Eq)]
/// Error for handling a buffer
pub enum BufferError {
    /// index out of bounds {0:?}
    OutOfBounds(Range<usize>),
    /// index not aligned {0}
    Unaligned(usize),
}

#[derive(Copy, Clone)]
/// Wrapper around a immutable reference to a byte slice
pub struct Buffer<'a>(&'a [u8]);

/// Result with a [`BufferError`]
pub type Res<T> = Result<T, BufferError>;

impl<'a> Deref for Buffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Compares two name strings
///
/// ## Safety
///
/// This panics if name_bytes does not contains a null terminator
pub(crate) fn compare_bytes(bytes: &[u8], name_bytes: &[u8]) -> Ordering {
    for i in 0..bytes.len() {
        match name_bytes[i].cmp(&bytes[i]) {
            Ordering::Equal => {}
            Ordering::Less => {
                // the null terminator is a special case of this one
                return Ordering::Less;
            }
            Ordering::Greater => {
                return Ordering::Greater;
            }
        }
    }
    if name_bytes[bytes.len()] == 0 {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

/// Get a reference to a type at the given address of this buffer
///
/// This functions checks whether the offset and alignment is valid
pub fn get_at<T>(buf: &[u8], addr: usize) -> Res<&T> {
    let base = buf.as_ptr();
    let len = buf.len();
    let size = std::mem::size_of::<T>();
    let align = std::mem::align_of::<T>();

    let needed = addr
        .checked_add(size)
        .ok_or(BufferError::OutOfBounds(addr..len))?;

    if needed > len {
        return Err(BufferError::OutOfBounds(addr..needed));
    }

    let start = unsafe { base.add(addr) };
    if 0 != start.align_offset(align) {
        return Err(BufferError::Unaligned(addr));
    }
    Ok(unsafe { &*(start as *const T) })
}

/// Get a reference to a slice at the given address of this buffer
///
/// This functions checks whether the offset and alignment is valid
pub fn get_slice_at<T>(buf: &[u8], addr: usize, count: usize) -> Res<&[T]> {
    let base = buf.as_ptr();
    let len = buf.len();
    let size = std::mem::size_of::<T>();
    let align = std::mem::align_of::<T>();

    let slice_bytes = size
        .checked_mul(count)
        .ok_or(BufferError::OutOfBounds(addr..len))?;

    let needed = addr
        .checked_add(slice_bytes)
        .ok_or(BufferError::OutOfBounds(addr..len))?;

    if needed > len {
        return Err(BufferError::OutOfBounds(addr..needed));
    }

    let start = unsafe { base.add(addr) };
    if 0 != start.align_offset(align) {
        return Err(BufferError::Unaligned(addr));
    }

    Ok(unsafe { &*(std::ptr::slice_from_raw_parts(start as *const T, count)) })
}

/// Get the database header
#[cfg(target_endian = "little")]
pub fn header_ref(buf: &[u8]) -> Res<&FDBHeader> {
    get_at(buf, 0)
}

/// Get the header of the file.
pub fn header(buf: &[u8], _: ()) -> Res<FDBHeader> {
    Ok(*header_ref(buf)?)
}

/// Get the table slice
pub fn table_headers<'a>(buf: &'a [u8], header: &'a FDBHeader) -> Res<&'a [FDBTableHeader]> {
    get_slice_at(
        buf,
        header.tables.base_offset as usize,
        header.tables.count as usize,
    )
}

/// Get the table definition reference
pub fn table_definition_ref(buf: &[u8], header: FDBTableHeader) -> Res<&FDBTableDefHeader> {
    get_at(buf, header.table_def_header_addr as usize)
}

/// Get the table data reference
pub fn table_data_ref(buf: &[u8], header: FDBTableHeader) -> Res<&FDBTableDataHeader> {
    get_at(buf, header.table_data_header_addr as usize)
}

/// Get the table definition header
pub fn table_definition(buf: &[u8], header: FDBTableHeader) -> Res<FDBTableDefHeader> {
    table_definition_ref(buf, header).map(|x| *x)
}

/// Get the table data header
pub fn table_data(buf: &[u8], header: FDBTableHeader) -> Res<FDBTableDataHeader> {
    table_data_ref(buf, header).map(|x| *x)
}

/// Compares the name given by `bytes` with the one referenced in `table_header`
pub fn cmp_table_header_name(buf: &[u8], bytes: &[u8], table_header: FDBTableHeader) -> Ordering {
    let def_header_addr = table_header.table_def_header_addr;
    // FIXME: what to do with this unwrap?
    let def_header = get_at::<FDBTableDefHeader>(buf, def_header_addr as usize).unwrap();
    let name_addr = def_header.table_name_addr as usize;

    let name_bytes = buf.get(name_addr..).unwrap();

    compare_bytes(bytes, name_bytes)
}

impl<'a> Buffer<'a> {
    /// Creates a new instance.
    pub fn new(buf: &'a [u8]) -> Self {
        Self(buf)
    }

    /// Returns the contained byte slice
    pub fn as_bytes(self) -> &'a [u8] {
        self.0
    }

    /// Try to cast to T
    pub fn try_cast<T: MinimallyAligned>(
        self,
        offset: u32,
    ) -> std::result::Result<&'a T, CastError> {
        assembly_core::buffer::try_cast(self.as_bytes(), offset)
    }

    /// Try to cast to T
    pub fn try_cast_slice<T: MinimallyAligned>(
        self,
        offset: u32,
        len: u32,
    ) -> std::result::Result<&'a [T], CastError> {
        assembly_core::buffer::try_cast_slice(self.as_bytes(), offset, len)
    }

    /// Cast to T
    pub fn cast<T: MinimallyAligned>(self, offset: u32) -> &'a T {
        assembly_core::buffer::cast(self.as_bytes(), offset)
    }

    /// Cast to slice of T
    pub fn cast_slice<T: MinimallyAligned>(self, offset: u32, len: u32) -> &'a [T] {
        assembly_core::buffer::cast_slice(self.as_bytes(), offset, len)
    }

    /// Get a subslice a the given offset of the given length
    pub fn get_len_at(self, start: usize, len: usize) -> Res<&'a [u8]> {
        let end = start + len;
        self.0
            .get(Range { start, end })
            .ok_or(BufferError::OutOfBounds(Range { start, end }))
    }

    /// Get a buffer as a latin1 string
    pub fn string(self, addr: u32) -> Res<&'a Latin1Str> {
        let start = addr as usize;
        let mut buf = self.0.get(start..).ok_or_else(|| {
            let end = self.0.len();
            BufferError::OutOfBounds(Range { start, end })
        })?;
        if let Some(nullpos) = memchr::memchr(0, buf) {
            buf = buf.split_at(nullpos).0;
        }
        Ok(Latin1Str::new(buf))
    }

    /// Get i64
    pub fn i64(self, addr: u32) -> Res<i64> {
        let start = addr as usize;
        let end = start + size_of::<u64>();
        if end > self.0.len() {
            Err(BufferError::OutOfBounds(Range { start, end }))
        } else {
            let (_, base) = self.0.split_at(start);
            let (bytes, _) = base.split_at(size_of::<u64>());
            let val = i64::from_le_bytes(bytes.try_into().unwrap());
            Ok(val)
        }
    }

    /// Get the table definition header at the given addr.
    pub fn table_def_header(&self, addr: u32) -> Res<FDBTableDefHeader> {
        let buf = self.get_len_at(addr as usize, 12)?;
        let (a, buf) = buf.split_at(4);
        let (b, c) = buf.split_at(4);
        Ok(FDBTableDefHeader {
            column_count: u32::from_le_bytes(a.try_into().unwrap()),
            table_name_addr: u32::from_le_bytes(b.try_into().unwrap()),
            column_header_list_addr: u32::from_le_bytes(c.try_into().unwrap()),
        })
    }

    /// Get the table data header at the given addr.
    pub fn table_data_header(self, addr: u32) -> Res<FDBTableDataHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        Ok(*from_bytes(buf))
    }

    /// Get the `FDBRowHeader` list entry at the given addr.
    pub fn row_header_list_entry(self, addr: u32) -> Res<FDBRowHeaderListEntry> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBRowHeaderListEntry {
            row_header_addr: u32::from_le_bytes(a.try_into().unwrap()),
            row_header_list_next_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }

    /// Get the `FDBRowHeader` at the given addr.
    pub fn row_header(self, addr: u32) -> Res<FDBRowHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        Ok(*from_bytes(buf))
    }
}

impl<'a> fmt::Debug for Buffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("base", &self.0.as_ptr())
            .field("len", &self.0.len())
            .finish()
    }
}
