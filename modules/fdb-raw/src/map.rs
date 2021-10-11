#[cfg(feature = "zero")]
use crate::zero::{FieldValueULE, OffsetULE, ULE32};

#[cfg(any(feature = "bcast", feature = "zero"))]
use crate::generic::{
    Array, BucketHeader, Column, FieldData, Header, RowHeader, RowHeaderCons, Table, TableData,
    TableDef,
};
#[cfg(feature = "bcast")]
use bytes_cast::unaligned::U32Le;

#[cfg(any(feature = "bcast", feature = "zero"))]
macro_rules! convert {
    ($base:ident: $($t1:ty = $t2:ty),+: $($k:ident),+) => {
        impl From<$base<$($t1),+>> for $base<$($t2),+> {
            fn from(h: $base<$($t1),+>) -> Self {
                Self {
                    $($k: h.$k.into(),)+
                }
            }
        }

        impl From<$base<$($t2),+>> for $base<$($t1),+> {
            fn from(h: $base<$($t2),+>) -> Self {
                Self {
                    $($k: h.$k.into(),)+
                }
            }
        }
    };
}

#[cfg(any(feature = "bcast", feature = "zero"))]
macro_rules! conversions {
    ($addr:ty, $len:ty, $ty:ty, $val:ty) => {
        convert!(Array: u32 = $addr, u32 = $len: base, length);
        convert!(Header: u32 = $addr, u32 = $len: tables);
        convert!(Table: u32 = $addr: def_header, data_header);
        convert!(
            TableDef: u32 = $addr,
            u32 = $len: column_count,
            table_name,
            column_list
        );
        convert!(Column: u32 = $addr, u32 = $ty: data_type, name);
        convert!(TableData: u32 = $addr, u32 = $len: buckets);
        convert!(BucketHeader: u32 = $addr: head);
        convert!(RowHeaderCons: u32 = $addr: first, rest);
        convert!(RowHeader: u32 = $addr, u32 = $len: fields);
        convert!(FieldData: u32 = $ty, [u8; 4] = $val: data_type, value);
    };
}

#[cfg(feature = "bcast")]
conversions!(U32Le, U32Le, U32Le, [u8; 4]);
#[cfg(feature = "zero")]
conversions!(OffsetULE, ULE32, ULE32, FieldValueULE);
