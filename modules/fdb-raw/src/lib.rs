#![warn(missing_docs)]
//! The structures, as they are serialized
//!
//! This module contains the low-level structs that make up the FDB file. These
//! structures are annotated with `#[repr(C)]` and can be used to read directly
//! from a memory-mapped file on a little-endian machine.
//!
//! Not all values of these structs are valid for FDB files, but all well-formed
//! FDB-files can be represented by these values. Most importantly, the
//! [`generic::Column::data_type`] only has a limited amount of defined values but
//! covers the whole 32 bits.

#![doc(html_logo_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![doc(html_favicon_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]

pub mod aligned;
#[cfg(feature = "bcast")]
pub mod bcast;
pub mod error;
pub mod generic;
mod map;
#[cfg(feature = "zero")]
pub mod zero;

#[cfg(feature = "pod")]
use bytemuck_derive::{Pod, Zeroable};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "pod", derive(Pod, Zeroable))]
#[repr(C)]
/// The value of a single field
pub struct FieldValue(pub [u8; 4]);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "pod", derive(Pod, Zeroable))]
#[repr(C)]
/// A 32-bit offset into a file
pub struct Offset(pub u32);

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::aligned;

    #[test]
    #[cfg(feature = "zero")]
    fn test_zerovec() {
        use zerovec::ZeroVec;

        use crate::aligned::TableHeader;

        let v = ZeroVec::<TableHeader>::try_from_bytes(&[10, 0, 0, 0, 20, 0, 0, 0]).unwrap();
        assert!(v.len() == 1);
        assert_eq!(
            v.get(0),
            Some(TableHeader {
                def_header: 10,
                data_header: 20,
            })
        );
    }

    #[test]
    fn test_align() {
        assert_eq!(mem::align_of::<aligned::Header>(), 4);
        assert_eq!(mem::align_of::<aligned::TableHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::TableDefHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::ColumnHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::TableDataHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::BucketHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::RowHeaderCons>(), 4);
        assert_eq!(mem::align_of::<aligned::RowHeader>(), 4);
        assert_eq!(mem::align_of::<aligned::FieldData>(), 4);
    }

    #[test]
    fn test_size_of() {
        assert_eq!(mem::size_of::<aligned::Header>(), 8);
        assert_eq!(mem::size_of::<aligned::TableHeader>(), 8);
        assert_eq!(mem::size_of::<aligned::TableDefHeader>(), 12);
        assert_eq!(mem::size_of::<aligned::ColumnHeader>(), 8);
        assert_eq!(mem::size_of::<aligned::TableDataHeader>(), 8);
        assert_eq!(mem::size_of::<aligned::BucketHeader>(), 4);
        assert_eq!(mem::size_of::<aligned::RowHeaderCons>(), 8);
        assert_eq!(mem::size_of::<aligned::RowHeader>(), 8);
        assert_eq!(mem::size_of::<aligned::FieldData>(), 8);
    }
}
