//! The structures, as they are serialized
//!
//! This module contains the low-level structs that make up the FDB file. These
//! structures are annotated with `#[repr(C)]` and can be used to read directly
//! from a memory-mapped file on a little-endian machine.
//!
//! Not all values of these structs are valid for FDB files, but all well-formed
//! FDB-files can be represented by these values. Most importantly, the
//! [`FDBColumnHeader::column_data_type`] only has a limited amount of defined values but
//! covers the whole 32 bits.
use bytemuck_derive::{Pod, Zeroable};

use super::common::{Context, Value};

pub mod lists;

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The basic format of an array reference
pub struct ArrayHeader {
    /// The number of entries in the array
    pub count: u32,
    /// The offset of the start of the array
    pub base_offset: u32,
}

impl From<(u32, u32)> for ArrayHeader {
    fn from((count, base_offset): (u32, u32)) -> Self {
        Self { count, base_offset }
    }
}

/// The header of the database file.
///
/// This struct exists only once at index 0 of the file.
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
pub struct FDBHeader {
    /// The [`FDBTableHeader`] array.
    pub tables: ArrayHeader,
}

impl FDBHeader {
    #[inline]
    /// Returns the length in bytes of the TableHeader array.
    pub const fn table_headers_byte_count(&self) -> usize {
        self.tables.count as usize * std::mem::size_of::<FDBTableHeader>()
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header of a table.
///
/// This struct is used in the global TableHeader list and contains
/// the offsets of the two structures that define the definition and
/// content of the tables.
pub struct FDBTableHeader {
    /// The offset of this table definition header.
    pub table_def_header_addr: u32,
    /// The offset of the table data header.
    pub table_data_header_addr: u32,
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header of a table definition
///
/// This struct exists once per table and contains links to information
/// on the name of the table and the names and data types of the columns.
pub struct FDBTableDefHeader {
    /// The number of columns in this table.
    pub column_count: u32,
    /// The offset of the (null-terminated) name of this table
    pub table_name_addr: u32,
    /// The offset of the array of [`FDBColumnHeader`]s
    pub column_header_list_addr: u32,
}

impl From<(u32, u32, u32)> for FDBTableDefHeader {
    fn from((column_count, table_name_addr, column_header_list_addr): (u32, u32, u32)) -> Self {
        Self {
            column_count,
            table_name_addr,
            column_header_list_addr,
        }
    }
}

impl From<[u32; 3]> for FDBTableDefHeader {
    fn from([column_count, table_name_addr, column_header_list_addr]: [u32; 3]) -> Self {
        Self {
            column_count,
            table_name_addr,
            column_header_list_addr,
        }
    }
}

impl FDBTableDefHeader {
    #[inline]
    /// Returns the expected byte length of the referenced [`FDBColumnHeader`] array.
    pub const fn column_header_list_byte_count(&self) -> usize {
        self.column_count as usize * std::mem::size_of::<FDBColumnHeader>()
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header of a column (field-of-row) definition
///
/// This struct contains information on the name and data type of a column.
/// It is always an element of the array pointed to by the [`FDBTableDefHeader`].
pub struct FDBColumnHeader {
    /// The numeric identifier of the data type.
    pub column_data_type: u32,
    /// The offset of the (null-terminated) name.
    pub column_name_addr: u32,
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header of a table data block
///
/// It contains a reference to the array of buckets that hold the row data.
pub struct FDBTableDataHeader {
    /// The buckets.
    pub buckets: ArrayHeader,
}

impl FDBTableDataHeader {
    #[inline]
    /// Returns the expected byte length of the [`FDBBucketHeader`] array.
    pub const fn bucket_header_list_byte_count(&self) -> usize {
        self.buckets.count as usize * std::mem::size_of::<FDBBucketHeader>()
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header of a single bucket.
///
/// A bucket is a linked list of references to rows that all have the same
/// primary key hash.
pub struct FDBBucketHeader {
    /// Offset of the first element of the linked list or 0xffffffff.
    pub row_header_list_head_addr: u32,
}

impl From<u32> for FDBBucketHeader {
    fn from(row_header_list_head_addr: u32) -> Self {
        Self {
            row_header_list_head_addr,
        }
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// One entry of the linked list of references to rows.
///
/// This struct always contains a reference to a row and may
/// point to another entry in the linked list.
pub struct FDBRowHeaderListEntry {
    /// The offset of the row header.
    pub row_header_addr: u32,
    /// The offset of the next list entry or `0`.
    pub row_header_list_next_addr: u32,
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The header for a single row
pub struct FDBRowHeader {
    /// The fields in this row
    pub fields: ArrayHeader,
}

impl FDBRowHeader {
    #[inline]
    /// Returns the expected byte length of the [`FDBFieldData`] array.
    pub const fn field_data_list_byte_count(&self) -> usize {
        self.fields.count as usize * std::mem::size_of::<FDBFieldData>()
    }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
/// The type and value of a row field.
pub struct FDBFieldData {
    /// The data type.
    pub data_type: u32,
    /// The bytes that specify the value.
    pub value: [u8; 4],
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// The `common::Context` for used to make `file::FDBFieldValue`
pub struct FileContext;

impl Context for FileContext {
    type String = IndirectValue;
    type I64 = IndirectValue;
    type Bytes = IndirectValue;
}

/// An indirect value in the file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IndirectValue {
    /// The base of the value
    pub addr: u32,
}

/// A database field value repr
pub type FDBFieldValue = Value<FileContext>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_align() {
        assert_eq!(mem::align_of::<FDBHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableDefHeader>(), 4);
        assert_eq!(mem::align_of::<FDBColumnHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableDataHeader>(), 4);
        assert_eq!(mem::align_of::<FDBBucketHeader>(), 4);
        assert_eq!(mem::align_of::<FDBRowHeaderListEntry>(), 4);
        assert_eq!(mem::align_of::<FDBRowHeader>(), 4);
        assert_eq!(mem::align_of::<FDBFieldData>(), 4);
    }

    #[test]
    fn test_size_of() {
        assert_eq!(mem::size_of::<FDBHeader>(), 8);
        assert_eq!(mem::size_of::<FDBTableHeader>(), 8);
        assert_eq!(mem::size_of::<FDBTableDefHeader>(), 12);
        assert_eq!(mem::size_of::<FDBColumnHeader>(), 8);
        assert_eq!(mem::size_of::<FDBTableDataHeader>(), 8);
        assert_eq!(mem::size_of::<FDBBucketHeader>(), 4);
        assert_eq!(mem::size_of::<FDBRowHeaderListEntry>(), 8);
        assert_eq!(mem::size_of::<FDBRowHeader>(), 8);
        assert_eq!(mem::size_of::<FDBFieldData>(), 8);
    }
}
