use crate::{error::ModuloMismatch, generic, FDBFieldValue};
use zerovec::ule::{AsULE, EqULE, PlainOldULE, ULE};

macro_rules! as_ule {
    ($ty:ty = $ule:ty) => {
        unsafe impl EqULE for $ty {}

        impl AsULE for $ty {
            type ULE = $ule;

            fn as_unaligned(&self) -> Self::ULE {
                (*self).into()
            }

            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                Self::from(*unaligned)
            }
        }
    };
}

impl AsULE for FDBFieldValue {
    type ULE = FieldValueULE;

    fn as_unaligned(&self) -> Self::ULE {
        FieldValueULE(self.0)
    }

    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        Self(unaligned.0)
    }
}

as_ule!(crate::aligned::Header = HeaderULE);
as_ule!(crate::aligned::TableHeader = TableHeaderULE);
as_ule!(crate::aligned::TableDefHeader = TableDefHeaderULE);
as_ule!(crate::aligned::ColumnHeader = ColumnHeaderULE);
as_ule!(crate::aligned::TableDataHeader = TableDataHeaderULE);
as_ule!(crate::aligned::BucketHeader = BucketHeaderULE);
as_ule!(crate::aligned::RowHeaderCons = RowHeaderConsULE);
as_ule!(crate::aligned::RowHeader = RowHeaderULE);
as_ule!(crate::aligned::FieldData = FieldDataULE);

macro_rules! ule_alias(
    ($ty:ty => $name:ident $ule:ident) => {
        #[cfg(feature = "pod")]
        use bytemuck_derive::{Pod, Zeroable};

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

impl Offset {
    pub fn usize(self) -> usize {
        self.0 as usize
    }
}

macro_rules! ule_alias(
    ($name:ident<$size:literal>) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub(super) [u8; $size]);

        impl From<PlainOldULE<$size>> for $name {
            fn from(ule: PlainOldULE<$size>) -> Self {
                Self(ule.0)
            }
        }

        impl From<&$name> for &PlainOldULE<$size> {
            fn from(ule: &$name) -> Self {
                unsafe { std::mem::transmute(ule) }
            }
        }
    };
);

ule_alias!(OffsetULE<4>);
ule_alias!(ULE32<4>);

impl From<u32> for OffsetULE {
    fn from(v: u32) -> Self {
        Self(v.to_le_bytes())
    }
}

impl From<OffsetULE> for u32 {
    fn from(v: OffsetULE) -> Self {
        u32::from_le_bytes(v.0)
    }
}

impl From<u32> for ULE32 {
    fn from(v: u32) -> Self {
        Self(v.to_le_bytes())
    }
}

impl From<ULE32> for u32 {
    fn from(v: ULE32) -> Self {
        u32::from_le_bytes(v.0)
    }
}

macro_rules! ule_impl {
    ($($ty:ty)*) => {$(
        impl ULE for $ty {
            type Error = ModuloMismatch;

            fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
                const SIZE: usize = std::mem::size_of::<$ty>();
                let base = bytes as *const [u8] as *const u8 as *const Self;
                let mod_len = bytes.len() % SIZE;
                if mod_len > 0 {
                    Err(ModuloMismatch {
                        input_len: bytes.len(),
                        type_size: SIZE,
                    })
                } else {
                    let len = bytes.len() / SIZE;
                    Ok(unsafe { std::slice::from_raw_parts(base, len) })
                }
            }

            fn as_byte_slice(slice: &[Self]) -> &[u8] {
                const SIZE: usize = std::mem::size_of::<$ty>();
                let base = slice as *const [Self] as *const Self as *const u8;
                unsafe { std::slice::from_raw_parts(base, slice.len() * SIZE) }
            }
        }
    )*};
}

ule_impl!(
    OffsetULE HeaderULE TableHeaderULE TableDefHeaderULE
    ColumnHeaderULE TableDataHeaderULE BucketHeaderULE RowHeaderULE
    RowHeaderConsULE FieldDataULE FieldValueULE
);

pub type HeaderULE = generic::Header<OffsetULE, ULE32>;
pub type TableHeaderULE = generic::Table<OffsetULE>;
pub type TableDefHeaderULE = generic::TableDef<OffsetULE, ULE32>;
pub type ColumnHeaderULE = generic::Column<OffsetULE, ULE32>;
pub type TableDataHeaderULE = generic::TableData<OffsetULE, ULE32>;
pub type BucketHeaderULE = generic::BucketHeader<OffsetULE>;
pub type RowHeaderConsULE = generic::RowHeaderCons<OffsetULE>;
pub type RowHeaderULE = generic::RowHeader<OffsetULE, ULE32>;
pub type FieldDataULE = generic::FieldData<ULE32, FieldValueULE>;

/// An FDB field value usable for unaligned reads
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug)]
pub struct FieldValueULE(pub(super) [u8; 4]);

impl From<[u8; 4]> for FieldValueULE {
    fn from(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }
}

impl From<FieldValueULE> for [u8; 4] {
    fn from(ule: FieldValueULE) -> Self {
        ule.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_align() {
        assert_eq!(mem::align_of::<HeaderULE>(), 1);
        assert_eq!(mem::align_of::<TableHeaderULE>(), 1);
        assert_eq!(mem::align_of::<TableDefHeaderULE>(), 1);
        assert_eq!(mem::align_of::<ColumnHeaderULE>(), 1);
        assert_eq!(mem::align_of::<TableDataHeaderULE>(), 1);
        assert_eq!(mem::align_of::<BucketHeaderULE>(), 1);
        assert_eq!(mem::align_of::<RowHeaderConsULE>(), 1);
        assert_eq!(mem::align_of::<RowHeaderULE>(), 1);
        assert_eq!(mem::align_of::<FieldDataULE>(), 1);
    }

    #[test]
    fn test_size_of() {
        assert_eq!(mem::size_of::<HeaderULE>(), 8);
        assert_eq!(mem::size_of::<TableHeaderULE>(), 8);
        assert_eq!(mem::size_of::<TableDefHeaderULE>(), 12);
        assert_eq!(mem::size_of::<ColumnHeaderULE>(), 8);
        assert_eq!(mem::size_of::<TableDataHeaderULE>(), 8);
        assert_eq!(mem::size_of::<BucketHeaderULE>(), 4);
        assert_eq!(mem::size_of::<RowHeaderConsULE>(), 8);
        assert_eq!(mem::size_of::<RowHeaderULE>(), 8);
        assert_eq!(mem::size_of::<FieldDataULE>(), 8);
    }
}
