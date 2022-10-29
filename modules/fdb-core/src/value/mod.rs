//! # Types that are common to most FDB-APIs
//!
//! This crate module contains rustic representations/types for values that
//! necessarily appear in most of the APIs in this crate.

use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Debug},
};

pub mod mem;
pub mod owned;
pub mod file;

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
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

impl<C: Context> fmt::Display for Value<C>
where
    C::I64: Debug,
    C::String: Debug,
    C::XML: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(f, "NULL"),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(v) => write!(f, "{}", v),
            Self::Text(t) => write!(f, "{:?}", t),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::BigInt(i) => write!(f, "{:?}", i),
            Self::VarChar(v) => write!(f, "{:?}", v),
        }
    }
}

impl<C1: Context, C2: Context> PartialEq<Value<C1>> for Value<C2>
where
    C1::I64: PartialEq<C2::I64>,
    C1::String: PartialEq<C2::String>,
    C1::XML: PartialEq<C2::XML>,
{
    fn eq(&self, other: &Value<C1>) -> bool {
        match other {
            Value::Nothing => matches!(self, Self::Nothing),
            Value::Integer(x) => matches!(self, Self::Integer(y) if x == y),
            Value::Float(x) => matches!(self, Self::Float(y) if x == y),
            Value::Text(x) => matches!(self, Self::Text(y) if x == y),
            Value::Boolean(x) => matches!(self, Self::Boolean(y) if x == y),
            Value::BigInt(x) => matches!(self, Self::BigInt(y) if x == y),
            Value::VarChar(x) => matches!(self, Self::VarChar(y) if x == y),
        }
    }
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

    /// Get the canonical SQLite name of this data type
    pub fn to_sqlite_type(self) -> &'static str {
        match self {
            ValueType::Nothing => "BLOB_NONE",
            ValueType::Integer => "INT32",
            ValueType::Float => "REAL",
            ValueType::Text => "TEXT4",
            ValueType::Boolean => "INT_BOOL",
            ValueType::BigInt => "INT64",
            ValueType::VarChar => "TEXT_XML",
        }
    }

    /// Take an SQLite column declaration type and guess the ValueType
    ///
    /// This function does a proper round-trip with `to_sqlite_type`
    ///
    /// ```
    /// use assembly_fdb_core::value::ValueType;
    ///
    /// fn round_trip(v: ValueType) -> Option<ValueType> {
    ///     ValueType::from_sqlite_type(v.to_sqlite_type())
    /// }
    ///
    /// // Check whether the round-trip works
    /// assert_eq!(round_trip(ValueType::Nothing), Some(ValueType::Nothing));
    /// assert_eq!(round_trip(ValueType::Integer), Some(ValueType::Integer));
    /// assert_eq!(round_trip(ValueType::Float), Some(ValueType::Float));
    /// assert_eq!(round_trip(ValueType::Text), Some(ValueType::Text));
    /// assert_eq!(round_trip(ValueType::Boolean), Some(ValueType::Boolean));
    /// assert_eq!(round_trip(ValueType::BigInt), Some(ValueType::BigInt));
    /// assert_eq!(round_trip(ValueType::VarChar), Some(ValueType::VarChar));
    ///
    /// // Check whether lcdr's names work
    /// assert_eq!(ValueType::from_sqlite_type("none"), Some(ValueType::Nothing));
    /// assert_eq!(ValueType::from_sqlite_type("int32"), Some(ValueType::Integer));
    /// assert_eq!(ValueType::from_sqlite_type("real"), Some(ValueType::Float));
    /// assert_eq!(ValueType::from_sqlite_type("text_4"), Some(ValueType::Text));
    /// assert_eq!(ValueType::from_sqlite_type("int_bool"), Some(ValueType::Boolean));
    /// assert_eq!(ValueType::from_sqlite_type("int64"), Some(ValueType::BigInt));
    /// assert_eq!(ValueType::from_sqlite_type("text_8"), Some(ValueType::VarChar));
    /// ```
    pub fn from_sqlite_type(decl_type: &str) -> Option<Self> {
        match decl_type {
            "BLOB_NONE" | "blob_none" | "none" | "NULL" => Some(ValueType::Nothing),
            "INT32" | "int32" | "TINYINT" | "SMALLINT" => Some(ValueType::Integer),
            "real" | "REAL" | "FLOAT" => Some(ValueType::Float),
            "TEXT4" | "text_4" | "TEXT" => Some(ValueType::Text),
            "BIT" | "INT_BOOL" | "int_bool" => Some(ValueType::Boolean),
            "INT64" | "int64" | "BIGINT" | "INTEGER" => Some(ValueType::BigInt),
            "XML" | "TEXT_XML" | "xml" | "text_8" | "text_xml" | "VARCHAR" => {
                Some(ValueType::VarChar)
            }
            _ => None,
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
