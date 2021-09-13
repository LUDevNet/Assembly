//! The low-level Handle API
//!
//! This API uses handles that store the data of one header alongside
//! a reference into the in-memory file.

use super::{
    buffer::{self, cmp_table_header_name, Buffer, BufferError, Res},
    slice::{FDBBucketHeaderSlice, FDBColumnHeaderSlice, FDBFieldDataSlice, FDBTableHeaderSlice},
    BaseHandle, Handle,
};
use crate::fdb::{
    common::{Latin1Str, UnknownValueType, ValueType},
    file::{
        FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBFieldValue, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
};
use assembly_core::displaydoc::Display;
use std::{
    borrow::Cow, convert::TryFrom, error::Error, fmt, ops::Deref, result::Result as StdResult,
};

/// Custom result type for this module
pub type Result<'a, T> = std::result::Result<Handle<'a, T>, BufferError>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The basic error type
pub struct BaseError<P: Deref>
where
    P::Target: AsRef<[u8]>,
{
    mem: P,
    kind: BaseErrorKind,
}

impl<P: Deref + fmt::Debug> Error for BaseError<P>
where
    P::Target: AsRef<[u8]>,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            BaseErrorKind::Unimplemented => None,
            BaseErrorKind::Buffer(e) => Some(e),
        }
    }
}

impl<P: Deref> fmt::Display for BaseError<P>
where
    P::Target: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl<P: Deref> BaseError<P>
where
    P::Target: AsRef<[u8]>,
{
    /// Creates a new error of kind [`BaseErrorKind::Unimplemented`]
    pub fn unimplemented(mem: P) -> Self {
        Self {
            mem,
            kind: BaseErrorKind::Unimplemented,
        }
    }
}

#[derive(Debug, Display, Clone, PartialEq, Eq)]
/// The different kinds of [`BaseError`]s
pub enum BaseErrorKind {
    /// Unimplemented
    Unimplemented,
    /// Failed to read from the buffer
    Buffer(BufferError),
}

impl From<BufferError> for BaseErrorKind {
    fn from(b: BufferError) -> Self {
        Self::Buffer(b)
    }
}

/// The base result type
pub type BaseResult<P, T> = std::result::Result<BaseHandle<P, T>, BaseError<P>>;

impl<P: Deref, T> BaseHandle<P, T>
where
    P::Target: AsRef<[u8]>,
{
    /// Get the tables
    pub fn map_into<M, O, E>(self, map: M) -> BaseResult<P, O>
    where
        M: Fn(&[u8], T) -> std::result::Result<O, E>,
        E: Into<BaseErrorKind>,
    {
        match map(self.mem.deref().as_ref(), self.raw) {
            Ok(new_raw) => Ok(BaseHandle {
                mem: self.mem,
                raw: new_raw,
            }),
            Err(e) => Err(BaseError {
                mem: self.mem,
                kind: e.into(),
            }),
        }
    }
}

impl<P: Deref> BaseHandle<P, ()>
where
    P::Target: AsRef<[u8]>,
{
    /// Get the tables
    pub fn into_tables(self) -> BaseResult<P, FDBHeader> {
        self.map_into(buffer::header)
    }
}

impl<P: Deref> BaseHandle<P, FDBHeader>
where
    P::Target: AsRef<[u8]>,
{
    /// Get the tables
    pub fn into_table_at(self, index: usize) -> BaseResult<P, Option<FDBTableHeader>> {
        self.map_into(|buf, header| -> Res<Option<FDBTableHeader>> {
            let slice = buffer::table_headers(buf, &header)?;
            Ok(slice.get(index).copied())
        })
    }

    /// Get the tables
    pub fn into_table_by_name(self, name: &Latin1Str) -> BaseResult<P, Option<FDBTableHeader>> {
        self.map_into(|buf, header| -> Res<Option<FDBTableHeader>> {
            let slice = buffer::table_headers(buf, &header)?;
            match slice.binary_search_by(|t| cmp_table_header_name(buf, name.as_bytes(), *t)) {
                Ok(index) => Ok(Some(*slice.get(index).unwrap())),
                Err(_) => Ok(None),
            }
        })
    }
}

