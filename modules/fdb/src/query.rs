//! ## Query the database
use std::num::ParseIntError;

use assembly_fdb_core::FdbHash;

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

    /// Create an integer PK hash
    pub fn integer(value: i32) -> Self {
        Self {
            hash_value: FdbHash::hash(&value),
            value: Field::Integer(value),
        }
    }

    /// Create an integer PK hash
    pub fn bigint(value: i64) -> Self {
        Self {
            hash_value: FdbHash::hash(&value),
            value: Field::BigInt(value),
        }
    }

    /// Create a text PK hash
    pub fn text(value: String) -> Self {
        Self {
            hash_value: FdbHash::hash(&value),
            value: Field::Text(value),
        }
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
    Ok(PrimaryKeyFilter::text(key))
}

/// Create an integer PK filter
pub fn integer_pk_filter(key: &str) -> Result<PrimaryKeyFilter, PKFilterError> {
    let value: i32 = key.parse().map_err(PKFilterError::KeyError)?;
    Ok(PrimaryKeyFilter::integer(value))
}

/// Create a bigint PK filter
pub fn bigint_pk_filter(key: &str) -> Result<PrimaryKeyFilter, PKFilterError> {
    let value: i64 = key.parse().map_err(PKFilterError::KeyError)?;
    Ok(PrimaryKeyFilter::bigint(value))
}

/// Create a PK filter from a string
pub fn pk_filter<T: Into<String>>(
    key: T,
    field_type: ValueType,
) -> Result<PrimaryKeyFilter, PKFilterError> {
    match field_type {
        ValueType::Text => text_pk_filter(key.into()),
        ValueType::Integer => integer_pk_filter(key.into().as_ref()),
        ValueType::BigInt => bigint_pk_filter(key.into().as_ref()),
        _ => Err(PKFilterError::UnsupportedType(field_type)),
    }
}
