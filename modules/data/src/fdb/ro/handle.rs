//! The low-level Handle API
//!
//! This API uses handles that store the data of one header alongside
//! a reference into the in-memory file.

use super::{Handle, buffer::{Buffer, BufferError}, slice::{
        FDBBucketHeaderSlice, FDBColumnHeaderSlice, FDBFieldDataSlice, FDBTableHeaderSlice,
        Latin1Str,
    }};
use crate::fdb::{
    core::ValueType,
    file::{
        FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
};
use std::borrow::Cow;

/// Custom result type for this module
pub type Result<'a, T> = std::result::Result<Handle<'a, T>, BufferError>;

/// The basic database handle
pub type Database<'a> = Handle<'a, ()>;

impl<'a> Database<'a> {
    /// Create a new database handle
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: Buffer::new(buffer),
            raw: (),
        }
    }

    /// Get the header for the local database
    pub fn header(&self) -> Result<'a, FDBHeader> {
        let header = self.buffer.header()?;
        Ok(self.wrap(header))
    }
}

impl<'a> Handle<'a, FDBHeader> {
    /// Get the number of tables
    pub fn table_count(&self) -> u32 {
        self.raw.tables.count
    }

    /// Get the table header slice
    pub fn table_header_list(&self) -> Result<'a, FDBTableHeaderSlice<'a>> {
        let len = self.table_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.tables.base_offset as usize, len)?;
        Ok(self.wrap(FDBTableHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;
    type IntoIter = Handle<'a, FDBTableHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBTableHeader> {
    /// Get the table definition header
    pub fn table_def_header(&self) -> Result<'a, FDBTableDefHeader> {
        let raw = self
            .buffer
            .table_def_header(self.raw.table_def_header_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the table data header
    pub fn table_data_header(&self) -> Result<'a, FDBTableDataHeader> {
        let raw = self
            .buffer
            .table_data_header(self.raw.table_data_header_addr)?;
        Ok(self.wrap(raw))
    }
}

impl<'a> Handle<'a, FDBTableDefHeader> {
    /// Get the number of columns
    pub fn column_count(&self) -> u32 {
        self.raw.column_count
    }

    /// Get the name of the table
    pub fn table_name(&self) -> Result<'a, &'a Latin1Str> {
        let raw = self.buffer.string(self.raw.table_name_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the column header list
    pub fn column_header_list(&self) -> Result<'a, FDBColumnHeaderSlice<'a>> {
        let len = self.column_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.column_header_list_addr as usize, len)?;
        Ok(self.wrap(FDBColumnHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;
    type IntoIter = Handle<'a, FDBColumnHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBColumnHeader> {
    /// Get the name of the column
    pub fn column_name(&self) -> Result<'a, &'a Latin1Str> {
        let raw = self.buffer.string(self.raw.column_name_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the type of the column
    pub fn column_data_type(&self) -> ValueType {
        ValueType::from(self.raw.column_data_type)
    }
}

impl<'a> Handle<'a, FDBTableDataHeader> {
    /// Get the number of buckets
    pub fn bucket_count(&self) -> u32 {
        self.raw.buckets.count
    }

    /// Get the slice of buckets
    pub fn bucket_header_list(&self) -> Result<'a, FDBBucketHeaderSlice<'a>> {
        let len = self.bucket_count() as usize * 4;
        let buf = self
            .buffer
            .get_len_at(self.raw.buckets.base_offset as usize, len)?;
        Ok(self.wrap(FDBBucketHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;
    type IntoIter = Handle<'a, FDBBucketHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBBucketHeader> {
    /// Get the first row header entry or `None`
    pub fn first(&self) -> Option<Result<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_head_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(
                self.buffer
                    .row_header_list_entry(addr)
                    .map(|e| self.wrap(e)),
            )
        }
    }

    /// Get an iterator over all buckets
    pub fn bucket_iter(&self) -> Handle<'a, FDBRowHeaderRef> {
        self.wrap(FDBRowHeaderRef(self.raw.row_header_list_head_addr))
    }
}

#[derive(Debug, Copy, Clone)]
/// A newtype for a row header reference
pub struct FDBRowHeaderRef(u32);

impl<'a> Iterator for Handle<'a, FDBRowHeaderRef> {
    type Item = Result<'a, FDBRowHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.raw.0;
        if addr == 0xFFFFFFFF {
            None
        } else {
            match self.buffer.row_header_list_entry(addr) {
                Ok(e) => {
                    self.raw.0 = e.row_header_list_next_addr;
                    match self.buffer.row_header(e.row_header_addr) {
                        Ok(rh) => Some(Ok(self.wrap(rh))),
                        Err(e) => {
                            self.raw.0 = 0xFFFFFFFF;
                            Some(Err(e))
                        }
                    }
                }
                Err(e) => {
                    self.raw.0 = 0xFFFFFFFF;
                    Some(Err(e))
                }
            }
        }
    }
}

impl<'a> Handle<'a, FDBRowHeaderListEntry> {
    /// Get the next row header list entry instance
    pub fn next(&self) -> Option<Result<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_next_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(
                self.buffer
                    .row_header_list_entry(addr)
                    .map(|e| self.wrap(e)),
            )
        }
    }

    /// Get the associated row header.
    pub fn row_header(&self) -> Result<'a, FDBRowHeader> {
        let e = self.buffer.row_header(self.raw.row_header_addr)?;
        Ok(self.wrap(e))
    }
}

impl<'a> Handle<'a, FDBRowHeader> {
    /// Get the number of fields
    pub fn field_count(&self) -> u32 {
        self.raw.fields.count
    }

    /// Get the slice of fields
    pub fn field_data_list(&self) -> Result<'a, FDBFieldDataSlice> {
        let len = self.field_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.fields.base_offset as usize, len)?;
        Ok(self.wrap(FDBFieldDataSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;
    type IntoIter = Handle<'a, FDBFieldDataSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBFieldDataSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, &'a Latin1Str> {
    /// Decode the string contained in this handle
    pub fn to_str(&self) -> Cow<'a, str> {
        self.raw.decode()
    }
}
