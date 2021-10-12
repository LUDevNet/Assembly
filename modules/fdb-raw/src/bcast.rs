//! # Implementations for the [`bytes_cast`] crate
//!
//! [`bytes_cast`]: https://docs.rs/bytes-cast

use crate::generic;
use bytes_cast::unaligned::U32Le;

/// The header of the file
pub type Header = generic::Header<U32Le, U32Le>;
/// One entry in the [`Header::tables`] array
pub type TableHeader = generic::Table<U32Le>;
/// The definition of a table
pub type TableDefHeader = generic::TableDef<U32Le, U32Le>;
/// One entry in the [`TableDefHeader::column_list`] array
pub type ColumnHeader = generic::Column<U32Le, U32Le>;
/// The contents of a table
pub type TableDataHeader = generic::TableData<U32Le, U32Le>;
/// One entry in the [`TableDataHeader::buckets`]
pub type BucketHeader = generic::BucketHeader<U32Le>;
/// One entry in the linked list of rows in a [`BucketHeader`]
pub type RowHeaderCons = generic::RowHeaderCons<U32Le>;
/// The data for a single row
pub type RowHeader = generic::RowHeader<U32Le, U32Le>;
/// One entry in the [`RowHeader::fields`] array
pub type FieldData = generic::FieldData<U32Le, [u8; 4]>;
