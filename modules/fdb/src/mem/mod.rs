//! Low-Level API that is suitable for non-little-endian machines
//!
//! This is the default in-memory API the the FDB file format. It is useful
//! for batch processing because it is fast and only loads the values that
//! are accessed.
//!
//! The reference structures in this module all implement [`Copy`].
//!
//! The only limitation is, that all references are bounded by the lifetime
//! of the original database buffer.
use assembly_core::buffer::{self, Repr, LEI64};
use buffer::CastError;
use memchr::memchr;

mod c;
use super::{
    common::{Context, Latin1Str, Value, ValueMapperMut, ValueType},
    file::{FDBFieldValue, FileContext, IndirectValue},
    ro::{
        buffer::{compare_bytes, Buffer},
        Handle, RefHandle,
    },
};
use c::{
    FDBBucketHeaderC, FDBColumnHeaderC, FDBFieldDataC, FDBHeaderC, FDBRowHeaderListEntryC,
    FDBTableDataHeaderC, FDBTableDefHeaderC, FDBTableHeaderC,
};
use std::{
    borrow::Cow,
    convert::{Infallible, TryFrom},
};

pub mod iter;

use iter::{BucketIter, TableRowIter};
pub use iter::{FieldIter, RowHeaderIter, TableIter}; // < FIXME> remove with next major update

fn get_latin1_str(buf: &[u8], offset: u32) -> &Latin1Str {
    let (_, haystack) = buf.split_at(offset as usize);
    if let Some(end) = memchr(0, haystack) {
        let (content, _) = haystack.split_at(end);
        unsafe { Latin1Str::from_bytes_unchecked(content) }
    } else {
        panic!(
            "Offset {} is supposed to be a string but does not have a null-terminator",
            offset
        );
    }
}

/// A complete in-memory read-only database
///
/// This struct contains a reference to the complete byte buffer of an FDB file.
#[derive(Copy, Clone)]
pub struct Database<'a> {
    inner: Handle<'a, ()>,
}

impl<'a> Database<'a> {
    /// Create a new database reference
    pub fn new(buf: &'a [u8]) -> Self {
        let inner = Handle::new_ref(buf);
        Self { inner }
    }

    /// Get a reference to the header
    pub fn header(self) -> Result<Header<'a>, CastError> {
        let inner = self.inner.try_map_cast(0)?;
        Ok(Header { inner })
    }

    /// Returns a reference to the tables array
    pub fn tables(self) -> Result<Tables<'a>, CastError> {
        let header = self.header()?;
        let tables = header.tables()?;
        Ok(tables)
    }
}

#[derive(Copy, Clone)]
/// Reference to the tables array
pub struct Header<'a> {
    inner: RefHandle<'a, FDBHeaderC>,
}

impl<'a> Header<'a> {
    fn tables(self) -> Result<Tables<'a>, CastError> {
        let header = self.inner.map_extract();
        let inner = self.inner.try_map_cast_array(header.into_raw().tables)?;
        Ok(Tables { inner })
    }
}

fn map_table_header<'a>(handle: RefHandle<'a, FDBTableHeaderC>) -> Result<Table<'a>, CastError> {
    let table_header = handle.into_raw().extract();

    let def_header: &'a FDBTableDefHeaderC =
        handle.buf().try_cast(table_header.table_def_header_addr)?;
    let def_header = def_header.extract();

    let data_header: &'a FDBTableDataHeaderC =
        handle.buf().try_cast(table_header.table_data_header_addr)?;
    let data_header = data_header.extract();

    let name = get_latin1_str(handle.buf().as_bytes(), def_header.table_name_addr);

    let columns: RefHandle<'a, [FDBColumnHeaderC]> =
        handle.try_map_cast_slice(def_header.column_header_list_addr, def_header.column_count)?;

    let buckets: RefHandle<'a, [FDBBucketHeaderC]> =
        handle.try_map_cast_array(data_header.buckets)?;

    Ok(Table::new(handle.wrap(InnerTable {
        name,
        columns: columns.raw(),
        buckets: buckets.raw(),
    })))
}

#[derive(Copy, Clone)]
/// Reference to the tables array
pub struct Tables<'a> {
    inner: RefHandle<'a, [FDBTableHeaderC]>,
}

impl<'a> Tables<'a> {
    /// Returns the length of the tables array
    pub fn len(self) -> usize {
        self.inner.into_raw().len()
    }

    /// Checks whether the tables array is empty
    pub fn is_empty(self) -> bool {
        self.inner.into_raw().len() == 0
    }

