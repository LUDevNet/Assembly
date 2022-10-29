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

pub mod iter;

use std::collections::BTreeMap;

use latin1str::Latin1Str;

pub use assembly_fdb_core::value::{
    mem::MemContext,
    owned::{Field, OwnedContext},
    Context, Value, ValueMapperMut, ValueType,
};

/// Map [MemContext] values to [OwnedContext] values
pub struct MemToOwned;

impl<'a> ValueMapperMut<MemContext<'a>, OwnedContext> for MemToOwned {
    fn map_string(&mut self, from: &&'a Latin1Str) -> String {
        from.decode().into_owned()
    }

    fn map_i64(&mut self, from: &i64) -> i64 {
        *from
    }

    fn map_xml(&mut self, from: &&'a Latin1Str) -> String {
        from.decode().into_owned()
    }
}

/// A sequence of fields
#[derive(Debug, Default)]
pub struct Row(Vec<Field>);

impl From<Vec<Field>> for Row {
    fn from(fields: Vec<Field>) -> Self {
        Row(fields)
    }
}

impl Row {
    /// Create a new, empty row
    pub fn new() -> Row {
        Row(Vec::new())
    }

    /// Return the fields of this row
    pub fn into_fields(self) -> Vec<Field> {
        self.0
    }

    /// Get a reference to the fields vector
    pub fn fields(&self) -> &Vec<Field> {
        &self.0
    }

    /// Get a mutable reference to the fields vector
    pub fn fields_mut(&mut self) -> &mut Vec<Field> {
        &mut self.0
    }
}

/// A container of rows with the same hash value
#[derive(Debug, Default)]
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
    /// The name of the column
    pub name: String,
    /// The type of the column
    pub field_type: ValueType,
}

impl From<(&str, ValueType)> for Column {
    fn from(data: (&str, ValueType)) -> Self {
        Column {
            name: String::from(data.0),
            field_type: data.1,
        }
    }
}

/// A list of columns with types and a name
#[derive(Debug)]
pub struct TableDef {
    /// The columns of the table in the same order as in the rows
    pub columns: Vec<Column>,
    /// The name of the table
    pub name: String,
}

/// An array of buckets, and a collection of rows
#[derive(Debug, Default)]
pub struct TableData {
    /// The buckets in this table
    pub buckets: Vec<Bucket>,
}

impl TableData {
    /// Creates a new instance
    pub fn new() -> Self {
        TableData {
            buckets: Vec::new(),
        }
    }
}

/// A list of buckets and thus collection of rows with a name
#[derive(Debug)]
pub struct Table {
    definition: TableDef,
    data: TableData,
}

impl Table {
    /// Creates a new table from a definition and data struct
    pub fn from(definition: TableDef, data: TableData) -> Self {
        Table { definition, data }
    }

    /// Creates a new table without data
    pub fn new(definition: TableDef) -> Self {
        let data = TableData::new();
        Table { definition, data }
    }

    /// Extract the buckets vector
    pub fn into_buckets(self) -> Vec<Bucket> {
        self.data.buckets
    }

    /// Returns a reference to the slice of buckets
    pub fn buckets(&self) -> &[Bucket] {
        &self.data.buckets
    }

    /// Returns a mutable reference to the vector of buckets
    pub fn buckets_mut(&mut self) -> &mut Vec<Bucket> {
        &mut self.data.buckets
    }

    /// Extract the columns vector
    pub fn into_columns(self) -> Vec<Column> {
        self.definition.columns
    }

    /// Returns a reference to the slice of columns
    pub fn columns(&self) -> &[Column] {
        &self.definition.columns
    }

    /// Returns a mutable reference to the vector of columns
    pub fn columns_mut(&mut self) -> &mut Vec<Column> {
        &mut self.definition.columns
    }

    /// Returns the name of the table
    pub fn name(&self) -> &str {
        self.definition.name.as_ref()
    }
}

/// # An ordered map of tables
///
/// A schema is an ordered map of tables. It represents a full
/// relational database and is the root struct type in this module.
#[derive(Debug, Default)]
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

    /// Returns the number of tables
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
