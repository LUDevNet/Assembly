//! # Types that are common to most FDB-APIs
//!
//! This crate module contains rustic representations/types for values that
//! necessarily appear in most of the APIs in this crate.

use std::{
    borrow::{Borrow, Cow},
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

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
/// A borrowed latin-1 encoded string (like `&str`)
pub struct Latin1Str {
    #[allow(dead_code)]
    inner: [u8],
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
}

/// Value datatypes used in the database
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// A short string
    VarChar,
    /// An unknown value
    Unknown(u32),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Nothing => write!(f, "NULL"),
            ValueType::Integer => write!(f, "INTEGER"),
            ValueType::Float => write!(f, "FLOAT"),
            ValueType::Text => write!(f, "TEXT"),
            ValueType::Boolean => write!(f, "BOOLEAN"),
            ValueType::BigInt => write!(f, "BIGINT"),
            ValueType::VarChar => write!(f, "VARCHAR"),
            ValueType::Unknown(i) => write!(f, "UNKNOWN({})", i),
        }
    }
}

impl From<ValueType> for u32 {
    fn from(value_type: ValueType) -> u32 {
        match value_type {
            ValueType::Nothing => 0,
            ValueType::Integer => 1,
            ValueType::Float => 3,
            ValueType::Text => 4,
            ValueType::Boolean => 5,
            ValueType::BigInt => 6,
            ValueType::VarChar => 8,
            ValueType::Unknown(key) => key,
        }
    }
}

impl From<u32> for ValueType {
    fn from(value_type: u32) -> ValueType {
        match value_type {
            0 => ValueType::Nothing,
            1 => ValueType::Integer,
            3 => ValueType::Float,
            4 => ValueType::Text,
            5 => ValueType::Boolean,
            6 => ValueType::BigInt,
            8 => ValueType::VarChar,
            k => ValueType::Unknown(k),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Latin1Str;

    #[test]
    fn test_req_bytes() {
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
