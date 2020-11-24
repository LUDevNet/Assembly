use assembly_core::buffer::{Unaligned, LEI64};
use derive_new::new;
use memchr::memchr;

mod c;
use super::{core::ValueType, de::slice::Latin1Str};
use c::{
    FDBBucketHeaderC, FDBColumnHeaderC, FDBFieldDataC, FDBHeaderC, FDBRowHeaderC,
    FDBRowHeaderListEntryC, FDBTableDataHeaderC, FDBTableDefHeaderC, FDBTableHeaderC,
};
use std::{borrow::Cow, cmp::Ordering};

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

#[derive(Copy, Clone, new)]
pub struct Database<'a> {
    buf: &'a [u8],
}

impl<'a> Database<'a> {
    pub fn tables(self) -> Tables<'a> {
        let header = FDBHeaderC::cast(self.buf, 0);
        let len = header.table_count.extract();
        let base = header.table_header_list_addr.extract();
        let slice = FDBTableHeaderC::cast_slice(self.buf, base, len);
        Tables {
            buf: self.buf,
            slice,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tables<'a> {
    buf: &'a [u8],
    slice: &'a [FDBTableHeaderC],
}

fn map_table_header<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBTableHeaderC) -> Table<'a> {
    move |header: &'a FDBTableHeaderC| {
        let table_header = header.extract();

        let def_header =
            FDBTableDefHeaderC::cast(buf, table_header.table_def_header_addr).extract();
        let data_header =
            FDBTableDataHeaderC::cast(buf, table_header.table_data_header_addr).extract();

        let name = get_latin1_str(buf, def_header.table_name_addr);
        let columns = FDBColumnHeaderC::cast_slice(
            buf,
            def_header.column_header_list_addr,
            def_header.column_count,
        );
        let buckets = FDBBucketHeaderC::cast_slice(
            buf,
            data_header.bucket_header_list_addr,
            data_header.bucket_count,
        );

        Table {
            buf,
            name,
            columns,
            buckets,
        }
    }
}

impl<'a> Tables<'a> {
    pub fn len(self) -> usize {
        self.slice.len()
    }

    pub fn is_empty(self) -> bool {
        self.slice.len() == 0
    }

    pub fn get(self, index: usize) -> Option<Table<'a>> {
        self.slice.get(index).map(map_table_header(self.buf))
    }

    pub fn iter(&self) -> impl Iterator<Item = Table<'a>> {
        self.slice.iter().map(map_table_header(self.buf))
    }

    pub fn by_name(&self, name: &str) -> Option<Table<'a>> {
        let bytes = name.as_bytes();
        self.slice
            .binary_search_by(|table_header| {
                let def_header_addr = table_header.table_def_header_addr.extract();
                let def_header = FDBTableDefHeaderC::cast(self.buf, def_header_addr);

                let name_addr = def_header.table_name_addr.extract() as usize;
                let name_bytes = &self.buf[name_addr..];

                for i in 0..bytes.len() {
                    match name_bytes[i].cmp(&bytes[i]) {
                        Ordering::Equal => {}
                        Ordering::Less => {
                            // the null terminator is a special case of this one
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                    }
                }
                if name_bytes[bytes.len()] == 0 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            })
            .ok()
            .and_then(|index| self.get(index))
    }
}

#[derive(Copy, Clone)]
pub struct Table<'a> {
    buf: &'a [u8],
    name: &'a Latin1Str,
    columns: &'a [FDBColumnHeaderC],
    buckets: &'a [FDBBucketHeaderC],
}

#[allow(clippy::needless_lifetimes)] // <- clippy gets this wrong, presumably because of impl trait?
fn map_column_header<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBColumnHeaderC) -> Column<'a> {
    move |header: &FDBColumnHeaderC| {
        let column_header = header.extract();
        let name = get_latin1_str(buf, column_header.column_name_addr);
        let domain = ValueType::from(column_header.column_data_type);

        Column { name, domain }
    }
}

fn get_row_header_list_entry(buf: &[u8], addr: u32) -> Option<&FDBRowHeaderListEntryC> {
    if addr == u32::MAX {
        None
    } else {
        Some(FDBRowHeaderListEntryC::cast(buf, addr))
    }
}

#[allow(clippy::needless_lifetimes)] // <- clippy gets this wrong
fn map_bucket_header<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBBucketHeaderC) -> Bucket<'a> {
    move |header: &FDBBucketHeaderC| {
        let bucket_header = header.extract();
        let addr = bucket_header.row_header_list_head_addr;
        let first = get_row_header_list_entry(buf, addr);
        Bucket { buf, first }
    }
}

impl<'a> Table<'a> {
    /// Get the undecoded name of the table
    pub fn name_raw(&self) -> &Latin1Str {
        self.name
    }

    /// Get the name of the table
    pub fn name(&self) -> Cow<str> {
        self.name.decode()
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
        self.columns.get(index).map(map_column_header(self.buf))
    }

