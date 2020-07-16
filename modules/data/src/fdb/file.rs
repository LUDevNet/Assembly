//! The structures, as they are serialized

#[derive(Debug)]
pub struct FDBHeader {
    pub table_count: u32,
    pub table_header_list_addr: u32,
}

impl FDBHeader {
    pub const BYTE_COUNT: usize = 8;

    #[inline]
    pub fn table_headers_byte_count(&self) -> usize {
        self.table_count as usize * FDBTableHeader::BYTE_COUNT
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FDBTableHeader {
    pub table_def_header_addr: u32,
    pub table_data_header_addr: u32,
}

impl FDBTableHeader {
    pub const BYTE_COUNT: usize = 8;
}

#[derive(Debug)]
pub struct FDBTableDefHeader {
    pub column_count: u32,
    pub table_name_addr: u32,
    pub column_header_list_addr: u32,
}

impl FDBTableDefHeader {
    pub const BYTE_COUNT: usize = 12;

    #[inline]
    pub fn column_header_list_byte_count(&self) -> usize {
        self.column_count as usize * FDBColumnHeader::BYTE_COUNT
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FDBColumnHeader {
    pub column_data_type: u32,
    pub column_name_addr: u32,
}

impl FDBColumnHeader {
    pub const BYTE_COUNT: usize = 8;
}

#[derive(Debug)]
pub struct FDBTableDataHeader {
    pub bucket_count: u32,
    pub bucket_header_list_addr: u32,
}

impl FDBTableDataHeader {
    pub const BYTE_COUNT: usize = 8;

    #[inline]
    pub fn bucket_header_list_byte_count(&self) -> usize {
        self.bucket_count as usize * FDBBucketHeader::BYTE_COUNT
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FDBBucketHeader {
    pub row_header_list_head_addr: u32,
}

impl FDBBucketHeader {
    pub const BYTE_COUNT: usize = 8;
}

#[derive(Debug)]
pub struct FDBRowHeaderListEntry {
    pub row_header_addr: u32,
    pub row_header_list_next_addr: u32,
}

impl FDBRowHeaderListEntry {
    pub const BYTE_COUNT: usize = 8;
}

#[derive(Debug)]
pub struct FDBRowHeader {
    pub field_count: u32,
    pub field_data_list_addr: u32,
}

impl FDBRowHeader {
    pub const BYTE_COUNT: usize = 8;

    #[inline]
    pub fn field_data_list_byte_count(&self) -> usize {
        self.field_count as usize * FDBFieldData::BYTE_COUNT
    }
}

#[derive(Debug)]
pub struct FDBFieldData {
    pub data_type: u32,
    pub value: [u8; 4],
}

impl FDBFieldData {
    pub const BYTE_COUNT: usize = 8;
}

/// A database field value repr
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FDBFieldValue {
    Nothing,
    Integer(i32),
    Float(f32),
    Text { addr: u32 },
    Boolean(bool),
    BigInt { addr: u32 },
    VarChar { addr: u32 },
}

#[derive(Debug)]
pub struct FDBTableHeaderList(pub Vec<FDBTableHeader>);

#[derive(Debug)]
pub struct FDBColumnHeaderList(pub Vec<FDBColumnHeader>);

#[derive(Debug)]
pub struct FDBBucketHeaderList(pub Vec<FDBBucketHeader>);

#[derive(Debug)]
pub struct FDBRowHeaderList(pub Vec<FDBRowHeader>);

#[derive(Debug)]
pub struct FDBFieldDataList(pub Vec<FDBFieldData>);

// Implementations FDBTableHeaderList
impl Into<Vec<FDBTableHeader>> for FDBTableHeaderList {
    fn into(self) -> Vec<FDBTableHeader> {
        self.0
    }
}

impl From<Vec<FDBTableHeader>> for FDBTableHeaderList {
    fn from(vec: Vec<FDBTableHeader>) -> Self {
        FDBTableHeaderList(vec)
    }
}

// Implementations FDBColumnHeaderList
impl Into<Vec<FDBColumnHeader>> for FDBColumnHeaderList {
    fn into(self) -> Vec<FDBColumnHeader> {
        self.0
    }
}

impl From<Vec<FDBColumnHeader>> for FDBColumnHeaderList {
    fn from(vec: Vec<FDBColumnHeader>) -> Self {
        FDBColumnHeaderList(vec)
    }
}

// Implementations FDBBucketHeaderList
impl Into<Vec<FDBBucketHeader>> for FDBBucketHeaderList {
    fn into(self) -> Vec<FDBBucketHeader> {
        self.0
    }
}

impl From<Vec<FDBBucketHeader>> for FDBBucketHeaderList {
    fn from(vec: Vec<FDBBucketHeader>) -> Self {
        FDBBucketHeaderList(vec)
    }
}

impl IntoIterator for FDBBucketHeaderList {
    type Item = FDBBucketHeader;
    type IntoIter = std::vec::IntoIter<FDBBucketHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Implementations FDBRowHeaderList
impl Into<Vec<FDBRowHeader>> for FDBRowHeaderList {
    fn into(self) -> Vec<FDBRowHeader> {
        self.0
    }
}

impl From<Vec<FDBRowHeader>> for FDBRowHeaderList {
    fn from(vec: Vec<FDBRowHeader>) -> Self {
        FDBRowHeaderList(vec)
    }
}

impl IntoIterator for FDBRowHeaderList {
    type Item = FDBRowHeader;
    type IntoIter = std::vec::IntoIter<FDBRowHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Implementations FDBFieldDataList
impl Into<Vec<FDBFieldData>> for FDBFieldDataList {
    fn into(self) -> Vec<FDBFieldData> {
        self.0
    }
}

impl From<Vec<FDBFieldData>> for FDBFieldDataList {
    fn from(vec: Vec<FDBFieldData>) -> Self {
        FDBFieldDataList(vec)
    }
}

impl IntoIterator for FDBFieldDataList {
    type Item = FDBFieldData;
    type IntoIter = std::vec::IntoIter<FDBFieldData>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_align() {
        assert_eq!(mem::align_of::<FDBHeader>(), 4);
    }
}
