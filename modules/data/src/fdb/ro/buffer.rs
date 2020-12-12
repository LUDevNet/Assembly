//! # General methods for aligned access to a byte buffer

use super::slice::Latin1Str;
use crate::fdb::file::{
    FDBHeader, FDBRowHeader, FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader,
    FDBTableHeader,
};
use assembly_core::{
    buffer::{CastError, MinimallyAligned},
    displaydoc::Display,
};
use bytemuck::from_bytes;
use std::{
    convert::TryInto,
    fmt,
    ops::{Deref, Range},
};
use thiserror::Error;

#[derive(Error, Debug, Display)]
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

    /// Get a reference to a type at the given address of this buffer
    ///
    /// This functions checks whether the offset and alignment is valid
    pub fn get_at<T>(self, addr: usize) -> Res<&'a T> {
        let base = self.0.as_ptr();
        let len = self.0.len();
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
        Err(BufferError::Unaligned(addr))
    }

    /// Get a reference to a slice at the given address of this buffer
    ///
    /// This functions checks whether the offset and alignment is valid
    pub fn get_slice_at<T>(self, addr: usize, count: usize) -> Res<&'a [T]> {
        let base = self.0.as_ptr();
        let len = self.0.len();
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
    pub fn header_ref(self) -> Res<&'a FDBHeader> {
        self.get_at(0)
    }

    /// Get the table slice
    pub fn table_headers(self, header: &'a FDBHeader) -> Res<&'a [FDBTableHeader]> {
        self.get_slice_at(
            header.tables.base_offset as usize,
            header.tables.count as usize,
        )
    }

    /// Get a subslice a the given offset of the given length
    pub fn get_len_at(self, start: usize, len: usize) -> Res<&'a [u8]> {
        let end = start + len;
        self.0
            .get(Range { start, end })
            .ok_or(BufferError::OutOfBounds(Range { start, end }))
    }

    /// Get a buffer as a latin1 string
    // FIXME: why does this not check for the terminating null byte?
    pub fn string(self, addr: u32) -> Res<&'a Latin1Str> {
        let start = addr as usize;
        let buf = self.0.get(start..).ok_or_else(|| {
            let end = self.0.len();
            BufferError::OutOfBounds(Range { start, end })
        })?;
        Ok(Latin1Str::new(buf))
    }

    /// Get the header of the file.
    pub fn header(self) -> Res<FDBHeader> {
        Ok(*self.header_ref()?)
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
