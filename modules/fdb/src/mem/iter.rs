//! ## Iterator types
//!
//! This module contains the types that can be used to iterate over the structs in the `assembly_fdb::mem` module.

use super::{
    c::{FDBBucketHeaderC, FDBFieldDataC, FDBRowHeaderC, FDBRowHeaderListEntryC, FDBTableHeaderC},
    get_field, get_row_header_list_entry, map_table_header, Bucket, Field, Row, Table,
};
use crate::ro::{Handle, RefHandle, SliceIterHandle};
use assembly_core::buffer::CastError;

use std::iter::{Flatten, Map};

#[derive(Clone)]
/// Iterator created by [`Tables::iter`][`super::Tables::iter`]
pub struct TableIter<'a> {
    inner: Handle<'a, std::slice::Iter<'a, FDBTableHeaderC>>,
}

impl<'a> TableIter<'a> {
    pub(super) fn new(inner: &RefHandle<'a, [FDBTableHeaderC]>) -> Self {
        Self {
            inner: inner.map_val(<[FDBTableHeaderC]>::iter),
        }
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Result<Table<'a>, CastError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .raw_mut()
            .next()
            .map(|raw| self.inner.wrap(raw))
            .map(map_table_header)
    }
}

fn bucket_rows(b: Bucket) -> RowHeaderIter {
    b.row_iter()
}

type FnBucketToRowIter<'a> = fn(Bucket<'a>) -> RowHeaderIter<'a>;

/// Iterator produced by [`Table::row_iter`]
pub struct TableRowIter<'a> {
    inner: Flatten<Map<BucketIter<'a>, FnBucketToRowIter<'a>>>,
}

impl<'a> TableRowIter<'a> {
    /// Create a new row iter from a bucket iter
    pub fn new(inner: BucketIter<'a>) -> Self {
        Self {
            inner: inner.map(bucket_rows as FnBucketToRowIter<'a>).flatten(),
        }
    }
}

impl<'a> Iterator for TableRowIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Clone)]
/// Iterator produced by [`Table::bucket_iter`]
pub struct BucketIter<'a> {
    inner: Handle<'a, std::slice::Iter<'a, FDBBucketHeaderC>>,
}

impl<'a> BucketIter<'a> {
    /// Create a new bucket iter
    pub fn new(inner: &RefHandle<'a, [FDBBucketHeaderC]>) -> Self {
        Self {
            inner: inner.map_val(<[FDBBucketHeaderC]>::iter),
        }
    }
}

impl<'a> Iterator for BucketIter<'a> {
    type Item = Bucket<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|b| {
                b.map_extract()
                    .map_val(|b| b.row_header_list_head_addr)
                    .map(get_row_header_list_entry)
                    .transpose()
            })
            .map(|inner| Bucket { inner })
    }
}

/// Struct that implements [`Bucket::row_iter`].
pub struct RowHeaderIter<'a> {
    next: Option<RefHandle<'a, FDBRowHeaderListEntryC>>,
}

impl<'a> RowHeaderIter<'a> {
    /// Create a new instance of this iterator
    pub fn new(next: Option<RefHandle<'a, FDBRowHeaderListEntryC>>) -> Self {
        Self { next }
    }
}

impl<'a> Iterator for RowHeaderIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let entry = next.map_extract();
            self.next = entry
                .map_val(|e| e.row_header_list_next_addr)
                .map(get_row_header_list_entry)
                .transpose();

            let row_header = entry
                .map(|b, r| b.cast::<FDBRowHeaderC>(r.row_header_addr))
                .map_extract();
            let fields = row_header
                .map(|b, r| b.cast_slice::<FDBFieldDataC>(r.fields.base_offset, r.fields.count));

            Some(Row::new(fields))
        } else {
            None
        }
    }
}

/// An iterator over fields in a row
pub struct FieldIter<'a> {
    inner: SliceIterHandle<'a, FDBFieldDataC>,
}

impl<'a> FieldIter<'a> {
    pub(crate) fn new(inner: RefHandle<'a, [FDBFieldDataC]>) -> Self {
        Self {
            inner: inner.map_val(<[FDBFieldDataC]>::iter),
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|r| r.map(get_field).into_raw())
    }
}