impl<P: Deref> BaseHandle<P, FDBTableHeader>
where
    P::Target: AsRef<[u8]>,
{
    /// Get the tables
    pub fn into_definition(self) -> BaseResult<P, FDBTableDefHeader> {
        self.map_into(buffer::table_definition)
    }

    /// Get the tables
    pub fn into_data(self) -> BaseResult<P, FDBTableDataHeader> {
        self.map_into(buffer::table_data)
    }
}

impl<P: Deref> BaseHandle<P, FDBTableDataHeader>
where
    P::Target: AsRef<[u8]>,
{
    /// Get the bucket for a particular id / hash
    pub fn get_bucket_for_hash(self, id: u32) -> BaseResult<P, FDBBucketHeader> {
        self.map_into::<_,_,BufferError>(|buf, raw| {
            let bucket_count = raw.buckets.count as usize;
            let buckets_addr = raw.buckets.base_offset as usize;
            let slice: &[FDBBucketHeader] = buffer::get_slice_at(buf, buckets_addr, bucket_count)?;
            Ok(slice[id as usize % bucket_count])
        })
    }
}

/// The basic database handle
pub type Database<'a> = Handle<'a, ()>;

impl<'a> Database<'a> {
    /// Create a new database handle
    pub fn new_ref(mem: &'a [u8]) -> Self {
        Self {
            mem: Buffer::new(mem),
            raw: (),
        }
    }

    /// Get the header for the local database
    pub fn tables(&self) -> Result<'a, FDBHeader> {
        let header = buffer::header(self.mem.as_bytes(), ())?;
        Ok(self.wrap(header))
    }
}

impl<'a> Handle<'a, FDBHeader> {
    /// Get the number of tables
    pub fn table_count(&self) -> u32 {
        self.raw.tables.count
    }

    /// Get the table header slice
    pub fn table_header_list(&self) -> Result<'a, FDBTableHeaderSlice<'a>> {
        let len = self.table_count() as usize * 8;
        let buf = self
            .mem
            .get_len_at(self.raw.tables.base_offset as usize, len)?;
        Ok(self.wrap(FDBTableHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;
    type IntoIter = Handle<'a, FDBTableHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw
            .next_back()
            .map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> Handle<'a, FDBTableHeader> {
    /// Get the table definition header
    pub fn table_def_header(&self) -> Result<'a, FDBTableDefHeader> {
        let raw = self.mem.table_def_header(self.raw.table_def_header_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the table data header
    pub fn table_data_header(&self) -> Result<'a, FDBTableDataHeader> {
        let raw = self
            .mem
            .table_data_header(self.raw.table_data_header_addr)?;
        Ok(self.wrap(raw))
    }
}

impl<'a> Handle<'a, FDBTableDefHeader> {
    /// Get the number of columns
    pub fn column_count(&self) -> u32 {
        self.raw.column_count
    }

    /// Get the name of the table
    pub fn table_name(&self) -> Result<'a, &'a Latin1Str> {
        let raw = self.mem.string(self.raw.table_name_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the column header list
    pub fn column_header_list(&self) -> Result<'a, FDBColumnHeaderSlice<'a>> {
        let len = self.column_count() as usize * 8;
        let buf = self
            .mem
            .get_len_at(self.raw.column_header_list_addr as usize, len)?;
        Ok(self.wrap(FDBColumnHeaderSlice(buf)))
    }
}

#[cfg(feature = "serde-derives")]
impl<'a> serde::Serialize for Handle<'a, FDBTableDefHeader> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut tbl = serializer.serialize_struct("Table", 2)?;
        tbl.serialize_field("name", self.table_name().unwrap().raw().decode().as_ref())?;
        tbl.serialize_field("columns", &self.column_header_list().unwrap())?;
        tbl.end()
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;
    type IntoIter = Handle<'a, FDBColumnHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle { mem: self.mem, raw })
    }
}

#[cfg(feature = "serde-derives")]
impl<'a> serde::Serialize for Handle<'a, FDBColumnHeaderSlice<'a>> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = self.raw().len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in self.into_iter() {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw
            .next_back()
            .map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> Handle<'a, FDBColumnHeader> {
    /// Get the name of the column
    pub fn column_name(&self) -> Result<'a, &'a Latin1Str> {
        let raw = self.mem.string(self.raw.column_name_addr)?;
        Ok(self.wrap(raw))
    }

    /// Get the type of the column
    pub fn column_data_type(&self) -> StdResult<ValueType, UnknownValueType> {
        ValueType::try_from(self.raw.column_data_type)
    }
}

#[cfg(feature = "serde-derives")]
impl<'a> serde::Serialize for Handle<'a, FDBColumnHeader> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut col = serializer.serialize_struct("Column", 2)?;
        col.serialize_field("name", self.column_name().unwrap().raw().decode().as_ref())?;
        col.serialize_field("data_type", &self.column_data_type().unwrap())?;
        col.end()
    }
}

