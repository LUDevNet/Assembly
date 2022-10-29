#![allow(clippy::upper_case_acronyms)]
use std::convert::TryFrom;

use crate::common::ValueType;
use assembly_core::buffer::{MinimallyAligned, Repr, LEU32};
use assembly_fdb_core::{
    file::{
        ArrayHeader, FDBBucketHeader, FDBColumnHeader, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
    value::file::{FDBFieldValue, IndirectValue},
};

/// An FDB header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBHeaderC {
    pub(super) table_count: LEU32,
    pub(super) table_header_list_addr: LEU32,
}

/// An FDB table header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBTableHeaderC {
    pub(super) table_def_header_addr: LEU32,
    pub(super) table_data_header_addr: LEU32,
}

/// An FDB table definition header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBTableDefHeaderC {
    pub(super) column_count: LEU32,
    pub(super) table_name_addr: LEU32,
    pub(super) column_header_list_addr: LEU32,
}

/// An FDB column header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBColumnHeaderC {
    pub(super) column_data_type: LEU32,
    pub(super) column_name_addr: LEU32,
}

/// An FDB table data header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBTableDataHeaderC {
    pub(super) bucket_count: LEU32,
    pub(super) bucket_header_list_addr: LEU32,
}

/// An FDB bucket header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBBucketHeaderC {
    pub(super) row_header_list_head_addr: LEU32,
}

/// An FDB row header list entry usable for unaligned reads
#[repr(C, align(1))]
#[derive(Debug)]
pub struct FDBRowHeaderListEntryC {
    pub(super) row_header_addr: LEU32,
    pub(super) row_header_list_next_addr: LEU32,
}

/// An FDB row header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBRowHeaderC {
    pub(super) field_count: LEU32,
    pub(super) field_data_list_addr: LEU32,
}

/// An FDB field value usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBFieldValueC(pub(super) [u8; 4]);

/// An FDB field data usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBFieldDataC {
    pub(super) data_type: LEU32,
    pub(super) value: FDBFieldValueC,
}

unsafe impl MinimallyAligned for FDBHeaderC {}

impl Repr for FDBHeaderC {
    type Value = FDBHeader;
    fn extract(&self) -> Self::Value {
        FDBHeader {
            tables: ArrayHeader {
                count: self.table_count.extract(),
                base_offset: self.table_header_list_addr.extract(),
            },
        }
    }
}

unsafe impl MinimallyAligned for FDBTableHeaderC {}

impl Repr for FDBTableHeaderC {
    type Value = FDBTableHeader;
    fn extract(&self) -> Self::Value {
        FDBTableHeader {
            table_def_header_addr: self.table_def_header_addr.extract(),
            table_data_header_addr: self.table_data_header_addr.extract(),
        }
    }
}

unsafe impl MinimallyAligned for FDBTableDefHeaderC {}

impl Repr for FDBTableDefHeaderC {
    type Value = FDBTableDefHeader;
    fn extract(&self) -> Self::Value {
        FDBTableDefHeader {
            column_count: self.column_count.extract(),
            table_name_addr: self.table_name_addr.extract(),
            column_header_list_addr: self.column_header_list_addr.extract(),
        }
    }
}

unsafe impl MinimallyAligned for FDBTableDataHeaderC {}

impl Repr for FDBTableDataHeaderC {
    type Value = FDBTableDataHeader;
    fn extract(&self) -> Self::Value {
        FDBTableDataHeader {
            buckets: ArrayHeader {
                count: self.bucket_count.extract(),
                base_offset: self.bucket_header_list_addr.extract(),
            },
        }
    }
}

unsafe impl MinimallyAligned for FDBColumnHeaderC {}

impl Repr for FDBColumnHeaderC {
    type Value = FDBColumnHeader;
    fn extract(&self) -> Self::Value {
        FDBColumnHeader {
            column_data_type: self.column_data_type.extract(),
            column_name_addr: self.column_name_addr.extract(),
        }
    }
}

unsafe impl MinimallyAligned for FDBBucketHeaderC {}

impl Repr for FDBBucketHeaderC {
    type Value = FDBBucketHeader;
    fn extract(&self) -> Self::Value {
        FDBBucketHeader {
            row_header_list_head_addr: self.row_header_list_head_addr.extract(),
        }
    }
}

unsafe impl MinimallyAligned for FDBRowHeaderListEntryC {}

impl Repr for FDBRowHeaderListEntryC {
    type Value = FDBRowHeaderListEntry;
    fn extract(&self) -> Self::Value {
        FDBRowHeaderListEntry {
            row_header_addr: self.row_header_addr.extract(),
            row_header_list_next_addr: self.row_header_list_next_addr.extract(),
        }
    }
}

unsafe impl MinimallyAligned for FDBRowHeaderC {}

impl Repr for FDBRowHeaderC {
    type Value = FDBRowHeader;
    fn extract(&self) -> Self::Value {
        FDBRowHeader {
            fields: ArrayHeader {
                count: self.field_count.extract(),
                base_offset: self.field_data_list_addr.extract(),
            },
        }
    }
}

unsafe impl MinimallyAligned for FDBFieldDataC {}

impl Repr for FDBFieldDataC {
    type Value = FDBFieldValue;
    fn extract(&self) -> Self::Value {
        // FIXME: Remove unwrap
        let data_type = ValueType::try_from(self.data_type.extract()).unwrap();
        match data_type {
            ValueType::Nothing => FDBFieldValue::Nothing,
            ValueType::Integer => FDBFieldValue::Integer(i32::from_le_bytes(self.value.0)),
            ValueType::Float => FDBFieldValue::Float(f32::from_le_bytes(self.value.0)),
            ValueType::Text => FDBFieldValue::Text(IndirectValue {
                addr: u32::from_le_bytes(self.value.0),
            }),
            ValueType::Boolean => FDBFieldValue::Boolean(self.value.0 != [0, 0, 0, 0]),
            ValueType::BigInt => FDBFieldValue::BigInt(IndirectValue {
                addr: u32::from_le_bytes(self.value.0),
            }),
            ValueType::VarChar => FDBFieldValue::VarChar(IndirectValue {
                addr: u32::from_le_bytes(self.value.0),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_align_of() {
        assert_eq!(1, std::mem::align_of::<FDBHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBTableHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBTableDefHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBTableDataHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBColumnHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBBucketHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBRowHeaderListEntryC>());
        assert_eq!(1, std::mem::align_of::<FDBRowHeaderC>());
        assert_eq!(1, std::mem::align_of::<FDBFieldDataC>());
        assert_eq!(1, std::mem::align_of::<FDBFieldValueC>());
    }

    #[test]
    fn check_unaligned_read() {
        let test: &[u32] = &[0x00000000, 0x00000800, 0x00000000];
        let buffer: &[u8] = unsafe { std::slice::from_raw_parts(test.as_ptr() as *const u8, 12) };

        // Is this actually unaligned?
        let base = unsafe { buffer.as_ptr().offset(1) };
        assert_eq!(1, base as usize % 4);

        // Can we successfully get the value?
        let header = unsafe { &*(base as *const FDBHeaderC) };
        assert_eq!(
            header.extract(),
            FDBHeader {
                tables: ArrayHeader {
                    count: 0,
                    base_offset: 8,
                }
            }
        );
    }
}
