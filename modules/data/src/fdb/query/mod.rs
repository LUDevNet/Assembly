//! ## Query the database
use std::num::ParseIntError;

use super::{common::ValueType, core::Field};
use assembly_core::displaydoc::Display;
use hsieh_hash::digest;
use thiserror::Error;

/// A struct that can act as a PK filter
///
/// This structure works much like a pre-implemented closure
/// for use in a `filter` function. It contains the hash of
/// a value and uses that to check whether the fields match.
pub struct PrimaryKeyFilter {
    hash_value: u32,
    value: Field,
}

impl PrimaryKeyFilter {
    /// Get the contained hash
    pub fn hash(&self) -> u32 {
        self.hash_value
    }

    /// Check `other` against the filter
    pub fn filter(&self, other: &Field) -> bool {
        &self.value == other
    }
}

#[derive(Error, Debug, Display)]
/// Errors when creating filters at runtime
pub enum PKFilterError {
    /// Unsupported Type {0:?}
    UnsupportedType(ValueType),
    /// Key Error
    KeyError(#[from] ParseIntError),
}

/// Create a text PK filter
pub fn text_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let hash_value = digest(key.as_bytes());
    let value = Field::Text(key);
    Ok(PrimaryKeyFilter { hash_value, value })
}

/// Create an integer PK filter
pub fn integer_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let value: i32 = key.parse().map_err(PKFilterError::KeyError)?;
    let hash_value = u32::from_ne_bytes(value.to_ne_bytes());
    Ok(PrimaryKeyFilter {
        hash_value,
        value: Field::Integer(value),
    })
}

/// Create a PK filter from a string
pub fn pk_filter<T: Into<String>>(
    key: T,
    field_type: ValueType,
) -> Result<PrimaryKeyFilter, PKFilterError> {
    match field_type {
        ValueType::Text => text_pk_filter(key.into()),
        ValueType::Integer => integer_pk_filter(key.into()),
        _ => Err(PKFilterError::UnsupportedType(field_type)),
    }
}
