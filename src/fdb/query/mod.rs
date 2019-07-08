//! ## Query the database
use hsieh_hash::digest;
use super::core::{ValueType, Field};


pub struct PrimaryKeyFilter {
    hash_value: u32,
    value: Field,
}

impl PrimaryKeyFilter {
    pub fn hash(&self) -> u32 {
        self.hash_value
    }

    pub fn filter(&self, other: &Field) -> bool {
        &self.value == other
    }
}

#[derive(Debug)]
pub enum PKFilterError {
    UnsupportedType(ValueType),
    KeyError(std::num::ParseIntError),
}

pub fn text_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let hash_value = digest(key.as_bytes());
    let value = Field::Text(key);
    Ok(PrimaryKeyFilter{hash_value, value})
}

pub fn integer_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let value: i32 = key.parse().map_err(PKFilterError::KeyError)?;
    let hash_value = u32::from_ne_bytes(value.to_ne_bytes());
    Ok(PrimaryKeyFilter{hash_value, value: Field::Integer(value)})
}

pub fn pk_filter<T: Into<String>>(key: T, field_type: ValueType) -> Result<PrimaryKeyFilter, PKFilterError> {
    match field_type {
        ValueType::Text => text_pk_filter(key.into()),
        ValueType::Integer => integer_pk_filter(key.into()),
        _ => Err(PKFilterError::UnsupportedType(field_type.clone())),
    }
}
