use crate::fdb::file::{
    FDBBucketHeader, FDBColumnHeader, FDBHeader, FDBTableDataHeader, FDBTableDefHeader,
    FDBTableHeader,
};
use assembly_core::buffer::{Unaligned, LEU32};

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

impl FDBHeaderC {
    pub fn table_count(&self) -> u32 {
        self.table_count.extract()
    }

    pub fn table_header_list_addr(&self) -> u32 {
        self.table_header_list_addr.extract()
    }
}

unsafe impl Unaligned for FDBHeaderC {
    type Value = FDBHeader;
    fn extract(&self) -> Self::Value {
        FDBHeader {
            table_count: self.table_count.extract(),
            table_header_list_addr: self.table_header_list_addr.extract(),
        }
    }
}

unsafe impl Unaligned for FDBTableHeaderC {
    type Value = FDBTableHeader;
    fn extract(&self) -> Self::Value {
        FDBTableHeader {
            table_def_header_addr: self.table_def_header_addr.extract(),
            table_data_header_addr: self.table_data_header_addr.extract(),
        }
    }
}

unsafe impl Unaligned for FDBTableDefHeaderC {
    type Value = FDBTableDefHeader;
    fn extract(&self) -> Self::Value {
        FDBTableDefHeader {
            column_count: self.column_count.extract(),
            table_name_addr: self.table_name_addr.extract(),
            column_header_list_addr: self.column_header_list_addr.extract(),
        }
    }
}

unsafe impl Unaligned for FDBTableDataHeaderC {
    type Value = FDBTableDataHeader;
    fn extract(&self) -> Self::Value {
        FDBTableDataHeader {
            bucket_count: self.bucket_count.extract(),
            bucket_header_list_addr: self.bucket_header_list_addr.extract(),
        }
    }
}

unsafe impl Unaligned for FDBColumnHeaderC {
    type Value = FDBColumnHeader;
    fn extract(&self) -> Self::Value {
        FDBColumnHeader {
            column_data_type: self.column_data_type.extract(),
            column_name_addr: self.column_name_addr.extract(),
        }
    }
}

unsafe impl Unaligned for FDBBucketHeaderC {
    type Value = FDBBucketHeader;
    fn extract(&self) -> Self::Value {
        FDBBucketHeader {
            row_header_list_head_addr: self.row_header_list_head_addr.extract(),
        }
    }
}
