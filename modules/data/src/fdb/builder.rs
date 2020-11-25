use super::core::{Field, ValueType};
use super::file::FDBFieldData;
use super::reader::{DatabaseBufReader, DatabaseReader};
use assembly_core::displaydoc::Display;
use thiserror::Error;

use std::io::{self, BufRead, Seek};

#[derive(Debug, Error, Display)]
pub enum BuildError {
    /// Unknown Type ID {0}
    UnknownType(u32),
    /// IO Error
    IO(#[from] io::Error),
}

pub type BuildResult<T> = Result<T, BuildError>;

impl<T: ?Sized> DatabaseBuilder for T where T: DatabaseBufReader + DatabaseReader + Seek + BufRead {}

pub trait DatabaseBuilder
where
    Self: Seek + BufRead + DatabaseBufReader + DatabaseReader,
{
    fn try_load_field(&mut self, data: &FDBFieldData) -> BuildResult<Field> {
        let bytes = data.value;
        match ValueType::from(data.data_type) {
            ValueType::Nothing => Ok(Field::Nothing),
            ValueType::Integer => Ok(bytes).map(i32::from_le_bytes).map(Field::Integer),
            ValueType::Float => Ok(bytes)
                .map(u32::from_le_bytes)
                .map(f32::from_bits)
                .map(Field::Float),
            ValueType::Text => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_string(addr))
                .map(Field::Text)
                .map_err(Into::into),
            ValueType::Boolean => Ok(bytes).map(|v| v != [0; 4]).map(Field::Boolean),
            ValueType::BigInt => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_i64(addr))
                .map(Field::BigInt)
                .map_err(Into::into),
            ValueType::VarChar => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_string(addr))
                .map(Field::VarChar)
                .map_err(Into::into),
            ValueType::Unknown(k) => Err(BuildError::UnknownType(k)),
        }
    }
}
