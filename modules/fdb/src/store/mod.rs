//! # Arena-Store &amp; Writer
//!
//! This module contains a (currently write-only) structure that represents a complete
//! FDB file. This structure can be used to create new FDB files.
//!
//! ## Usage
//!
//! ```
//! use latin1str::Latin1String;
//! use assembly_fdb::{
//!     value::{ValueType, owned::{Field}},
//!     store::{Database, Table},
//! };
//!
//! // Create a new database
//! let mut db = Database::new();
//!
//! // Create a table
//! let mut table = Table::new(16);
//!
//! // Add columns to the table
//! table.push_column(Latin1String::encode("ID"), ValueType::Integer);
//!
//! // Add data to the table
//! table.push_row(1, &[Field::Integer(1)]);
//! table.push_row(2, &[Field::Integer(2)]);
//! table.push_row(5, &[Field::Integer(5)]);
//! table.push_row(6, &[Field::Integer(6)]);
//!
//! // Add table to the database
//! db.push_table(Latin1String::encode("Table"), table);
//!
//! // Write the database to a type that implements [`std::io::Write`]
//! let mut out: Vec<u8> = Vec::new();
//! db.write(&mut out).expect("success");
//! ```

use crate::io::write::WriteLE;
use assembly_fdb_core::{
    file::{
        ArrayHeader, FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
    value::{owned::OwnedContext, Context, Value, ValueMapperMut, ValueType},
};
use latin1str::{Latin1Str, Latin1String};
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    io,
    mem::size_of,
};

#[cfg(test)]
mod tests;

/// Calculates the number of 4-byte units that are needed to store
/// this string with at least one null terminator.
fn req_buf_len(s: &Latin1Str) -> usize {
    s.len() / 4 + 1
}

/// The whole database
pub struct Database {
    tables: BTreeMap<Latin1String, Table>,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    /// Create a new database
    pub fn new() -> Self {
        Self {
            tables: BTreeMap::new(),
        }
    }

    /// Push a table to the database
    pub fn push_table<S>(&mut self, name: S, table: Table)
    where
        S: Into<Latin1String>,
    {
        self.tables.insert(name.into(), table);
    }

    /// Computes the size of the serialized database
    pub fn compute_size(&self) -> usize {
        let table_size: usize = self
            .tables
            .iter()
            .map(|(n, t)| t.compute_size(n))
            .map(|x| x.def + x.data)
            .sum();
        8 // FDBHeader
        + table_size
    }

