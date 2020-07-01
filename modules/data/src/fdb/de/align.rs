use crate::fdb::file::FDBHeader;
use assembly_core::buffer::{Unaligned, LEU32};

/// An FDB Header usable for unaligned reads
#[repr(C, align(1))]
pub struct FDBHeaderC {
    table_count: LEU32,
    table_header_list_addr: LEU32,
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
