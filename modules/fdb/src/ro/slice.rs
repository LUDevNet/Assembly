#![allow(clippy::upper_case_acronyms)]
//! # Handling of slice references into the in-memory DB file

use assembly_fdb_core::file::{FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBTableHeader};
use std::convert::TryInto;

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

impl<'a> FDBColumnHeaderSlice<'a> {
    /// Get the len of this slice
    pub const fn len(&self) -> usize {
        self.0.len() / std::mem::size_of::<FDBColumnHeader>()
    }

    /// Check whether the slice is empty
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let base = n * std::mem::size_of::<Self::Item>();
        let next = base + std::mem::size_of::<Self::Item>();
        if self.0.len() >= next {
            let (_skipped, start) = self.0.split_at(base);
            let (next, rest) = start.split_at(std::mem::size_of::<Self::Item>());
            self.0 = rest;
            Some(read_bucket_header(next.try_into().unwrap()))
        } else {
            self.0 = self.0.split_at(self.0.len()).1;
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
