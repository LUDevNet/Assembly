#![allow(unused)]
use crate::fdb::core::ValueType;
use crate::fdb::de::slice::Latin1String;
use std::collections::BTreeMap;

pub struct Schema {
    tables: Vec<Table>,
}

impl Schema {
    fn compute_size(&self) -> usize {
        let table_size: usize = self.tables.iter().map(|t| t.compute_size()).sum();
        8 // FDBHeader
        + table_size
    }
}

pub struct Table {
    columns: Vec<Column>,
    strings: BTreeMap<usize, Vec<Latin1String>>,
    u64s: Vec<u64>,
    buckets: Vec<Bucket>,
    rows: Vec<Row>,
    fields: Vec<Field>,
}

impl Table {
    fn compute_size(&self) -> usize {
        let string_size: usize = self.strings.iter().map(|(k, v)| k * v.len()).sum(); // Strings
        12 // FDBTableDefHeader
        + 8 // FDBTableDataHeader
        + 8 * self.columns.len() // FDBColumnHeader
        + 4 * self.buckets.len() // FDBBucketHeader
        + 8 * self.rows.len() // FDBRowHeaderListEntry
        + 8 * self.rows.len() // FDBRowHeader
        + 8 * self.fields.len() // FDBFieldData
        + 4 * string_size
        + 8 * self.u64s.len() // u64s
    }
}

pub struct Column {
    name_str_index: (usize, usize),
    data_type: ValueType,
}

pub struct Bucket {
    first_row: usize,
}

pub struct Row {
    first_field_index: usize,
    next_row: usize,
}

pub struct Field {}
