use super::core::{Field, ValueType};
use super::file::FDBFieldData;
use super::reader::{DatabaseBufReader, DatabaseReader};
use assembly_core::anyhow;
use thiserror::Error;

use std::io::{BufRead, Seek};

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Unknown Type ID {0}")]
    UnknownType(u32),
}

pub type BuildResult<T> = Result<T, anyhow::Error>;

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
                .map(Field::Text),
            ValueType::Boolean => Ok(bytes).map(|v| v != [0; 4]).map(Field::Boolean),
            ValueType::BigInt => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_i64(addr))
                .map(Field::BigInt),
            ValueType::VarChar => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| self.get_string(addr))
                .map(Field::VarChar),
            ValueType::Unknown(k) => Err(BuildError::UnknownType(k).into()),
        }
    }
}
