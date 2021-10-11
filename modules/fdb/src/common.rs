//! # Types that are common to most FDB-APIs
//!
//! This crate module contains rustic representations/types for values that
//! necessarily appear in most of the APIs in this crate.

use std::{
    borrow::{Borrow, Cow},
    convert::TryFrom,
    error::Error,
    fmt,
    ops::Deref,
};

use encoding_rs::WINDOWS_1252;
use memchr::memchr;

#[repr(transparent)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
/// An owned latin-1 encoded string
pub struct Latin1String {
    inner: Box<[u8]>,
}

impl Latin1String {
    /// Create a new string
    ///
    /// ## Safety
    ///
    /// Must not contain null bytes
    pub unsafe fn new(inner: Box<[u8]>) -> Self {
        Self { inner }
    }

    /// Create a new instance from a rust string.
    ///
    /// **Note**: This encodes any unavailable unicode codepoints as their equivalent HTML-Entity.
    /// This is an implementation detail of the `encoding_rs` crate and not really useful for this crate.
    pub fn encode(string: &str) -> Cow<Latin1Str> {
        let (res, _enc, _has_replaced_chars) = WINDOWS_1252.encode(string);
        match res {
            Cow::Owned(o) => Cow::Owned(Self {
                inner: o.into_boxed_slice(),
            }),
            Cow::Borrowed(b) => Cow::Borrowed(unsafe { Latin1Str::from_bytes_unchecked(b) }),
        }
    }
}

impl Borrow<Latin1Str> for Latin1String {
    fn borrow(&self) -> &Latin1Str {
        unsafe { Latin1Str::from_bytes_unchecked(&self.inner) }
    }
}

impl Deref for Latin1String {
    type Target = Latin1Str;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl From<Cow<'_, Latin1Str>> for Latin1String {
    fn from(cow: Cow<'_, Latin1Str>) -> Self {
        cow.into_owned()
    }
}

impl From<&Latin1Str> for Latin1String {
    fn from(src: &Latin1Str) -> Latin1String {
        src.to_owned()
    }
}

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
/// A borrowed latin-1 encoded string (like `&str`)
pub struct Latin1Str {
    #[allow(dead_code)]
    inner: [u8],
}

#[cfg(feature = "serde-derives")]
impl serde::Serialize for Latin1Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.decode().as_ref())
    }
}

impl fmt::Debug for &'_ Latin1Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decode().fmt(f)
    }
}

impl ToOwned for Latin1Str {
    type Owned = Latin1String;

    fn to_owned(&self) -> Self::Owned {
        Latin1String {
            inner: self.as_bytes().into(),
        }
    }
}

impl Latin1Str {
    /// Takes all bytes until before the first null byte or end of slice.
    pub(super) fn new(bytes: &[u8]) -> &Self {
        let text = if let Some(index) = memchr(0x00, bytes) {
            bytes.split_at(index).0
        } else {
            bytes
        };
        unsafe { Self::from_bytes_unchecked(text) }
    }

    /// Turns some bytes into a Latin1Str slice
    ///
    /// ## Safety
    ///
    /// The byte slice may not contain any null bytes
    pub unsafe fn from_bytes_unchecked(text: &[u8]) -> &Self {
        &*(text as *const [u8] as *const Latin1Str)
    }

    /// Get the bytes of the string
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /// Get the bytes of the string
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check whether the str is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Calculates the number of 4-byte units that are needed to store
    /// this string with at least one null terminator.
    pub fn req_buf_len(&self) -> usize {
        self.inner.len() / 4 + 1
    }

    /// Decode the string
    pub fn decode(&self) -> Cow<str> {
        WINDOWS_1252.decode(self.as_bytes()).0
    }

    /// Hash the string using [`sfhash`]
    pub fn hash(&self) -> u32 {
        sfhash::digest(self.as_bytes())
    }
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
    use super::Latin1Str;

    #[test]
    fn test_latin1_req_bytes() {
        assert_eq!(1, Latin1Str::new(b"a").req_buf_len());
        assert_eq!(1, Latin1Str::new(b"ab").req_buf_len());
        assert_eq!(1, Latin1Str::new(b"abc").req_buf_len());
        assert_eq!(2, Latin1Str::new(b"abcd").req_buf_len());
        assert_eq!(2, Latin1Str::new(b"abcde").req_buf_len());
        assert_eq!(2, Latin1Str::new(b"abcdef").req_buf_len());
        assert_eq!(2, Latin1Str::new(b"abcdefg").req_buf_len());
        assert_eq!(3, Latin1Str::new(b"abcdefgh").req_buf_len());
    }
}
