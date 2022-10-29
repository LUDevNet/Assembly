//! # General methods for aligned access to a byte buffer

use crate::{
    common::Latin1Str,
    file::{
        FDBHeader, FDBRowHeader, FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader,
        FDBTableHeader,
    },
    handle::Buffer,
    util::compare_bytes,
};
use bytemuck::from_bytes;
use displaydoc::Display;
use std::{cmp::Ordering, convert::TryInto, mem::size_of, ops::Range};
use thiserror::Error;

#[derive(Error, Debug, Display, Clone, PartialEq, Eq)]
/// Error for handling a buffer
pub enum BufferError {
    /// index out of bounds {0:?}
    OutOfBounds(Range<usize>),
    /// index not aligned {0}
    Unaligned(usize),
}

/// Result with a [`BufferError`]
pub type Res<T> = Result<T, BufferError>;

/*#[derive(Copy, Clone)]
/// Wrapper around a immutable reference to a byte slice
pub struct Buffer<'a>(&'a [u8]);

impl<'a> Deref for Buffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}*/

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

/// Additional methods on `&[u8]`
pub trait BufferExt: Buffer {
    /// Get a subslice a the given offset of the given length
    fn get_len_at(&self, start: usize, len: usize) -> Res<&[u8]>;
    /// Get a buffer as a latin1 string
    fn string(&self, addr: u32) -> Res<&Latin1Str>;
    /// Get i64
    fn i64(&self, addr: u32) -> Res<i64>;
    /// Get the table definition header at the given addr.
    fn table_def_header(&self, addr: u32) -> Res<FDBTableDefHeader>;
    /// Get the table data header at the given addr.
    fn table_data_header(&self, addr: u32) -> Res<FDBTableDataHeader>;
    /// Get the `FDBRowHeader` list entry at the given addr.
    fn row_header_list_entry(&self, addr: u32) -> Res<FDBRowHeaderListEntry>;
    /// Get the `FDBRowHeader` at the given addr.
    fn row_header(&self, addr: u32) -> Res<FDBRowHeader>;
}

impl BufferExt for [u8] {
    /// Get a subslice a the given offset of the given length
    fn get_len_at(&self, start: usize, len: usize) -> Res<&[u8]> {
        let end = start + len;
        let range = Range { start, end };
        self.get(range.clone())
            .ok_or(BufferError::OutOfBounds(range))
    }

    fn string(&self, addr: u32) -> Res<&Latin1Str> {
        let start = addr as usize;
        let buf = self.get(start..).ok_or_else(|| {
            let end = self.len();
            BufferError::OutOfBounds(Range { start, end })
        })?;
        Ok(Latin1Str::from_bytes_until_nul(buf))
    }

    fn i64(&self, addr: u32) -> Res<i64> {
        let start = addr as usize;
        let end = start + size_of::<u64>();
        if end > self.len() {
            Err(BufferError::OutOfBounds(Range { start, end }))
        } else {
            let (_, base) = self.split_at(start);
            let (bytes, _) = base.split_at(size_of::<u64>());
            let val = i64::from_le_bytes(bytes.try_into().unwrap());
            Ok(val)
        }
    }

    fn table_def_header(&self, addr: u32) -> Res<FDBTableDefHeader> {
        let buf = self.get_len_at(addr as usize, 12)?;
        let (a, buf) = buf.split_at(4);
        let (b, c) = buf.split_at(4);
        Ok(FDBTableDefHeader {
            column_count: u32::from_le_bytes(a.try_into().unwrap()),
            table_name_addr: u32::from_le_bytes(b.try_into().unwrap()),
            column_header_list_addr: u32::from_le_bytes(c.try_into().unwrap()),
        })
    }

    fn table_data_header(&self, addr: u32) -> Res<FDBTableDataHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        Ok(*from_bytes(buf))
    }

    /// Get the `FDBRowHeader` list entry at the given addr.
    fn row_header_list_entry(&self, addr: u32) -> Res<FDBRowHeaderListEntry> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBRowHeaderListEntry {
            row_header_addr: u32::from_le_bytes(a.try_into().unwrap()),
            row_header_list_next_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }

    /// Get the `FDBRowHeader` at the given addr.
    fn row_header(&self, addr: u32) -> Res<FDBRowHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        Ok(*from_bytes(buf))
    }
}
