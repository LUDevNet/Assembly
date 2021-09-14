use zerovec::ule::{PlainOldULE, ULE};

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

type ULE32 = PlainOldULE<4>;
ule_alias!(OffsetULE<4>);

#[derive(Debug)]
pub struct ModuloMismatch(usize);

macro_rules! ule_impl {
    ($($ty:ty)*) => {$(
        impl ULE for $ty {
            type Error = ModuloMismatch;

            fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
                const SIZE: usize = std::mem::size_of::<$ty>();
                let base = bytes as *const [u8] as *const u8 as *const Self;
                let mod_len = bytes.len() % SIZE;
                if mod_len > 0 {
                    Err(ModuloMismatch(mod_len))
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
    OffsetULE ArrayHeaderULE HeaderULE TableHeaderULE TableDefHeaderULE
    ColumnHeaderULE TableDataHeaderULE BucketHeaderULE RowHeaderULE
    RowHeaderConsULE FieldDataULE FieldValueULE
);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ArrayHeaderULE {
    /// The number of entries in the array
    pub(super) count: ULE32,
    /// The offset of the start of the array
    pub(super) base_offset: OffsetULE,
}

/// An FDB header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HeaderULE {
    pub(super) tables: ArrayHeaderULE,
}

/// An FDB table header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TableHeaderULE {
    pub(super) table_def_header_addr: ULE32,
    pub(super) table_data_header_addr: ULE32,
}

/// An FDB table definition header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TableDefHeaderULE {
    pub(super) column_count: ULE32,
    pub(super) table_name_addr: OffsetULE,
    pub(super) column_header_list_addr: OffsetULE,
}

/// An FDB column header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColumnHeaderULE {
    pub(super) column_data_type: ULE32,
    pub(super) column_name_addr: OffsetULE,
}

/// An FDB table data header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TableDataHeaderULE {
    pub(super) buckets: ArrayHeaderULE,
}

/// An FDB bucket header usable for unaligned reads
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BucketHeaderULE {
    pub(super) row_header_list_head_addr: ULE32,
}

/// An FDB row header list entry usable for unaligned reads
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug)]
pub struct RowHeaderConsULE {
    pub(super) first: OffsetULE,
    pub(super) rest: OffsetULE,
}

/// An FDB row header usable for unaligned reads
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug)]
pub struct RowHeaderULE {
    pub(super) fields: ArrayHeaderULE,
}

/// An FDB field value usable for unaligned reads
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug)]
pub struct FieldValueULE(pub(super) [u8; 4]);

/// An FDB field data usable for unaligned reads
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug)]
pub struct FieldDataULE {
    pub(super) data_type: ULE32,
    pub(super) value: FieldValueULE,
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
