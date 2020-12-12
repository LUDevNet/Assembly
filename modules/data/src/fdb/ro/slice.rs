//! # Handling of slice references into the in-memory DB file

use crate::fdb::file::{FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBTableHeader};
use encoding_rs::WINDOWS_1252;
use memchr::memchr;
use std::{
    borrow::{Borrow, Cow},
    convert::TryInto,
    fmt::Debug,
    ops::Deref,
};

#[repr(transparent)]
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

impl Debug for &'_ Latin1Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Invariant: length must always be a multiple of 8 bytes
#[derive(Copy, Clone)]
pub struct FDBTableHeaderSlice<'a>(pub(super) &'a [u8]);

fn read_table_header(buf: &[u8; 8]) -> FDBTableHeader {
    let (a, b) = buf.split_at(4);
    FDBTableHeader {
        table_def_header_addr: u32::from_le_bytes(a.try_into().unwrap()),
        table_data_header_addr: u32::from_le_bytes(b.try_into().unwrap()),
    }
}

impl<'a> Iterator for FDBTableHeaderSlice<'a> {
    type Item = FDBTableHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() >= 8 {
            let (next, rest) = self.0.split_at(8);
            self.0 = rest;
            let header = read_table_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FDBTableHeaderSlice<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len >= 8 {
            let (rest, next) = self.0.split_at(len - 8);
            self.0 = rest;
            let header = read_table_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

/// Invariant: length must always be a multiple of 8 bytes
#[derive(Copy, Clone)]
pub struct FDBColumnHeaderSlice<'a>(pub(super) &'a [u8]);

fn read_column_header(buf: &[u8; 8]) -> FDBColumnHeader {
    let (a, b) = buf.split_at(4);
    FDBColumnHeader {
        column_data_type: u32::from_le_bytes(a.try_into().unwrap()),
        column_name_addr: u32::from_le_bytes(b.try_into().unwrap()),
    }
}

impl<'a> Iterator for FDBColumnHeaderSlice<'a> {
    type Item = FDBColumnHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() >= 8 {
            let (next, rest) = self.0.split_at(8);
            self.0 = rest;
            let header = read_column_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FDBColumnHeaderSlice<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len >= 8 {
            let (rest, next) = self.0.split_at(len - 8);
            self.0 = rest;
            let header = read_column_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

/// Invariant: length must always be a multiple of 4 bytes
#[derive(Copy, Clone)]
pub struct FDBBucketHeaderSlice<'a>(pub(super) &'a [u8]);

fn read_bucket_header(buf: &[u8; 4]) -> FDBBucketHeader {
    FDBBucketHeader {
        row_header_list_head_addr: u32::from_le_bytes(*buf),
    }
}

impl<'a> Iterator for FDBBucketHeaderSlice<'a> {
    type Item = FDBBucketHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() >= 4 {
            let (next, rest) = self.0.split_at(4);
            self.0 = rest;
            let header = read_bucket_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FDBBucketHeaderSlice<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len >= 4 {
            let (rest, next) = self.0.split_at(len - 4);
            self.0 = rest;
            let header = read_bucket_header(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

/// Invariant: length must always be a multiple of 4 bytes
#[derive(Copy, Clone)]
pub struct FDBFieldDataSlice<'a>(pub(super) &'a [u8]);

fn read_field_data(buf: &[u8; 8]) -> FDBFieldData {
    let (a, b) = buf.split_at(4);
    FDBFieldData {
        data_type: u32::from_le_bytes(a.try_into().unwrap()),
        value: b.try_into().unwrap(),
    }
}

impl<'a> Iterator for FDBFieldDataSlice<'a> {
    type Item = FDBFieldData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() >= 8 {
            let (next, rest) = self.0.split_at(8);
            self.0 = rest;
            let header = read_field_data(next.try_into().unwrap());
            Some(header)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FDBFieldDataSlice<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len >= 8 {
            let (rest, next) = self.0.split_at(len - 8);
            self.0 = rest;
            let header = read_field_data(next.try_into().unwrap());
            Some(header)
        } else {
            None
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
