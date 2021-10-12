//! # "Normal" aligned versions of the FDB structures

use crate::generic;

/// The header at the start of the file
pub type Header = generic::Header<u32, u32>;
/// The entry in the table array
pub type TableHeader = generic::Table<u32>;
/// The definition of the data
pub type TableDefHeader = generic::TableDef<u32, u32>;
/// The data of a column
pub type ColumnHeader = generic::Column<u32, u32>;
/// The content of a table
pub type TableDataHeader = generic::TableData<u32, u32>;
/// The entry in the bucket list
pub type BucketHeader = generic::BucketHeader<u32>;
/// One element in the linked-list of rows
pub type RowHeaderCons = generic::RowHeaderCons<u32>;
/// The data for a row
pub type RowHeader = generic::RowHeader<u32, u32>;
/// One entry in the list of fields
pub type FieldData = generic::FieldData<u32, [u8; 4]>;

impl Header {
    #[inline]
    /// Returns the length in bytes of the TableHeader array.
    pub const fn table_headers_byte_count(&self) -> usize {
        self.tables.length as usize * std::mem::size_of::<TableHeader>()
    }
}

impl TableDefHeader {
    #[inline]
    /// Returns the expected byte length of the referenced [`ColumnHeader`] array.
    pub const fn column_header_list_byte_count(&self) -> usize {
        self.column_count as usize * std::mem::size_of::<ColumnHeader>()
    }
}

impl TableDataHeader {
    #[inline]
    /// Returns the expected byte length of the [`BucketHeader`] array.
    pub const fn bucket_header_list_byte_count(&self) -> usize {
        self.buckets.length as usize * std::mem::size_of::<BucketHeader>()
    }
}

impl RowHeader {
    #[inline]
    /// Returns the expected byte length of the [`FieldData`] array.
    pub const fn field_data_list_byte_count(&self) -> usize {
        self.fields.length as usize * std::mem::size_of::<FieldData>()
    }
}
