//! # General-Purpose file loader
//!
//! This is the original entry point to the FDB loading API. A [`SchemaLoader`] wraps
//! an implementation of [`BufRead`] and loads the data from the file into an
//! instance of [`Schema`].
//!
//! This uses the methods defined in the `reader` module and produces the data
//! structure defined in the `core` module.

use super::file::{
    FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBRowHeader, FDBTableDataHeader,
    FDBTableDefHeader, FDBTableHeader,
};
use super::reader::builder::DatabaseBuilder;
use super::reader::{DatabaseBufReader, DatabaseReader};
use super::{common::ValueType, core::*};
use assembly_core::reader::{FileError, FileResult};
use std::convert::TryFrom;
use std::fs;
use std::io::{BufRead, BufReader, Seek};

/// Configuration for the [`SchemaLoader`]
pub trait LoaderConfig {
    /// Whether to process to table specified by `def`
    fn load_table_data(&self, def: &TableDef) -> bool;
}

/// Configuration for SchemaLoader
pub struct LoaderConfigImpl<P>
where
    P: Fn(&TableDef) -> bool,
{
    /// The policy for tables
    pub table_data_policy: P,
}

impl<P> LoaderConfig for LoaderConfigImpl<P>
where
    P: Fn(&TableDef) -> bool,
{
    fn load_table_data(&self, def: &TableDef) -> bool {
        (self.table_data_policy)(def)
    }
}

/// Structure to load a schema from some encapsulated stream
pub struct SchemaLoader<'a, T, C> {
    inner: &'a mut T,
    config: C,
}

impl TryFrom<&str> for Schema {
    type Error = FileError;

    fn try_from(filename: &str) -> FileResult<Schema> {
        let file = fs::File::open(filename)?;
        Schema::try_from(file)
    }
}

impl TryFrom<fs::File> for Schema {
    type Error = FileError;

    fn try_from(file: fs::File) -> FileResult<Schema> {
        let mut reader = BufReader::new(file);
        let config = LoaderConfigImpl {
            table_data_policy: |_| true,
        };
        let mut loader = SchemaLoader::open(&mut reader, config);
        loader.try_load_schema()
    }
}

impl<'a, T, C> SchemaLoader<'a, T, C>
where
    T: BufRead + Seek,
    C: LoaderConfig,
{
    /// Create a new loader from the given reader
    pub fn open(inner: &'a mut T, config: C) -> Self {
        Self { inner, config }
    }

    /// Try to load a row
    pub fn try_load_row(&mut self, header: FDBRowHeader) -> FileResult<Row> {
        let a = &mut self.inner;
        let field_list = a.get_field_data_list(header)?;
        let field_data: Vec<FDBFieldData> = field_list.into();
        let mut fields: Vec<Field> = Vec::with_capacity(field_data.len());
        for field in field_data {
            match self.inner.try_load_field(&field) {
                Ok(value) => fields.push(value),
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(Row::from(fields))
    }

    /// Try to load a bucket
    pub fn try_load_bucket(&mut self, header: FDBBucketHeader) -> FileResult<Bucket> {
        let row_header_addr_it = self
            .inner
            .get_row_header_addr_iterator(header.row_header_list_head_addr);
        let row_header_addr_list = row_header_addr_it.collect::<Result<Vec<_>, _>>()?;
        let mut rows: Vec<Row> = Vec::with_capacity(row_header_addr_list.len());
        for row_header_addr in row_header_addr_list {
            let row_header = self.inner.get_row_header(row_header_addr)?;
            let row = self.try_load_row(row_header)?;
            rows.push(row);
        }
        Ok(Bucket(rows))
    }

    /// Try to load a column
    pub fn try_load_column(&mut self, header: FDBColumnHeader) -> FileResult<Column> {
        // FIXME: remove unwrap
        let col_type = ValueType::try_from(header.column_data_type).unwrap();
        let col_name = self.inner.get_string(header.column_name_addr)?;
        Ok(Column::from((col_name.as_ref(), col_type)))
    }

    /// Try to load a table definition
    pub fn try_load_table_def(&mut self, header: FDBTableDefHeader) -> FileResult<TableDef> {
        let name = self.inner.get_string(header.table_name_addr)?;
        let column_header_list: Vec<FDBColumnHeader> =
            self.inner.get_column_header_list(&header)?.into();

        let columns: Vec<Column> = column_header_list
            .iter()
            .map(|column_header| self.try_load_column(*column_header))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TableDef { columns, name })
    }

    /// Try to load table data
    pub fn try_load_table_data(&mut self, header: FDBTableDataHeader) -> FileResult<TableData> {
        let bucket_header_list: Vec<FDBBucketHeader> =
            self.inner.get_bucket_header_list(&header)?.into();

        let buckets: Vec<Bucket> = bucket_header_list
            .iter()
            .map(|bucket_header| self.try_load_bucket(*bucket_header))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TableData { buckets })
    }

    /// Try to load a table
    pub fn try_load_table(&mut self, header: FDBTableHeader) -> FileResult<Table> {
        let def_header = self
            .inner
            .get_table_def_header(header.table_def_header_addr)?;
        let definition = self.try_load_table_def(def_header)?;
        if self.config.load_table_data(&definition) {
            let data_header = self
                .inner
                .get_table_data_header(header.table_data_header_addr)?;
            let data = self.try_load_table_data(data_header)?;
            Ok(Table::from(definition, data))
        } else {
            Ok(Table::new(definition))
        }
    }

    /// Try to load a schema
    pub fn try_load_schema(&mut self) -> FileResult<Schema> {
        let header = self.inner.get_header()?;
        let table_header_list: Vec<FDBTableHeader> =
            self.inner.get_table_header_list(header)?.into();
        let tables: Vec<Table> = table_header_list
            .iter()
            .map(|table_header| self.try_load_table(*table_header))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Schema::from(tables))
    }
}