    /// Get the table reference at the specified index
    pub fn get(self, index: usize) -> Option<Result<Table<'a>, CastError>> {
        self.inner.get(index).map(map_table_header)
    }

    /// Get an interator over all tables
    pub fn iter(&self) -> TableIter<'a> {
        TableIter::new(&self.inner)
    }

    /// Get a table by its name
    pub fn by_name(&self, name: &str) -> Option<Result<Table<'a>, CastError>> {
        let bytes = name.as_bytes();
        self.inner
            .into_raw()
            .binary_search_by(|table_header| {
                let def_header_addr = table_header.table_def_header_addr.extract();
                let def_header = buffer::cast::<FDBTableDefHeaderC>(
                    self.inner.buf().as_bytes(),
                    def_header_addr,
                );

                let name_addr = def_header.table_name_addr.extract() as usize;
                let name_bytes = &self.inner.buf().as_bytes()[name_addr..];

                compare_bytes(bytes, name_bytes)
            })
            .ok()
            .and_then(|index| self.get(index))
    }
}

#[allow(clippy::needless_lifetimes)] // <- clippy gets this wrong, presumably because of impl trait?
fn map_column_header<'a>(
    buf: &'a [u8],
) -> impl Fn(&'a FDBColumnHeaderC) -> Column<'a> + Copy + Clone {
    move |header: &FDBColumnHeaderC| {
        let column_header = header.extract();
        let name = get_latin1_str(buf, column_header.column_name_addr);
        // FIXME: remove unwrap
        let domain = ValueType::try_from(column_header.column_data_type).unwrap();

        Column { name, domain }
    }
}

fn get_row_header_list_entry(buf: Buffer, addr: u32) -> Option<&FDBRowHeaderListEntryC> {
    if addr == u32::MAX {
        None
    } else {
        Some(buf.cast::<FDBRowHeaderListEntryC>(addr))
    }
}

/*#[allow(clippy::needless_lifetimes)] // <- clippy gets this wrong
fn map_bucket_header<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBBucketHeaderC) -> Bucket<'a> {
    move |header: &FDBBucketHeaderC| {
        let bucket_header = header.extract();
        let addr = bucket_header.row_header_list_head_addr;
        let first = get_row_header_list_entry(buf, addr);
        Bucket { buf, first }
    }
}*/

#[derive(Copy, Clone)]
struct InnerTable<'a> {
    name: &'a Latin1Str,
    columns: &'a [FDBColumnHeaderC],
    buckets: &'a [FDBBucketHeaderC],
}

#[derive(Copy, Clone)]
/// Reference to a single table
pub struct Table<'a> {
    inner: Handle<'a, InnerTable<'a>>,
}

impl<'a> Table<'a> {
    fn new(inner: Handle<'a, InnerTable<'a>>) -> Self {
        Self { inner }
    }

    /// Get the undecoded name of the table
    pub fn name_raw(&self) -> &'a Latin1Str {
        self.inner.raw.name
    }

    /// Get the name of the table
    pub fn name(&self) -> Cow<'a, str> {
        self.inner.raw.name.decode()
    }

    /// Get a list of rows by index
    pub fn index_iter(&self, id: u32) -> impl Iterator<Item = Row<'a>> {
        let bucket: usize = id as usize % self.bucket_count();
        self.bucket_at(bucket).into_iter().flat_map(move |b| {
            b.row_iter()
                .filter(move |r| r.field_at(0) == Some(Field::Integer(id as i32)))
        })
    }

    /// Get the column at the index
    ///
    /// **Note**: This does some computation, call only once per colum if possible
    pub fn column_at(&self, index: usize) -> Option<Column<'a>> {
        self.inner
            .raw
            .columns
            .get(index)
            .map(map_column_header(self.inner.mem.as_bytes()))
    }

    /// Get the column iterator
    ///
    /// **Note**: This does some computation, call only once if possible
    pub fn column_iter(&self) -> impl Iterator<Item = Column<'a>> + Clone {
        self.inner
            .raw
            .columns
            .iter()
            .map(map_column_header(self.inner.mem.as_bytes()))
    }

    /// The amount of columns in this table
    pub fn column_count(&self) -> usize {
        self.inner.raw.columns.len()
    }

    /// Get the bucket at the index
    ///
    /// **Note**: This does some computation, call only once per bucket if possible
    pub fn bucket_at(&self, index: usize) -> Option<Bucket<'a>> {
        self.inner
            .map_val(|raw| raw.buckets)
            .get(index)
            .map(|e| {
                e.map_extract()
                    .map_val(|r| r.row_header_list_head_addr)
                    .map(get_row_header_list_entry)
                    .transpose()
            })
            .map(Bucket::new)
    }

    /// Get the bucket for the given hash
    ///
    /// **Note**: This always calls [Table::bucket_at] exactly once
    pub fn bucket_for_hash(&self, hash: u32) -> Bucket<'a> {
        let index = hash as usize % self.inner.raw.buckets.len();
        self.bucket_at(index).unwrap()
    }

    /// Get the bucket iterator
    ///
    /// **Note**: This does some computation, call only once if possible
    pub fn bucket_iter(&self) -> BucketIter<'a> {
        BucketIter::new(&self.inner.map_val(|r| r.buckets))
    }

    /// Get the amount of buckets
    pub fn bucket_count(&self) -> usize {
        self.inner.raw.buckets.len()
    }

    /// Get an iterator over all rows
    pub fn row_iter(&self) -> TableRowIter<'a> {
        TableRowIter::new(self.bucket_iter())
    }
}

