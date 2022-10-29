//! # Types that are common to most FDB-APIs
//!
//! This crate module contains rustic representations/types for values that
//! necessarily appear in most of the APIs in this crate.

use std::{convert::TryFrom, error::Error, fmt};

pub use latin1str::{Latin1Str, Latin1String};

/// Calculates the number of 4-byte units that are needed to store
/// this string with at least one null terminator.
pub fn req_buf_len(s: &Latin1Str) -> usize {
    s.len() / 4 + 1
}

/// Hash the string using [`sfhash`]
pub fn str_hash(s: &Latin1Str) -> u32 {
    sfhash::digest(s.as_bytes())
}

/// Type-Parameters to [`Value`]
///
/// This trait is used to parameterize `Value` to produce the concrete types
/// that are used elsewhere in this crate.
pub trait Context {
    /// The type that holds a `ValueType::String`
    type String;
    /// The type that holds a `ValueType::BigInt`
    type I64;
    /// The type that holds a `ValueType::VarChar`
    type XML;
}

/// Trait for mapping value from one context to another
///
/// This traits allows us to implement a generic [`Value::map`] function
/// that works similar to three [`FnMut`] closures but can guarantee that
/// only one of them ever borrows `Self` mutably at the same time.
pub trait ValueMapperMut<TI, TO>
where
    TI: Context,
    TO: Context,
{
    /// Called when mapping a string
    fn map_string(&mut self, from: &TI::String) -> TO::String;
    /// Called when mapping an i64
    fn map_i64(&mut self, from: &TI::I64) -> TO::I64;
    /// Called when mapping an XML value
    fn map_xml(&mut self, from: &TI::XML) -> TO::XML;
}

/// A single field value in the database
///
/// This is a generic enum that is the template for all
/// other `Field` types in this crate.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde-derives", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-derives", serde(untagged))]
pub enum Value<T: Context> {
    /// The NULL value
    Nothing,
    /// A 32 bit integer
    Integer(i32),
    /// A 32 bit IEEE floating point number
    Float(f32),
    /// A string
    Text(T::String),
    /// A boolean
    Boolean(bool),
    /// A 64 bit integer
    BigInt(T::I64),
    /// A (XML?) string
    VarChar(T::XML),
}

impl<T: Context> Clone for Value<T>
where
    T::String: Clone,
    T::XML: Clone,
    T::I64: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Value::Nothing => Value::Nothing,
            Value::Integer(v) => Value::Integer(*v),
            Value::Float(v) => Value::Float(*v),
            Value::Text(v) => Value::Text(v.clone()),
            Value::Boolean(v) => Value::Boolean(*v),
            Value::BigInt(v) => Value::BigInt(v.clone()),
            Value::VarChar(v) => Value::VarChar(v.clone()),
        }
    }
}

impl<T: Context> Copy for Value<T>
where
    T::String: Copy,
    T::XML: Copy,
    T::I64: Copy,
{
}

impl<T: Context> Value<T> {
    /// Creates a value of a different context using the given mapper
    pub fn map<O, M>(&self, mapper: &mut M) -> Value<O>
    where
        O: Context,
        M: ValueMapperMut<T, O>,
    {
        match self {
            Value::Nothing => Value::Nothing,
            Value::Integer(v) => Value::Integer(*v),
            Value::Float(v) => Value::Float(*v),
            Value::Text(v) => Value::Text(mapper.map_string(v)),
            Value::Boolean(v) => Value::Boolean(*v),
            Value::BigInt(v) => Value::BigInt(mapper.map_i64(v)),
            Value::VarChar(v) => Value::VarChar(mapper.map_xml(v)),
        }
    }

