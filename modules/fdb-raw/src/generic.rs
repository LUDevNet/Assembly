//! # Generic structures
//!
//! This module contains
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// A address+length combination
pub struct Array<Addr, Len> {
    /// The base offset of the array (offset of the first item)
    pub base: Addr,
    /// The number of entries in the array
    pub length: Len,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The header at the start of the file
pub struct Header<Addr, Len> {
    /// The list of tables in the database
    ///
    /// This list should be sorted by the name of the table
    pub tables: Array<Addr, Len>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// One entry in the tables list
pub struct Table<Addr> {
    /// The offset of this table definition header.
    pub def_header: Addr,
    /// The offset of the table data header.
    pub data_header: Addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The definition of a table
pub struct TableDef<Addr, Len> {
    /// The number of columns in this table.
    pub column_count: Len,
    /// The offset of the (null-terminated) name of this table
    pub table_name: Addr,
    /// The offset of the array of [`Column`]s
    pub column_list: Addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// One entry in a columns list
pub struct Column<Addr, Ty> {
    /// The numeric identifier of the data type.
    pub data_type: Ty,
    /// The offset of the (null-terminated) name.
    pub name: Addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The content of a table
pub struct TableData<Addr, Len> {
    /// The buckets.
    pub buckets: Array<Addr, Len>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// One entry of the bucket list
pub struct BucketHeader<Addr> {
    /// Offset of the first element of the linked list or 0xffffffff.
    pub head: Addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// One element of the linked list of rows
pub struct RowHeaderCons<Addr> {
    /// The offset of the row header.
    pub first: Addr,
    /// The offset of the next list entry or `0`.
    pub rest: Addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The data for a single row
pub struct RowHeader<Addr, Len> {
    /// The fields in this row
    pub fields: Array<Addr, Len>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// One entry in the fields list
pub struct FieldData<Ty, Val> {
    /// The data type.
    pub data_type: Ty,
    /// The bytes that specify the value.
    pub value: Val,
}
