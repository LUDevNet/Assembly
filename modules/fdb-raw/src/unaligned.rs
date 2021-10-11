use crate::generic;
use bytes_cast::unaligned::U32Le;

pub type HeaderULE = generic::Header<U32Le, U32Le>;
pub type TableHeaderULE = generic::Table<U32Le>;
pub type TableDefHeaderULE = generic::TableDef<U32Le, U32Le>;
pub type ColumnHeaderULE = generic::Column<U32Le, U32Le>;
pub type TableDataHeaderULE = generic::TableData<U32Le, U32Le>;
pub type BucketHeaderULE = generic::BucketHeader<U32Le>;
pub type RowHeaderConsULE = generic::RowHeaderCons<U32Le>;
pub type RowHeaderULE = generic::RowHeader<U32Le, U32Le>;
pub type FieldDataULE = generic::FieldData<U32Le, [u8; 4]>;
