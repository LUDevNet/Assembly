//! # \[WIP\] Experiment for arena-storage of a database

#![allow(unused)]
use assembly_core::nom::number::complete::u32;

use crate::fdb::core::ValueType;
use crate::fdb::ro::slice::Latin1String;
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    hint::unreachable_unchecked,
    io, unreachable,
};

use super::{
    file::{
        ArrayHeader, FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
    ro::slice::Latin1Str,
};

/// The whole database
pub struct Database {
    tables: Vec<Table>,
}

fn write_array_header<O: io::Write>(array: &ArrayHeader, out: &mut O) -> io::Result<()> {
    out.write_all(&array.count.to_le_bytes())?;
    out.write_all(&array.base_offset.to_le_bytes())?;
    Ok(())
}

fn write_str<O: io::Write>(string: &Latin1Str, out: &mut O) -> io::Result<()> {
    out.write_all(string.as_bytes());
    match string.len() % 4 {
        0 => out.write_all(&[0, 0, 0, 0])?,
        1 => out.write_all(&[0, 0, 0])?,
        2 => out.write_all(&[0, 0])?,
        3 => out.write_all(&[0])?,
        _ => unsafe { unreachable_unchecked() },
    }
    Ok(())
}

impl Database {
    /// Computes the size of the serialized database
    pub fn compute_size(&self) -> usize {
        let table_size: usize = self
            .tables
            .iter()
            .map(|t| t.compute_size())
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
        let len_vec: Vec<_> = self.tables.iter().map(Table::compute_size).collect();
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
        for table in &self.tables {
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

            let mut column_name_addr = table_name_addr + (table.name.req_buf_len() as u32 * 4);
            for column in &table.columns {
                let column_header = FDBColumnHeader {
                    column_data_type: column.data_type.into(),
                    column_name_addr,
                };
                out.write_all(&column_header.column_data_type.to_le_bytes())?;
                out.write_all(&column_header.column_name_addr.to_le_bytes())?;
                column_name_addr += column.name.req_buf_len() as u32 * 4;
            }

            write_str(&table.name, out)?;
            for column in &table.columns {
                write_str(&column.name, out)?;
            }

            // Serialize table data
            let base_offset = column_name_addr + std::mem::size_of::<FDBTableDataHeader>() as u32;
            let table_data_header = FDBTableDataHeader {
                buckets: ArrayHeader {
                    count: 0,
                    base_offset,
                },
            };
            write_array_header(&table_data_header.buckets, out)?;

            // Increment final offset
            start = base_offset;
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
    name: Latin1String,
    columns: Vec<Column>,
    strings: BTreeMap<usize, Vec<Latin1String>>,
    u64s: Vec<u64>,
    buckets: Vec<Bucket>,
    rows: Vec<Row>,
    fields: Vec<Field>,
}

impl Table {
    /// Creates a new table
    pub fn new(name: impl Into<Latin1String>) -> Self {
        Table {
            buckets: vec![],
            columns: vec![],
            fields: vec![],
            name: name.into(),
            strings: BTreeMap::new(),
            rows: vec![],
            u64s: vec![],
        }
    }

    /// Add a column to this table
    pub fn push_column(&mut self, data_type: ValueType, name: impl Into<Latin1String>) {
        self.columns.push(Column {
            data_type,
            name: name.into(),
        })
    }

    fn compute_def_size(&self) -> usize {
        std::mem::size_of::<FDBTableDefHeader>()
            + self.name.req_buf_len() * 4
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
            + std::mem::size_of::<u64>() * self.u64s.len()
    }

    fn compute_size(&self) -> TableSize {
        TableSize {
            def: self.compute_def_size(),
            data: self.compute_data_size(),
        }
    }
}

/// A single column
pub struct Column {
    name: Latin1String,
    data_type: ValueType,
}

/// A single bucket
pub struct Bucket {
    first_row: usize,
}

/// A single row
pub struct Row {
    first_field_index: usize,
    next_row: usize,
}

/// A single field
pub struct Field {}

#[cfg(test)]
mod tests {
    use crate::fdb::mem;

    use super::*;

    #[test]
    fn test_write_empty() {
        let mut out = Vec::new();
        let db = Database { tables: vec![] };
        db.write(&mut out).unwrap();
        let cmp: &[u8] = &[0, 0, 0, 0, 8, 0, 0, 0];
        assert_eq!(&out[..], cmp);
    }

    #[test]
    fn test_write_table_without_columns() {
        let mut out = Vec::new();
        let db = Database {
            tables: vec![Table::new(Latin1String::encode("Foobar"))],
        };
        db.write(&mut out).unwrap();
        let cmp: &'static [u8] = &[
            1, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
            16, 0, 0, 0, 36, 0, 0, 0, // FDBTableHeader
            0, 0, 0, 0, 28, 0, 0, 0, 28, 0, 0, 0, // FDBTableDefHeader
            b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // Name
            0, 0, 0, 0, 44, 0, 0, 0, // FDBTableDataHeader
        ];
        assert_eq!(&out[..], cmp);

        let odb = mem::Database::new(&out);
        let otb = odb.tables().unwrap();
        assert_eq!(otb.len(), 1);
        let foobar = otb.get(0).expect("table #0").expect("table load");
        assert_eq!(foobar.column_count(), 0);
        assert_eq!(foobar.name(), "Foobar");
    }

    #[test]
    fn test_write_table_with_columns() {
        let mut out = Vec::new();
        let mut table = Table::new(Latin1String::encode("Foobar"));
        table.push_column(ValueType::Integer, Latin1String::encode("foo"));
        let db = Database {
            tables: vec![table],
        };
        db.write(&mut out).unwrap();
        let cmp: &'static [u8] = &[
            1, 0, 0, 0, 8, 0, 0, 0, // FDBHeader
            16, 0, 0, 0, 48, 0, 0, 0, // FDBTableHeader
            1, 0, 0, 0, 36, 0, 0, 0, 28, 0, 0, 0, // FDBTableDefHeader
            1, 0, 0, 0, 44, 0, 0, 0, // FDBColumnHeader
            b'F', b'o', b'o', b'b', b'a', b'r', 0, 0, // table `Foobar`
            b'f', b'o', b'o', 0, // column `Foobar`.`foo`
            0, 0, 0, 0, 56, 0, 0, 0, // FDBTableDataHeader
        ];
        assert_eq!(&out[..], cmp);

        let odb = mem::Database::new(&out);
        let otb = odb.tables().unwrap();
        assert_eq!(otb.len(), 1);
        let foobar = otb.get(0).expect("table #0").expect("table load");
        assert_eq!(foobar.name(), "Foobar");
        let columns: Vec<_> = foobar.column_iter().collect();
        assert_eq!(columns.len(), 1);
        assert_eq!(columns[0].name(), "foo");
    }
}