    /// Write the database to an output stream
    pub fn write<O: io::Write>(&self, out: &mut O) -> io::Result<()> {
        let base_offset = 8;
        let count = self
            .tables
            .len()
            .try_into()
            .expect("tables.len() does not fit in u32");
        let header = FDBHeader {
            tables: ArrayHeader { base_offset, count },
        };
        header.tables.write_le(out)?;
        let len_vec: Vec<_> = self.tables.iter().map(|(n, t)| t.compute_size(n)).collect();
        let mut start_vec = Vec::with_capacity(self.tables.len());
        let table_list_base = base_offset + count * size_of::<FDBTableHeader>() as u32;
        let mut start = table_list_base;
        for len in len_vec.iter() {
            start_vec.push(start);
            Table::write_header(&mut start, len, out)?;
        }

        let mut start = table_list_base;
        for (table_name, table) in &self.tables {
            start = table.write(table_name, start, out)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct TableSize {
    def: usize,
    data: usize,
}

/// A single table
pub struct Table {
    columns: Vec<Column>,
    strings: StringArena,
    i64s: Vec<i64>,
    buckets: Vec<Bucket>,
    rows: Vec<Row>,
    fields: Vec<Field>,
}

type StringArena = BTreeMap<usize, Vec<Latin1String>>;

struct StoreMapper<'t> {
    strings: &'t mut StringArena,
    i64s: &'t mut Vec<i64>,
}

impl<'t> ValueMapperMut<OwnedContext, StoreContext> for StoreMapper<'t> {
    fn map_string(&mut self, from: &String) -> TextRef {
        let s = Latin1String::encode(from).into_owned();
        let lkey = req_buf_len(&s);
        let lstrings = self.strings.entry(lkey).or_default();
        let inner = /*if let Some(index) = lstrings.iter().position(|p| s == *p) {
            index
        } else */{
            let len = lstrings.len();
            lstrings.push(s);
            len
        };
        TextRef { outer: lkey, inner }
    }

    fn map_i64(&mut self, from: &i64) -> I64Ref {
        let index = self.i64s.len();
        self.i64s.push(*from);
        I64Ref { index }
    }

    fn map_xml(&mut self, from: &String) -> TextRef {
        self.map_string(from)
    }
}

impl Table {
    /// Creates a new table
    pub fn new(bucket_count: usize) -> Self {
        Table {
            buckets: vec![
                Bucket {
                    first_row_last: None
                };
                bucket_count
            ],
            columns: vec![],
            fields: vec![],
            strings: BTreeMap::new(),
            rows: vec![],
            i64s: vec![],
        }
    }

    /// Get all columns
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Add a column to this table
    pub fn push_column<S>(&mut self, name: S, data_type: ValueType)
    where
        S: Into<Latin1String>,
    {
        self.columns.push(Column {
            data_type,
            name: name.into(),
        })
    }

    /// Push a row into this table
    pub fn push_row(&mut self, pk: usize, fields: &[crate::value::owned::Field]) {
        let first_field_index = self.fields.len();
        let row = self.rows.len();

        // find out where to place it
        let bucket_index = pk % self.buckets.len();
        let bucket = &mut self.buckets[bucket_index];

        // Add to linked list
        if let Some((_, last)) = &mut bucket.first_row_last {
            self.rows[*last].next_row = Some(row);
            *last = row;
        } else {
            bucket.first_row_last = Some((row, row))
        }

        self.rows.push(Row {
            first_field_index,
            count: fields.len().try_into().unwrap(),
            next_row: None,
        });

        let mut mapper = StoreMapper {
            strings: &mut self.strings,
            i64s: &mut self.i64s,
        };
        for field in fields {
            self.fields.push(field.map(&mut mapper));
        }
    }

    fn write_header<IO: io::Write>(
        start: &mut u32,
        len: &TableSize,
        out: &mut IO,
    ) -> io::Result<()> {
        let table_def_header_addr = *start;
        let table_data_header_addr = *start + u32::try_from(len.def).unwrap();

        FDBTableHeader {
            table_def_header_addr,
            table_data_header_addr,
        }
        .write_le(out)?;

        *start = table_data_header_addr + u32::try_from(len.data).unwrap();
        Ok(())
    }

    fn write<IO: io::Write>(
        &self,
        table_name: &Latin1Str,
        start: u32,
        out: &mut IO,
    ) -> io::Result<u32> {
        // Serialize table definition
        let column_count = self.columns.len().try_into().unwrap();
        let column_header_list_addr = start + size_of::<FDBTableDefHeader>() as u32;
        let table_name_addr =
            column_header_list_addr + size_of::<FDBColumnHeader>() as u32 * column_count;

        FDBTableDefHeader {
            column_count,
            table_name_addr,
            column_header_list_addr,
        }
        .write_le(out)?;

        let mut column_name_addr = table_name_addr + (req_buf_len(table_name) as u32 * 4);
        for column in &self.columns {
            FDBColumnHeader {
                column_data_type: column.data_type.into(),
                column_name_addr,
            }
            .write_le(out)?;
            column_name_addr += req_buf_len(&column.name) as u32 * 4;
        }

        table_name.write_le(out)?;
        for column in &self.columns {
            column.name.write_le(out)?;
        }

        // Serialize table data
        let bucket_base_offset = column_name_addr + size_of::<FDBTableDataHeader>() as u32;
        let bucket_count = self.buckets.len().try_into().unwrap();

        FDBTableDataHeader {
            buckets: ArrayHeader {
                count: bucket_count,
                base_offset: bucket_base_offset,
            },
        }
        .write_le(out)?;

        let row_header_list_base =
            bucket_base_offset + bucket_count * size_of::<FDBBucketHeader>() as u32;

        let map_row_entry =
            &|index| row_header_list_base + (index * size_of::<FDBRowHeaderListEntry>()) as u32;

        for bucket in &self.buckets {
            let row_header_list_head_addr = bucket
                .first_row_last
                .map(|(first, _)| first)
                .map(map_row_entry)
                .unwrap_or(0xffffffff);

            FDBBucketHeader {
                row_header_list_head_addr,
            }
            .write_le(out)?;
        }

        let row_count: u32 = self.rows.len().try_into().unwrap();
        let row_header_base =
            row_header_list_base + row_count * size_of::<FDBRowHeaderListEntry>() as u32;

        for (index, row) in self.rows.iter().enumerate() {
            let row_header_addr = row_header_base + (index * size_of::<FDBRowHeader>()) as u32;
            let row_header_list_next_addr = row.next_row.map(map_row_entry).unwrap_or(0xffffffff);
            FDBRowHeaderListEntry {
                row_header_addr,
                row_header_list_next_addr,
            }
            .write_le(out)?;
        }

        let field_base_offset = row_header_base + row_count * size_of::<FDBRowHeader>() as u32;

        for row in &self.rows {
            let fields = ArrayHeader {
                base_offset: field_base_offset
                    + (row.first_field_index * size_of::<FDBFieldData>()) as u32,
                count: row.count,
            };
            FDBRowHeader { fields }.write_le(out)?;
        }

        let i64s_base_offset =
            field_base_offset + (self.fields.len() * size_of::<FDBFieldData>()) as u32;
        let strings_base_offset = i64s_base_offset + (self.i64s.len() * size_of::<u64>()) as u32;

        let mut string_len_base = strings_base_offset;
        let mut string_len_offsets = BTreeMap::new();
        for (&key, value) in &self.strings {
            let string_len = key * 4;
            string_len_offsets.insert(key, string_len_base);
            string_len_base += (string_len * value.len()) as u32;
        }

        const TRUE_LE32: [u8; 4] = [1, 0, 0, 0];
        const FALSE_LE32: [u8; 4] = [0, 0, 0, 0];

        for field in &self.fields {
            let (data_type, value) = match field {
                Field::Nothing => (0, [0; 4]),
                Field::Integer(i) => (1, i.to_le_bytes()),
                Field::Float(f) => (3, f.to_le_bytes()),
                Field::Text(TextRef { outer, inner }) => (4, {
                    let v = string_len_offsets.get(outer).unwrap() + (inner * outer * 4) as u32;
                    v.to_le_bytes()
                }),
                Field::Boolean(b) => (5, if *b { TRUE_LE32 } else { FALSE_LE32 }),
                Field::BigInt(i64_ref) => (6, {
                    let v = i64s_base_offset + (i64_ref.index * size_of::<u64>()) as u32;
                    v.to_le_bytes()
                }),
                Field::VarChar(text_ref) => (8, {
                    let v = string_len_offsets.get(&text_ref.outer).unwrap()
                        + (text_ref.inner * text_ref.outer * 4) as u32;
                    v.to_le_bytes()
                }),
            };
            FDBFieldData { data_type, value }.write_le(out)?;
        }

        // Write out all i64s
        for &num in &self.i64s {
            out.write_all(&num.to_le_bytes())?;
        }

        // Write out all strings
        for value in self.strings.values() {
            for string in value {
                string.write_le(out)?;
            }
        }

        // Increment final offset
        Ok(string_len_base)
    }

    fn compute_def_size(&self, name: &Latin1Str) -> usize {
        size_of::<FDBTableDefHeader>()
            + req_buf_len(name) * 4
            + size_of::<FDBColumnHeader>() * self.columns.len()
            + self
                .columns
                .iter()
                .map(|c| req_buf_len(&c.name))
                .sum::<usize>()
                * 4
    }

    fn compute_data_size(&self) -> usize {
        let string_size: usize = self.strings.iter().map(|(k, v)| k * v.len()).sum(); // Strings
        size_of::<FDBTableDataHeader>()
            + size_of::<FDBBucketHeader>() * self.buckets.len()
            + size_of::<FDBRowHeaderListEntry>() * self.rows.len()
            + size_of::<FDBRowHeader>() * self.rows.len()
            + size_of::<FDBFieldData>() * self.fields.len()
            + 4 * string_size
            + size_of::<u64>() * self.i64s.len()
    }

    fn compute_size(&self, name: &Latin1Str) -> TableSize {
        TableSize {
            def: self.compute_def_size(name),
            data: self.compute_data_size(),
        }
    }
}

/// A single column
pub struct Column {
    name: Latin1String,
    data_type: ValueType,
}

impl Column {
    /// Get the data type of this column
    pub fn value_type(&self) -> ValueType {
        self.data_type
    }
}

/// A single bucket
#[derive(Debug, Copy, Clone)]
struct Bucket {
    first_row_last: Option<(usize, usize)>,
}

/// A single row
struct Row {
    first_field_index: usize,
    count: u32,
    next_row: Option<usize>,
}

/// The [`Context`] for this modules [`Field`]
struct StoreContext;

/// Reference to an arena allocated string
struct TextRef {
    /// The length-key of the string
    outer: usize,
    /// The index in the strings array
    inner: usize,
}

/// Reference to an arena allocated i64
struct I64Ref {
    /// The offset of the value
    index: usize,
}

impl Context for StoreContext {
    type String = TextRef;
    type I64 = I64Ref;
    type XML = TextRef;
}

type Field = Value<StoreContext>;
