//! # Arena-Store &amp; Writer
//!
//! This module contains a (currently write-only) structure that represents a complete
//! FDB file. This structure can be used to create new FDB files.
//!
//! ## Usage
//!
//! ```
//! use assembly_data::fdb::{
//!     core::{ValueType, Field},
//!     ro::slice::Latin1String,
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

use crate::fdb::core::ValueType;
use crate::fdb::ro::slice::Latin1String;
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    hint::unreachable_unchecked,
    io,
};

use super::{
    file::{
        ArrayHeader, FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
    ro::slice::Latin1Str,
};

#[cfg(test)]
mod tests;

fn write_array_header<O: io::Write>(array: &ArrayHeader, out: &mut O) -> io::Result<()> {
    out.write_all(&array.count.to_le_bytes())?;
    out.write_all(&array.base_offset.to_le_bytes())?;
    Ok(())
}

fn write_str<O: io::Write>(string: &Latin1Str, out: &mut O) -> io::Result<()> {
    out.write_all(string.as_bytes())?;
    match string.len() % 4 {
        0 => out.write_all(&[0, 0, 0, 0])?,
        1 => out.write_all(&[0, 0, 0])?,
        2 => out.write_all(&[0, 0])?,
        3 => out.write_all(&[0])?,
        _ => unsafe { unreachable_unchecked() },
    }
    Ok(())
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
        write_array_header(&header.tables, out)?;
        let len_vec: Vec<_> = self.tables.iter().map(|(n, t)| t.compute_size(n)).collect();
        let mut start_vec = Vec::with_capacity(self.tables.len());
        let table_list_base = base_offset + count * std::mem::size_of::<FDBTableHeader>() as u32;
        let mut start = table_list_base;
        for len in len_vec.iter().copied() {
            start_vec.push(start);

            let table_def_header_addr = start;
            let table_data_header_addr = start + u32::try_from(len.def).unwrap();
            let table_header = FDBTableHeader {
                table_def_header_addr,
                table_data_header_addr,
            };
            out.write_all(&table_header.table_def_header_addr.to_le_bytes())?;
            out.write_all(&table_header.table_data_header_addr.to_le_bytes())?;

            start = table_data_header_addr + u32::try_from(len.data).unwrap();
        }

        let mut start = table_list_base;
        for (table_name, table) in &self.tables {
            // Serialize table definition
            let column_count = table.columns.len().try_into().unwrap();
            let column_header_list_addr = start + std::mem::size_of::<FDBTableDefHeader>() as u32;
            let table_name_addr = column_header_list_addr
                + std::mem::size_of::<FDBColumnHeader>() as u32 * column_count;

            let table_def_header = FDBTableDefHeader {
                column_count,
                table_name_addr,
                column_header_list_addr,
            };
            out.write_all(&table_def_header.column_count.to_le_bytes())?;
            out.write_all(&table_def_header.table_name_addr.to_le_bytes())?;
            out.write_all(&table_def_header.column_header_list_addr.to_le_bytes())?;

            let mut column_name_addr = table_name_addr + (table_name.req_buf_len() as u32 * 4);
            for column in &table.columns {
                let column_header = FDBColumnHeader {
                    column_data_type: column.data_type.into(),
                    column_name_addr,
                };
                out.write_all(&column_header.column_data_type.to_le_bytes())?;
                out.write_all(&column_header.column_name_addr.to_le_bytes())?;
                column_name_addr += column.name.req_buf_len() as u32 * 4;
            }

            write_str(table_name, out)?;
            for column in &table.columns {
                write_str(&column.name, out)?;
            }

            // Serialize table data
            let bucket_base_offset =
                column_name_addr + std::mem::size_of::<FDBTableDataHeader>() as u32;
            let bucket_count = table.buckets.len().try_into().unwrap();
            let table_data_header = FDBTableDataHeader {
                buckets: ArrayHeader {
                    count: bucket_count,
                    base_offset: bucket_base_offset,
                },
            };
            write_array_header(&table_data_header.buckets, out)?;

            let row_header_list_base =
                bucket_base_offset + bucket_count * std::mem::size_of::<FDBBucketHeader>() as u32;

            for bucket in &table.buckets {
                let bucket_header = FDBBucketHeader {
                    row_header_list_head_addr: bucket
                        .first_row_last
                        .map(|(index, _)| {
                            row_header_list_base
                                + (index * std::mem::size_of::<FDBRowHeaderListEntry>()) as u32
                        })
                        .unwrap_or(0xffffffff),
                };
                out.write_all(&bucket_header.row_header_list_head_addr.to_le_bytes())?;
            }

            let row_count: u32 = table.rows.len().try_into().unwrap();
            let row_header_base = row_header_list_base
                + row_count * std::mem::size_of::<FDBRowHeaderListEntry>() as u32;

            for (index, row) in table.rows.iter().enumerate() {
                let row_list_entry = FDBRowHeaderListEntry {
                    row_header_addr: row_header_base
                        + (index * std::mem::size_of::<FDBRowHeader>()) as u32,
                    row_header_list_next_addr: row
                        .next_row
                        .map(|index| {
                            row_header_list_base
                                + (index * std::mem::size_of::<FDBRowHeaderListEntry>()) as u32
                        })
                        .unwrap_or(0xffffffff),
                };
                out.write_all(&row_list_entry.row_header_addr.to_le_bytes())?;
                out.write_all(&row_list_entry.row_header_list_next_addr.to_le_bytes())?;
            }

            let field_base_offset =
                row_header_base + row_count * std::mem::size_of::<FDBRowHeader>() as u32;

            for row in &table.rows {
                let row_header = FDBRowHeader {
                    fields: ArrayHeader {
                        base_offset: field_base_offset
                            + (row.first_field_index * std::mem::size_of::<FDBFieldData>()) as u32,
                        count: row.count,
                    },
                };
                write_array_header(&row_header.fields, out)?;
            }

            let i64s_base_offset = field_base_offset
                + (table.fields.len() * std::mem::size_of::<FDBFieldData>()) as u32;
            let strings_base_offset =
                i64s_base_offset + (table.i64s.len() * std::mem::size_of::<u64>()) as u32;

            let mut string_len_base = strings_base_offset;
            let mut string_len_offsets = BTreeMap::new();
            for (&key, value) in &table.strings {
                let string_len = key * 4;
                string_len_offsets.insert(key, string_len_base);
                string_len_base += (string_len * value.len()) as u32;
            }

            for field in &table.fields {
                let (key, value) = match field {
                    Field::Nothing => (0, [0; 4]),
                    Field::Integer(i) => (1, i.to_le_bytes()),
                    Field::Float(f) => (3, f.to_le_bytes()),
                    Field::Text { outer, inner } => (4, {
                        let v =
                            string_len_offsets.get(&outer).unwrap() + (inner * outer * 4) as u32;
                        v.to_le_bytes()
                    }),
                    Field::Boolean(b) => (5, if *b { [1, 0, 0, 0] } else { [0; 4] }),
                    Field::BigInt { index } => (6, {
                        let v = i64s_base_offset + (*index * std::mem::size_of::<u64>()) as u32;
                        v.to_le_bytes()
                    }),
                    Field::VarChar { outer, inner } => (8, {
                        let v =
                            string_len_offsets.get(outer).unwrap() + (*inner * *outer * 4) as u32;
                        v.to_le_bytes()
                    }),
                };
                out.write_all(&u32::to_le_bytes(key))?;
                out.write_all(&value)?;
            }

            // Write out all i64s
            for &num in &table.i64s {
                out.write_all(&num.to_le_bytes())?;
            }

            // Write out all strings
            for value in table.strings.values() {
                for string in value {
                    write_str(string, out)?;
                }
            }

            // Increment final offset
            start = string_len_base;
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
    strings: BTreeMap<usize, Vec<Latin1String>>,
    i64s: Vec<i64>,
    buckets: Vec<Bucket>,
    rows: Vec<Row>,
    fields: Vec<Field>,
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
    pub fn push_row(&mut self, pk: usize, fields: &[super::core::Field]) {
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

        for field in fields {
            self.fields.push(match field {
                super::core::Field::Nothing => Field::Nothing,
                super::core::Field::Integer(i) => Field::Integer(*i),
                super::core::Field::Float(f) => Field::Float(*f),
                super::core::Field::Text(string) => {
                    let s = Latin1String::encode(string).into_owned();
                    let lkey = s.req_buf_len();
                    let lstrings = self.strings.entry(lkey).or_default();
                    let inner = lstrings.len();
                    lstrings.push(s);
                    Field::Text { outer: lkey, inner }
                }
                super::core::Field::Boolean(b) => Field::Boolean(*b),
                super::core::Field::BigInt(v) => {
                    let index = self.i64s.len();
                    self.i64s.push(*v);
                    Field::BigInt { index }
                }
                super::core::Field::VarChar(string) => {
                    let s = Latin1String::encode(string).into_owned();
                    let lkey = s.req_buf_len();
                    let lstrings = self.strings.entry(lkey).or_default();
                    let inner = lstrings.len();
                    lstrings.push(s);
                    Field::VarChar { outer: lkey, inner }
                }
            });
        }
    }

    fn compute_def_size(&self, name: &Latin1Str) -> usize {
        std::mem::size_of::<FDBTableDefHeader>()
            + name.req_buf_len() * 4
            + std::mem::size_of::<FDBColumnHeader>() * self.columns.len()
            + self
                .columns
                .iter()
                .map(|c| c.name.req_buf_len())
                .sum::<usize>()
                * 4
    }

    fn compute_data_size(&self) -> usize {
        let string_size: usize = self.strings.iter().map(|(k, v)| k * v.len()).sum(); // Strings
        std::mem::size_of::<FDBTableDataHeader>()
            + std::mem::size_of::<FDBBucketHeader>() * self.buckets.len()
            + std::mem::size_of::<FDBRowHeaderListEntry>() * self.rows.len()
            + std::mem::size_of::<FDBRowHeader>() * self.rows.len()
            + std::mem::size_of::<FDBFieldData>() * self.fields.len()
            + 4 * string_size
            + std::mem::size_of::<u64>() * self.i64s.len()
    }

    fn compute_size(&self, name: &Latin1Str) -> TableSize {
        TableSize {
            def: self.compute_def_size(name),
            data: self.compute_data_size(),
        }
    }
}

/// A single column
struct Column {
    name: Latin1String,
    data_type: ValueType,
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

/// A single field
enum Field {
    /// The `NULL` value
    Nothing,
    /// A 32 bit signed integer
    Integer(i32),
    /// A 32 bit IEEE floating point number
    Float(f32),
    /// A piece of Latin-1 encoded text
    Text {
        /// The length-key of the string
        outer: usize,
        /// The index in the strings array
        inner: usize,
    },
    /// A boolean
    Boolean(bool),
    /// An indirect 64 bit integer
    BigInt {
        /// The offset of the value
        index: usize,
    },
    /// A (base64 encoded?) null-terminated string
    VarChar {
        /// The length-key of the string
        outer: usize,
        /// The index in the strings array
        inner: usize,
    },
}
