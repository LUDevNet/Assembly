//! ## Query the database
use std::num::ParseIntError;

use super::{
    common::{Context, Value, ValueType},
    core::Field,
};
use displaydoc::Display;
use thiserror::Error;

/// A struct that can act as a PK filter
///
/// This structure works much like a pre-implemented closure
/// for use in a `filter` function. It contains the hash of
/// a value and uses that to check whether the fields match.
#[derive(Debug, Clone)]
pub struct PrimaryKeyFilter {
    hash_value: u32,
    value: Field,
}

impl PrimaryKeyFilter {
    /// Get the contained hash
    pub fn hash(&self) -> u32 {
        self.hash_value
    }

    /// Get the value used for exact comparison
    pub fn original(&self) -> &Field {
        &self.value
    }

    /// Check `other` against the filter
    pub fn filter<C: Context>(&self, other: &Value<C>) -> bool
    where
        Field: PartialEq<Value<C>>,
    {
        &self.value == other
    }
}

#[derive(Error, Debug, Display)]
#[allow(clippy::upper_case_acronyms)]
/// Errors when creating filters at runtime
pub enum PKFilterError {
    /// Unsupported Type {0:?}
    UnsupportedType(ValueType),
    /// Key Error
    KeyError(#[from] ParseIntError),
}

/// Create a text PK filter
pub fn text_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let hash_value = sfhash::digest(key.as_bytes());
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

/// Create a bigint PK filter
pub fn bigint_pk_filter(key: String) -> Result<PrimaryKeyFilter, PKFilterError> {
    let value: i64 = key.parse().map_err(PKFilterError::KeyError)?;
    let hash_value = (u64::from_ne_bytes(value.to_ne_bytes()) % 0x1_0000_0000) as u32;
    Ok(PrimaryKeyFilter {
        hash_value,
        value: Field::BigInt(value),
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
        ValueType::BigInt => bigint_pk_filter(key.into()),
        _ => Err(PKFilterError::UnsupportedType(field_type)),
    }
}