    /// Returns `Some` with the value if the field contains an [`Value::Integer`].
    pub fn into_opt_integer(self) -> Option<i32> {
        if let Self::Integer(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some` with the value if the field contains a [`Value::Float`].
    pub fn into_opt_float(self) -> Option<f32> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some` with the value if the field contains a [`Value::Text`].
    pub fn into_opt_text(self) -> Option<T::String> {
        if let Self::Text(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some` with the value if the field contains a [`Value::Boolean`].
    pub fn into_opt_boolean(self) -> Option<bool> {
        if let Self::Boolean(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some` with the value if the field contains a [`Value::BigInt`].
    pub fn into_opt_big_int(self) -> Option<T::I64> {
        if let Self::BigInt(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some` with the value if the field contains a [`Value::VarChar`].
    pub fn into_opt_varchar(self) -> Option<T::XML> {
        if let Self::VarChar(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Context> From<&Value<T>> for ValueType {
    fn from(val: &Value<T>) -> Self {
        match val {
            Value::Nothing => ValueType::Nothing,
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::Text(_) => ValueType::Text,
            Value::Boolean(_) => ValueType::Boolean,
            Value::BigInt(_) => ValueType::BigInt,
            Value::VarChar(_) => ValueType::VarChar,
        }
    }
}

/// Value datatypes used in the database
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde-derives", derive(serde::Serialize))]
pub enum ValueType {
    /// The NULL value
    Nothing,
    /// A 32-bit signed integer
    Integer,
    /// A 32-bit IEEE floating point number
    Float,
    /// A long string
    Text,
    /// A boolean
    Boolean,
    /// A 64 bit integer
    BigInt,
    /// An (XML?) string
    VarChar,
}

impl ValueType {
    /// Get a static name for the type
    pub fn static_name(&self) -> &'static str {
        match self {
            ValueType::Nothing => "NULL",
            ValueType::Integer => "INTEGER",
            ValueType::Float => "FLOAT",
            ValueType::Text => "TEXT",
            ValueType::Boolean => "BOOLEAN",
            ValueType::BigInt => "BIGINT",
            ValueType::VarChar => "VARCHAR",
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.static_name())
    }
}

impl From<ValueType> for u8 {
    fn from(value_type: ValueType) -> u8 {
        match value_type {
            ValueType::Nothing => 0,
            ValueType::Integer => 1,
            ValueType::Float => 3,
            ValueType::Text => 4,
            ValueType::Boolean => 5,
            ValueType::BigInt => 6,
            ValueType::VarChar => 8,
        }
    }
}

impl From<ValueType> for u32 {
    fn from(value_type: ValueType) -> u32 {
        u8::from(value_type).into()
    }
}

/// This represents a value type that could not be parsed
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownValueType(u32);

impl UnknownValueType {
    /// Get the value that could not be interpreted
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Error for UnknownValueType {}
impl fmt::Display for UnknownValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown FDB value type {}", self.0)
    }
}

impl TryFrom<u32> for ValueType {
    type Error = UnknownValueType;

    fn try_from(value_type: u32) -> Result<ValueType, Self::Error> {
        match value_type {
            0 => Ok(ValueType::Nothing),
            1 => Ok(ValueType::Integer),
            3 => Ok(ValueType::Float),
            4 => Ok(ValueType::Text),
            5 => Ok(ValueType::Boolean),
            6 => Ok(ValueType::BigInt),
            8 => Ok(ValueType::VarChar),
            _ => Err(UnknownValueType(value_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use latin1str::Latin1Str;

    use super::req_buf_len;

    #[test]
    fn test_latin1_req_bytes() {
        assert_eq!(1, req_buf_len(Latin1Str::from_bytes_until_nul(b"a")));
        assert_eq!(1, req_buf_len(Latin1Str::from_bytes_until_nul(b"ab")));
        assert_eq!(1, req_buf_len(Latin1Str::from_bytes_until_nul(b"abc")));
        assert_eq!(2, req_buf_len(Latin1Str::from_bytes_until_nul(b"abcd")));
        assert_eq!(2, req_buf_len(Latin1Str::from_bytes_until_nul(b"abcde")));
        assert_eq!(2, req_buf_len(Latin1Str::from_bytes_until_nul(b"abcdef")));
        assert_eq!(2, req_buf_len(Latin1Str::from_bytes_until_nul(b"abcdefg")));
        assert_eq!(3, req_buf_len(Latin1Str::from_bytes_until_nul(b"abcdefgh")));
    }
}