    /// Get the column iterator
    ///
    /// **Note**: This does some computation, call only once if possible
    pub fn column_iter(&self) -> impl Iterator<Item = Column<'a>> {
        self.columns.iter().map(map_column_header(self.buf))
    }

    /// The amount of columns in this table
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get the bucket at the index
    ///
    /// **Note**: This does some computation, call only once per bucket if possible
    pub fn bucket_at(&self, index: usize) -> Option<Bucket<'a>> {
        self.buckets.get(index).map(map_bucket_header(self.buf))
    }

    /// Get the bucket iterator
    ///
    /// **Note**: This does some computation, call only once if possible
    pub fn bucket_iter(&self) -> impl Iterator<Item = Bucket<'a>> {
        self.buckets.iter().map(map_bucket_header(self.buf))
    }

    /// Get the amount of buckets
    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// Get an iterator over all rows
    pub fn row_iter(&self) -> impl Iterator<Item = Row<'a>> {
        self.bucket_iter().map(|b| b.row_iter()).flatten()
    }
}

pub struct Column<'a> {
    name: &'a Latin1Str,
    domain: ValueType,
}

impl<'a> Column<'a> {
    pub fn name(&self) -> Cow<'a, str> {
        self.name.decode()
    }

    pub fn value_type(&self) -> ValueType {
        self.domain
    }
}

pub struct Bucket<'a> {
    buf: &'a [u8],
    first: Option<&'a FDBRowHeaderListEntryC>,
}

impl<'a> Bucket<'a> {
    pub fn row_iter(&self) -> RowHeaderIter<'a> {
        RowHeaderIter {
            buf: self.buf,
            next: self.first,
        }
    }
}

pub struct RowHeaderIter<'a> {
    buf: &'a [u8],
    next: Option<&'a FDBRowHeaderListEntryC>,
}

impl<'a> Iterator for RowHeaderIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let entry = next.extract();
            self.next = get_row_header_list_entry(self.buf, entry.row_header_list_next_addr);
            let row_header = FDBRowHeaderC::cast(self.buf, entry.row_header_addr).extract();

            let fields = FDBFieldDataC::cast_slice(
                self.buf,
                row_header.field_data_list_addr,
                row_header.field_count,
            );

            Some(Row {
                buf: self.buf,
                fields,
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Row<'a> {
    buf: &'a [u8],
    fields: &'a [FDBFieldDataC],
}

#[allow(clippy::needless_lifetimes)] // <- clippy gets this wrong
fn map_field<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBFieldDataC) -> Field<'a> {
    move |data: &FDBFieldDataC| {
        let data_type = ValueType::from(data.data_type.extract());
        let bytes = data.value.0;
        match data_type {
            ValueType::Nothing => Field::Nothing,
            ValueType::Integer => Field::Integer(i32::from_le_bytes(bytes)),
            ValueType::Float => Field::Float(f32::from_le_bytes(bytes)),
            ValueType::Text => {
                let addr = u32::from_le_bytes(bytes);
                let text = get_latin1_str(buf, addr);
                Field::Text(text)
            }
            ValueType::Boolean => Field::Boolean(bytes != [0, 0, 0, 0]),
            ValueType::BigInt => {
                let addr = u32::from_le_bytes(bytes);
                let val = LEI64::cast(buf, addr).extract();
                Field::BigInt(val)
            }
            ValueType::VarChar => {
                let addr = u32::from_le_bytes(bytes);
                let text = get_latin1_str(buf, addr);
                Field::VarChar(text)
            }
            ValueType::Unknown(i) => unimplemented!("Cannot read unknown value type {}", i),
        }
    }
}

impl<'a> Row<'a> {
    /// Get the field at the index
    pub fn field_at(&self, index: usize) -> Option<Field<'a>> {
        self.fields.get(index).map(map_field(self.buf))
    }

    /// Get the iterator over all fields
    pub fn field_iter(&self) -> impl Iterator<Item = Field<'a>> {
        self.fields.iter().map(map_field(self.buf))
    }

    /// Get the count of fields
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum Field<'a> {
    Nothing,
    Integer(i32),
    Float(f32),
    Text(&'a Latin1Str),
    Boolean(bool),
    BigInt(i64),
    VarChar(&'a Latin1Str),
}

impl<'a> Field<'a> {
    pub fn into_opt_integer(self) -> Option<i32> {
        if let Self::Integer(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_opt_float(self) -> Option<f32> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_opt_text(self) -> Option<&'a Latin1Str> {
        if let Self::Text(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_opt_boolean(self) -> Option<bool> {
        if let Self::Boolean(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_opt_big_int(self) -> Option<i64> {
        if let Self::BigInt(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_opt_varchar(self) -> Option<&'a Latin1Str> {
        if let Self::VarChar(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