/// Reference to a column definition
pub struct Column<'a> {
    name: &'a Latin1Str,
    domain: ValueType,
}

impl<'a> Column<'a> {
    /// Returns the name of a column
    pub fn name(&self) -> Cow<'a, str> {
        self.name.decode()
    }

    /// Returns the name of a column
    pub fn name_raw(&self) -> &'a Latin1Str {
        self.name
    }

    /// Returns the default value type of the column
    pub fn value_type(&self) -> ValueType {
        self.domain
    }
}

/// Reference to a single bucket
#[derive(Debug, Copy, Clone)]
pub struct Bucket<'a> {
    inner: Option<RefHandle<'a, FDBRowHeaderListEntryC>>,
}

impl<'a> Bucket<'a> {
    /// Returns an iterator over all rows in this bucket
    pub fn row_iter(&self) -> RowHeaderIter<'a> {
        RowHeaderIter::new(self.inner)
    }

    /// Check whether the bucket is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    fn new(inner: Option<RefHandle<'a, FDBRowHeaderListEntryC>>) -> Self {
        Self { inner }
    }
}

#[derive(Copy, Clone)]
/// Reference to a single row
pub struct Row<'a> {
    inner: RefHandle<'a, [FDBFieldDataC]>,
}

fn get_field<'a>(buf: Buffer<'a>, data: &'a FDBFieldDataC) -> Field<'a> {
    let data_type = ValueType::try_from(data.data_type.extract()).unwrap();
    let bytes = data.value.0;
    get_field_raw(buf, data_type, bytes)
}

fn get_field_raw(buf: Buffer, data_type: ValueType, bytes: [u8; 4]) -> Field {
    match data_type {
        ValueType::Nothing => Field::Nothing,
        ValueType::Integer => Field::Integer(i32::from_le_bytes(bytes)),
        ValueType::Float => Field::Float(f32::from_le_bytes(bytes)),
        ValueType::Text => {
            let addr = u32::from_le_bytes(bytes);
            let text = get_latin1_str(buf.as_bytes(), addr);
            Field::Text(text)
        }
        ValueType::Boolean => Field::Boolean(bytes != [0, 0, 0, 0]),
        ValueType::BigInt => {
            let addr = u32::from_le_bytes(bytes);
            let val = buf.cast::<LEI64>(addr).extract();
            Field::BigInt(val)
        }
        ValueType::Xml => {
            let addr = u32::from_le_bytes(bytes);
            let text = get_latin1_str(buf.as_bytes(), addr);
            Field::Xml(text)
        }
    }
}

impl<'a> Row<'a> {
    fn new(inner: RefHandle<'a, [FDBFieldDataC]>) -> Self {
        Self { inner }
    }

    /// Get the field at the index
    pub fn field_at(&self, index: usize) -> Option<Field<'a>> {
        self.inner.get(index).map(|f| f.map(get_field).into_raw())
    }

    /// Get the iterator over all fields
    pub fn field_iter(&self) -> FieldIter<'a> {
        FieldIter::new(self.inner)
    }

    /// Get the count of fields
    pub fn field_count(&self) -> usize {
        self.inner.raw().len()
    }
}

impl<'a> IntoIterator for Row<'a> {
    type Item = Field<'a>;
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.field_iter()
    }
}

#[derive(Debug, PartialEq)]
/// The context for `mem::Field`
pub struct MemContext<'a> {
    _m: std::marker::PhantomData<fn() -> &'a ()>,
}

impl<'a> Context for MemContext<'a> {
    type String = &'a Latin1Str;
    type I64 = i64;
    type XML = &'a Latin1Str;
}

/// Value of or reference to a field value
pub type Field<'a> = Value<MemContext<'a>>;

struct MemFromFile<'a>(Buffer<'a>);

impl<'a> ValueMapperMut<FileContext, MemContext<'a>> for MemFromFile<'a> {
    fn map_string(&mut self, from: &IndirectValue) -> &'a Latin1Str {
        self.0.string(from.addr).unwrap()
    }

    fn map_i64(&mut self, from: &IndirectValue) -> i64 {
        self.0.i64(from.addr).unwrap()
    }

    fn map_xml(&mut self, from: &IndirectValue) -> &'a Latin1Str {
        self.0.string(from.addr).unwrap()
    }
}

impl<'a> TryFrom<Handle<'a, FDBFieldValue>> for Field<'a> {
    type Error = Infallible;

    fn try_from(value: Handle<'a, FDBFieldValue>) -> Result<Self, Self::Error> {
        let mut mem = MemFromFile(value.buf());
        Ok(value.raw().map(&mut mem))
    }
}