impl<'a> Handle<'a, FDBTableDataHeader> {
    /// Get the number of buckets
    pub fn bucket_count(&self) -> u32 {
        self.raw.buckets.count
    }

    /// Get the slice of buckets
    pub fn bucket_header_list(&self) -> Result<'a, FDBBucketHeaderSlice<'a>> {
        let len = self.bucket_count() as usize * 4;
        let buf = self
            .mem
            .get_len_at(self.raw.buckets.base_offset as usize, len)?;
        Ok(self.wrap(FDBBucketHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;
    type IntoIter = Handle<'a, FDBBucketHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle { mem: self.mem, raw })
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n).map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw
            .next_back()
            .map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> Handle<'a, FDBBucketHeader> {
    /// Get the first row header entry or `None`
    pub fn first(&self) -> Option<Result<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_head_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(self.mem.row_header_list_entry(addr).map(|e| self.wrap(e)))
        }
    }

    /// Get an iterator over all buckets
    pub fn row_header_iter(&self) -> Handle<'a, FDBRowHeaderRef> {
        self.wrap(FDBRowHeaderRef(self.raw.row_header_list_head_addr))
    }
}

#[derive(Debug, Copy, Clone)]
/// A newtype for a row header reference
#[allow(clippy::upper_case_acronyms)]
pub struct FDBRowHeaderRef(u32);

impl<'a> Iterator for Handle<'a, FDBRowHeaderRef> {
    type Item = Result<'a, FDBRowHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.raw.0;
        if addr == 0xFFFFFFFF {
            None
        } else {
            match self.mem.row_header_list_entry(addr) {
                Ok(e) => {
                    self.raw.0 = e.row_header_list_next_addr;
                    match self.mem.row_header(e.row_header_addr) {
                        Ok(rh) => Some(Ok(self.wrap(rh))),
                        Err(e) => {
                            self.raw.0 = 0xFFFFFFFF;
                            Some(Err(e))
                        }
                    }
                }
                Err(e) => {
                    self.raw.0 = 0xFFFFFFFF;
                    Some(Err(e))
                }
            }
        }
    }
}

impl<'a> Handle<'a, FDBRowHeaderListEntry> {
    /// Get the next row header list entry instance
    pub fn next(&self) -> Option<Result<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_next_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(self.mem.row_header_list_entry(addr).map(|e| self.wrap(e)))
        }
    }

    /// Get the associated row header.
    pub fn row_header(&self) -> Result<'a, FDBRowHeader> {
        let e = self.mem.row_header(self.raw.row_header_addr)?;
        Ok(self.wrap(e))
    }
}

impl<'a> Handle<'a, FDBRowHeader> {
    /// Get the number of fields
    pub fn field_count(&self) -> u32 {
        self.raw.fields.count
    }

    /// Get the slice of fields
    pub fn field_data_list(&self) -> Result<'a, FDBFieldDataSlice<'a>> {
        let len = self.field_count() as usize * 8;
        let buf = self
            .mem
            .get_len_at(self.raw.fields.base_offset as usize, len)?;
        Ok(self.wrap(FDBFieldDataSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;
    type IntoIter = Handle<'a, FDBFieldDataSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBFieldDataSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw
            .next_back()
            .map(|raw| Handle { mem: self.mem, raw })
    }
}

impl<'a> Handle<'a, FDBFieldData> {
    /// Get the value from this handle
    pub fn try_get_value(&self) -> Result<'a, FDBFieldValue> {
        // FIXME: propagate error
        Ok(self.map(|_, r| FDBFieldValue::try_from(r).unwrap()))
    }
}

impl<'a> Handle<'a, &'a Latin1Str> {
    /// Decode the string contained in this handle
    pub fn to_str(&self) -> Cow<'a, str> {
        self.raw.decode()
    }
}
