//! # Implementations of `IntoIterator` for the core model

use super::{Bucket, Field, Row, Table};
use std::{iter::FlatMap, slice::Iter as SliceIter, vec::IntoIter as VecIntoIter};

/// An iterator over a vector of fields in a row.
pub type FieldVecIter = VecIntoIter<Field>;

impl IntoIterator for Row {
    type Item = Field;
    type IntoIter = FieldVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_fields().into_iter()
    }
}

/// An iterator over a slice of fields in a row reference.
pub type FieldRefIter<'a> = SliceIter<'a, Field>;

impl<'a> IntoIterator for &'a Row {
    type Item = &'a Field;
    type IntoIter = FieldRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields().iter()
    }
}

/// An iterator over a vector of rows in a bucket.
pub type RowVecIter = VecIntoIter<Row>;
/// An iterator over a slice of rows in a bucket reference.
pub type RowSliceIter<'a> = SliceIter<'a, Row>;

impl IntoIterator for Bucket {
    type Item = Row;
    type IntoIter = RowVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows().into_iter()
    }
}

impl<'a> IntoIterator for &'a Bucket {
    type Item = &'a Row;
    type IntoIter = RowSliceIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows_ref().iter()
    }
}

/// An iterator over a vector of buckets in a table.
pub type TableBucketIter = VecIntoIter<Bucket>;
/// A static pointer to a function from `Bucket` to a row iterator.
pub type BucketRowIterMapper = fn(Bucket) -> RowVecIter;
/// A flattened iterator over all rows in a table, disregarding buckets.
pub type TableRowIter = FlatMap<TableBucketIter, RowVecIter, BucketRowIterMapper>;

impl IntoIterator for Table {
    type Item = Row;
    type IntoIter = TableRowIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_buckets().into_iter().flat_map(Bucket::into_iter)
    }
}

/// An iterator over a slice of buckets in a table reference.
pub type TableBucketRefIter<'a> = SliceIter<'a, Bucket>;
/// A static pointer to a function from `Bucket` reference to a `Row` reference iterator.
pub type BucketRowRefIterMapper<'a> = fn(&'a Bucket) -> RowSliceIter<'a>;
/// A flattened iterator over all row references in a table, disregarding buckets.
pub type TableRowRefIter<'a> =
    FlatMap<TableBucketRefIter<'a>, RowSliceIter<'a>, BucketRowRefIterMapper<'a>>;

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Row;
    type IntoIter = TableRowRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets().iter().flat_map(<&Bucket>::into_iter)
    }
}
