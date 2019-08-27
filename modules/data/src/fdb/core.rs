//! # The data structures for representing the file/database.
//!
//! An FDB file is layed out as a hash map. The top level is a list
//! of tables, lexically ordered by their name (all uppercase names
//! before all lowercase ones).
//!
//! Each table consists of an array of Buckets, where each bucket
//! Corresponds to one hash value of the primary column.
//!
//! Each bucket consists of a list of rows. These rows may be sorted
//! in ascending order of primary keys, but that is not fully verified.
//!
//! Each row contains a vector of fields, with a data type and respective
//! data.
//!
//! Each Table has a list of columns with the names and default data
//! Types corresponding to the layout of each row.

use std::collections::BTreeMap;
use std::fmt;

/// Value datatypes used in the database
#[derive(Debug, Copy, Clone)]
pub enum ValueType {
    /// The NULL value
    Nothing,
    /// A 32-bit signed integer
    Integer,
    /// A 32-bit IEEE floating point number
    Float,
    /// A long string
    Text,
    /// A boolean
    Boolean,
    /// A 64 bit integer
    BigInt,
    /// A short string
    VarChar,
    /// An unknown value
    Unknown(u32),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Nothing => write!(f, "NULL"),
            ValueType::Integer => write!(f, "INTEGER"),
            ValueType::Float => write!(f, "FLOAT"),
            ValueType::Text => write!(f, "TEXT"),
            ValueType::Boolean => write!(f, "BOOLEAN"),
            ValueType::BigInt => write!(f, "BIGINT"),
            ValueType::VarChar => write!(f, "VARCHAR"),
            ValueType::Unknown(i) => write!(f, "UNKNOWN({})", i),
        }
    }
}

impl From<ValueType> for u32 {
    fn from(value_type: ValueType) -> u32 {
        match value_type {
            ValueType::Nothing => 0,
            ValueType::Integer => 1,
            ValueType::Float => 3,
            ValueType::Text => 4,
            ValueType::Boolean => 5,
            ValueType::BigInt => 6,
            ValueType::VarChar => 8,
            ValueType::Unknown(key) => key,
        }
    }
}

impl From<u32> for ValueType {
    fn from(value_type: u32) -> ValueType {
        match value_type {
            0 => ValueType::Nothing,
            1 => ValueType::Integer,
            3 => ValueType::Float,
            4 => ValueType::Text,
            5 => ValueType::Boolean,
            6 => ValueType::BigInt,
            8 => ValueType::VarChar,
            k => ValueType::Unknown(k),
        }
    }
}

/// A database single field
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Nothing,
    Integer(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    BigInt(i64),
    VarChar(String),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Nothing => write!(f, "NULL"),
            Field::Integer(i) => write!(f, "{}", i),
            Field::Float(v) => write!(f, "{}", v),
            Field::Text(t) => write!(f, "{:?}", t),
            Field::Boolean(b) => write!(f, "{}", b),
            Field::BigInt(i) => write!(f, "{}", i),
            Field::VarChar(v) => write!(f, "{:?}", v),
        }
    }
}

impl From<&Field> for ValueType {
    fn from(val: &Field) -> Self {
        match val {
            Field::Nothing => ValueType::Nothing,
            Field::Integer(_) => ValueType::Integer,
            Field::Float(_) => ValueType::Float,
            Field::Text(_) => ValueType::Text,
            Field::Boolean(_) => ValueType::Boolean,
            Field::BigInt(_) => ValueType::BigInt,
            Field::VarChar(_) => ValueType::VarChar,
        }
    }
}

/// A sequence of fields
#[derive(Debug)]
pub struct Row(Vec<Field>);

impl From<Vec<Field>> for Row {
    fn from(fields: Vec<Field>) -> Self {
        Row(fields)
    }
}

impl Row {
    #[allow(dead_code)]
    pub fn new() -> Row {
        Row(Vec::new())
    }

    pub fn fields(self) -> Vec<Field> {
        self.0
    }

    pub fn fields_ref(&self) -> &Vec<Field> {
        &self.0
    }
}

/// A container of rows with the same hash value
pub struct Bucket(pub Vec<Row>);

impl Bucket {
    /// Create a new empty bucket
    pub fn new() -> Bucket {
        Bucket(Vec::new())
    }

    /// Get the rows of the bucket
    pub fn rows(self) -> Vec<Row> {
        self.0
    }

    /// Get a reference to the rows from a reference to a bucket
    pub fn rows_ref(&self) -> &Vec<Row> {
        &self.0
    }

    /// Get a mutable reference to the rows from a reference to a bucket
    pub fn rows_mut(&mut self) -> &mut Vec<Row> {
        &mut self.0
    }
}

/// Name and default type for one field in each row
#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub field_type: ValueType,
}

impl From<(&str, ValueType)> for Column {
    fn from(data: (&str, ValueType)) -> Self {
        Column { name: String::from(data.0), field_type: data.1 }
    }
}

/// A list of columns with types and a name
pub struct TableDef {
    pub columns: Vec<Column>,
    pub name: String,
}

/// An array of buckets, and a collection of rows
pub struct TableData {
    pub buckets: Vec<Bucket>,
}

impl TableData {
    pub fn new() -> Self {
        TableData{buckets: Vec::new()}
    }
}

/// A list of buckets and thus collection of rows with a name
#[allow(dead_code)]
pub struct Table {
    definition: TableDef,
    data: TableData,
}

impl Table {
    pub fn from(definition: TableDef, data: TableData) -> Self {
        Table { definition, data }
    }

    pub fn new(definition: TableDef) -> Self {
        let data = TableData::new();
        Table { definition, data }
    }

    pub fn buckets(self) -> Vec<Bucket> {
        self.data.buckets
    }

    pub fn buckets_ref(&self) -> &Vec<Bucket> {
        &self.data.buckets
    }

    pub fn columns(self) -> Vec<Column> {
        self.definition.columns
    }

    pub fn columns_ref(&self) -> &Vec<Column> {
        &self.definition.columns
    }

    pub fn name(&self) -> &str {
        self.definition.name.as_ref()
    }
}

/// # An ordered map of tables
///
/// A schema is an ordered map of tables. It represents a full
/// relational database and is the root struct type in this module.
pub struct Schema {
    /// The tables in this schema
    pub tables: BTreeMap<String, Table>,
}

impl Schema {
    /// Create a new empty schema
    pub fn new() -> Schema {
        Schema {
            tables: BTreeMap::new(),
        }
    }

    /// Get a reference to the table of that name it it exists
    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    /// Get a mutable reference to the table of that name it it exists
    pub fn table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}

impl From<Vec<Table>> for Schema {
    fn from(tables: Vec<Table>) -> Self {
        let mut tree = BTreeMap::new();
        for table in tables {
            tree.insert(table.name().to_string(), table);
        }
        Schema { tables: tree }
    }
}
