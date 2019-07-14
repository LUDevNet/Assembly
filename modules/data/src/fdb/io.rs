use std::{fs, io};
use std::io::{Seek, BufRead, BufReader};
use std::convert::TryFrom;
use super::reader::{DatabaseFile, DatabaseBufReader, DatabaseReader, DatabaseLifetimeReader};
use super::builder::{DatabaseBuilder};
use super::core::*;
use super::file::{
    FDBTableHeader,
    FDBTableDefHeader,
    FDBColumnHeader,
    FDBTableDataHeader,
    FDBBucketHeader,
    FDBRowHeader,
    FDBFieldData,
};
use assembly_core::reader::{FileError};
use assembly_core::nom::{Err as NomErr};

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Seek(io::Error),
    Read(io::Error),
    StringEncoding(String),
    Count(std::num::TryFromIntError),
    File(FileError),
    UnknownType(u32),
    Incomplete,
    ParseError,
    ParseFailure,
    NotImplemented,
}

type LoadResult<A> = Result<A, LoadError>;

impl From<NomErr<&[u8]>> for LoadError {
    fn from(e: NomErr<&[u8]>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            NomErr::Incomplete(_) => LoadError::Incomplete,
            NomErr::Error(_) => LoadError::ParseError,
            NomErr::Failure(_) => LoadError::ParseFailure,
        }
    }
}

impl From<FileError> for LoadError {
    fn from(e: FileError) -> LoadError {
        LoadError::File(e)
    }
}

pub trait LoaderConfig {
    fn load_table_data(&self, def: &TableDef) -> bool;
}

/// Configuration for SchemaLoader
pub struct LoaderConfigImpl<P>
where P: Fn(&TableDef) -> bool {
    pub table_data_policy: P,
}

impl<P> LoaderConfig for LoaderConfigImpl<P>
where P: Fn(&TableDef) -> bool {
    fn load_table_data(&self, def: &TableDef) -> bool {
        (self.table_data_policy)(def)
    }
}

/// Structure to load a schema from some encapsulated stream
pub struct SchemaLoader<'a, T, C> {
    inner: DatabaseFile<'a, T>,
    config: C,
}

impl TryFrom<&str> for Schema {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<Schema> {
        let file = fs::File::open(filename).map_err(LoadError::Io)?;
        Schema::try_from(file)
    }
}

impl TryFrom<fs::File> for Schema {
    type Error = LoadError;

    fn try_from(file: fs::File) -> LoadResult<Schema> {
        let mut reader = BufReader::new(file);
        let config = LoaderConfigImpl {
            table_data_policy: |_| true,
        };
        let mut loader = SchemaLoader::open(&mut reader, config);
        loader.try_load_schema()
    }
}

impl<'a,T,C> SchemaLoader<'a,T,C>
where T: BufRead + Seek, C: LoaderConfig {

    /// Create a new loader from the given reader
    pub fn open(inner: &'a mut T, config: C) -> Self {
        let db = DatabaseFile::open(inner);
        Self{inner: db, config}
    }

    /// Try to load a row
    pub fn try_load_row<'b>(&'b mut self, header: FDBRowHeader) -> LoadResult<Row> {
        let a = &mut self.inner;
        let field_list = a.get_field_data_list(header).map_err(LoadError::File)?;
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
    pub fn try_load_bucket<'b>(&'b mut self, header: FDBBucketHeader) -> LoadResult<Bucket> {
        let row_header_addr_it = self.inner
            .get_row_header_addr_iterator(header.row_header_list_head_addr);
        let row_header_addr_list = row_header_addr_it
            .collect::<Result<Vec<_>, _>>().map_err(LoadError::File)?;
        let mut rows: Vec<Row> = Vec::with_capacity(row_header_addr_list.len());
        for row_header_addr in row_header_addr_list {
            let row_header = self.inner.get_row_header(row_header_addr)?;
            let row = self.try_load_row(row_header)?;
            rows.push(row);
        }
        Ok(Bucket(rows))
    }

    /// Try to load a column
    pub fn try_load_column<'b>(&'b mut self, header: FDBColumnHeader) -> LoadResult<Column> {
        let col_type = ValueType::from(header.column_data_type);
        let col_name = self.inner.get_string(header.column_name_addr).map_err(LoadError::File)?;
        Ok(Column::from((col_name.as_ref(), col_type)))
    }

    /// Try to load a table definition
    pub fn try_load_table_def<'b>(&'b mut self, header: FDBTableDefHeader) -> LoadResult<TableDef> {
        let name = self.inner.get_string(header.table_name_addr).map_err(LoadError::File)?;
        let column_header_list: Vec<FDBColumnHeader> = self.inner
            .get_column_header_list(&header).map_err(LoadError::File)?.into();

        let columns: Vec<Column> = column_header_list.iter()
            .map(|column_header| self.try_load_column(*column_header))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TableDef{columns, name})
    }

    /// Try to load table data
    pub fn try_load_table_data<'b>(&'b mut self, header: FDBTableDataHeader) -> LoadResult<TableData> {
        let bucket_header_list: Vec<FDBBucketHeader> = self.inner
            .get_bucket_header_list(&header).map_err(LoadError::File)?.into();

        let buckets: Vec<Bucket> = bucket_header_list.iter()
            .map(|bucket_header| self.try_load_bucket(*bucket_header))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TableData{buckets})
    }

    /// Try to load a table
    pub fn try_load_table<'b>(&'b mut self, header: FDBTableHeader) -> LoadResult<Table> {
        let def_header = self.inner
            .get_table_def_header(header.table_def_header_addr)
            .map_err(LoadError::File)?;
        let definition = self.try_load_table_def(def_header)?;
        if self.config.load_table_data(&definition) {
            let data_header = self.inner
                .get_table_data_header(header.table_data_header_addr)
                .map_err(LoadError::File)?;
            let data = self.try_load_table_data(data_header)?;
            Ok(Table::from(definition, data))
        } else {
            Ok(Table::new(definition))
        }
    }

    /// Try to load a schema
    pub fn try_load_schema<'b>(&'b mut self) -> LoadResult<Schema> {
        let header = self.inner.get_header().map_err(LoadError::File)?;
        let table_header_list: Vec<FDBTableHeader> = self.inner
            .get_table_header_list(header)
            .map_err(LoadError::File)?.into();
        let tables: Vec<Table> = table_header_list.iter()
            .map(|table_header| self.try_load_table(*table_header))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Schema::from(tables))
    }
}
