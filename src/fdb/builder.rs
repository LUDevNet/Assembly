use super::super::core::reader::{FileError};
use super::core::{Field, ValueType};
use super::file::FDBFieldData;
use super::reader::{DatabaseBufReader, DatabaseReader};

use std::io::{Seek, BufRead};

#[derive(Debug)]
pub enum BuildError {
    File(FileError),
    UnknownType(u32),
}

pub type BuildResult<T> = Result<T, BuildError>;

impl<T,R> DatabaseBuilder<T> for R
where
    R: DatabaseBufReader<T> + DatabaseReader<T>,
    T: Seek + BufRead {}

pub trait DatabaseBuilder<T>
where T: Seek + BufRead,
Self: DatabaseBufReader<T> + DatabaseReader<T> {
    fn try_load_field(&mut self, data: &FDBFieldData) -> BuildResult<Field> {
        let bytes = data.value;
        match ValueType::from(data.data_type) {
            ValueType::Nothing => Ok(Field::Nothing),
            ValueType::Integer => Ok(bytes)
                .map(i32::from_le_bytes)
                .map(Field::Integer),
            ValueType::Float => Ok(bytes)
                .map(u32::from_le_bytes)
                .map(f32::from_bits)
                .map(Field::Float),
            ValueType::Text => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_string(addr).map_err(BuildError::File))
                .map(Field::Text),
            ValueType::Boolean => Ok(bytes)
                .map(|v| v != [0; 4])
                .map(Field::Boolean),
            ValueType::BigInt => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_i64(addr).map_err(BuildError::File))
                .map(Field::BigInt),
            ValueType::VarChar => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_string(addr).map_err(BuildError::File))
                .map(Field::VarChar),
            ValueType::Unknown(k) =>
                Err(BuildError::UnknownType(k))
        }
    }
}
