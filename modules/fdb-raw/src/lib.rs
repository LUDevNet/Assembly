#![allow(clippy::upper_case_acronyms)]
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

#[cfg(feature = "pod")]
use bytemuck_derive::{Pod, Zeroable};
use ule::{
    ArrayHeaderULE, BucketHeaderULE, ColumnHeaderULE, FieldDataULE, FieldValueULE, HeaderULE,
    OffsetULE, RowHeaderConsULE, RowHeaderULE, TableDataHeaderULE, TableDefHeaderULE,
    TableHeaderULE,
};
#[cfg(feature = "zero")]
use zerovec::ule::AsULE;
#[cfg(feature = "zero")]
pub mod ule;

macro_rules! as_ule {
    ($(#[$m1:meta])*
        $ty:ident {
        $(
            $(#[$m2:meta])*
            $f:ident: $v:ty),* $(,)?
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "pod", derive(Pod, Zeroable))]
        #[repr(C)]
        $(#[$m1])*
        pub struct $ty {

            $($(#[$m2])* pub $f: $v),*
        }
    };
    ($(#[$m1:meta])*
        $ty:ident as $ule:ty {
        $(
            $(#[$m2:meta])*
            $f:ident: $v:ty),* $(,)?
    }) => {
        as_ule!($(#[$m1])* $ty {
            $(
            $(#[$m2])*
            $f: $v
            ),*
        });

        impl AsULE for $ty {
            type ULE = $ule;

            fn as_unaligned(&self) -> Self::ULE {
                Self::ULE {$($f: self.$f.as_unaligned(),)*}
            }

            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                Self {$(
                    $f: <$v>::from_unaligned(&unaligned.$f),
                )*}
            }
        }
    };
}

macro_rules! from_impl {
    ($struct:ty => ($($key:ident: $ty:ty),*)) => {
        impl From<($($ty),*)> for $struct {
            fn from(($($key),*): ($($ty),*)) -> Self {
                Self { $($key),* }
            }
        }
    };
    ($struct:ty => [$($key:ident),*]: [$ty:ty; $c:literal]) => {
        impl From<[$ty; $c]> for $struct {
            fn from([$($key),*]: [$ty; $c]) -> Self {
                Self { $($key),* }
            }
        }
    };
    ($struct:ty => $key:ident: $ty:ty) => {
        impl From<$ty> for $struct {
            fn from($key: $ty) -> Self {
                Self { $key }
            }
        }
    };
}

macro_rules! ule_alias(
    ($ty:ty => $name:ident $ule:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "pod", derive(Pod, Zeroable))]
        #[repr(C)]
        pub struct $name(pub $ty);

        impl AsULE for $name {
            type ULE = $ule;

            fn as_unaligned(&self) -> Self::ULE {
                self.0.as_unaligned().into()
            }

            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                Self(<$ty>::from_unaligned(unaligned.into()))
            }
        }
    }
);

ule_alias!(u32 => Offset OffsetULE);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "pod", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FDBFieldValue(pub [u8; 4]);

impl AsULE for FDBFieldValue {
    type ULE = FieldValueULE;

    fn as_unaligned(&self) -> Self::ULE {
        FieldValueULE(self.0)
    }

    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        Self(unaligned.0)
    }
}

as_ule!(
    /// The basic format of an array reference
    ArrayHeader as ArrayHeaderULE {
        /// The number of entries in the array
        count: u32,
        /// The offset of the start of the array
        base_offset: Offset,
    }
);

from_impl!(ArrayHeader => (count: u32, base_offset: Offset));

as_ule!(
    /// The header of the database file.
    ///
    /// This struct exists only once at index 0 of the file.
    FDBHeader as HeaderULE {
        /// The [`FDBTableHeader`] array.
        tables: ArrayHeader,
    }
);

impl FDBHeader {
    #[inline]
    /// Returns the length in bytes of the TableHeader array.
    pub const fn table_headers_byte_count(&self) -> usize {
        self.tables.count as usize * std::mem::size_of::<FDBTableHeader>()
    }
}

as_ule!(
/// The header of a table.
///
/// This struct is used in the global TableHeader list and contains
/// the offsets of the two structures that define the definition and
/// content of the tables.
FDBTableHeader as TableHeaderULE {
    /// The offset of this table definition header.
    table_def_header_addr: u32,
    /// The offset of the table data header.
    table_data_header_addr: u32,
});

as_ule!(
/// The header of a table definition
///
/// This struct exists once per table and contains links to information
/// on the name of the table and the names and data types of the columns.
FDBTableDefHeader as TableDefHeaderULE {
    /// The number of columns in this table.
    column_count: u32,
    /// The offset of the (null-terminated) name of this table
    table_name_addr: Offset,
    /// The offset of the array of [`FDBColumnHeader`]s
    column_header_list_addr: Offset,
});

from_impl!(FDBTableDefHeader => (column_count: u32, table_name_addr: Offset, column_header_list_addr: Offset));

impl FDBTableDefHeader {
    #[inline]
    /// Returns the expected byte length of the referenced [`FDBColumnHeader`] array.
    pub const fn column_header_list_byte_count(&self) -> usize {
        self.column_count as usize * std::mem::size_of::<FDBColumnHeader>()
    }
}

as_ule!(
/// The header of a column (field-of-row) definition
///
/// This struct contains information on the name and data type of a column.
/// It is always an element of the array pointed to by the [`FDBTableDefHeader`].
FDBColumnHeader as ColumnHeaderULE {
    /// The numeric identifier of the data type.
    column_data_type: u32,
    /// The offset of the (null-terminated) name.
    column_name_addr: Offset,
});

as_ule!(
/// The header of a table data block
///
/// It contains a reference to the array of buckets that hold the row data.
FDBTableDataHeader as TableDataHeaderULE {
    /// The buckets.
    buckets: ArrayHeader,
});

impl FDBTableDataHeader {
    #[inline]
    /// Returns the expected byte length of the [`FDBBucketHeader`] array.
    pub const fn bucket_header_list_byte_count(&self) -> usize {
        self.buckets.count as usize * std::mem::size_of::<FDBBucketHeader>()
    }
}

as_ule!(
/// The header of a single bucket.
///
/// A bucket is a linked list of references to rows that all have the same
/// primary key hash.
FDBBucketHeader  as BucketHeaderULE {
    /// Offset of the first element of the linked list or 0xffffffff.
    row_header_list_head_addr: u32,
});

from_impl!(FDBBucketHeader => row_header_list_head_addr: u32);

as_ule!(
/// One entry of the linked list of references to rows.
///
/// This struct always contains a reference to a row and may
/// point to another entry in the linked list.
FDBRowHeaderCons as RowHeaderConsULE {
    /// The offset of the row header.
    first: Offset,
    /// The offset of the next list entry or `0`.
    rest: Offset,
});

as_ule!(
/// The header for a single row
FDBRowHeader as RowHeaderULE {
    /// The fields in this row
    fields: ArrayHeader,
});

impl FDBRowHeader {
    #[inline]
    /// Returns the expected byte length of the [`FDBFieldData`] array.
    pub const fn field_data_list_byte_count(&self) -> usize {
        self.fields.count as usize * std::mem::size_of::<FDBFieldData>()
    }
}

as_ule!(
/// The type and value of a row field.
FDBFieldData as FieldDataULE {
    /// The data type.
    data_type: u32,
    /// The bytes that specify the value.
    value: FDBFieldValue,
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    #[cfg(feature = "zero")]
    fn test_zerovec() {
        use zerovec::ZeroVec;

        let v = ZeroVec::<FDBTableHeader>::try_from_bytes(&[10, 0, 0, 0, 20, 0, 0, 0]).unwrap();
        assert!(v.len() == 1);
        assert_eq!(
            v.get(0),
            Some(FDBTableHeader {
                table_def_header_addr: 10,
                table_data_header_addr: 20,
            })
        );
    }

    #[test]
    fn test_align() {
        assert_eq!(mem::align_of::<FDBHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableDefHeader>(), 4);
        assert_eq!(mem::align_of::<FDBColumnHeader>(), 4);
        assert_eq!(mem::align_of::<FDBTableDataHeader>(), 4);
        assert_eq!(mem::align_of::<FDBBucketHeader>(), 4);
        assert_eq!(mem::align_of::<FDBRowHeaderCons>(), 4);
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
        assert_eq!(mem::size_of::<FDBRowHeaderCons>(), 8);
        assert_eq!(mem::size_of::<FDBRowHeader>(), 8);
        assert_eq!(mem::size_of::<FDBFieldData>(), 8);
    }
}
