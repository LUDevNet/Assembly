use super::core::{Bucket, Field, Row, Table};
use std::{iter::FlatMap, slice::Iter as SliceIter, vec::IntoIter as VecIntoIter};

pub type FieldVecIter = VecIntoIter<Field>;

impl IntoIterator for Row {
    type Item = Field;
    type IntoIter = FieldVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fields().into_iter()
    }
}

pub type FieldRefIter<'a> = SliceIter<'a, Field>;

impl<'a> IntoIterator for &'a Row {
    type Item = &'a Field;
    type IntoIter = FieldRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields_ref().iter()
    }
}

pub type RowVecIter = VecIntoIter<Row>;
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

pub type TableBucketIter = VecIntoIter<Bucket>;
pub type BucketRowIterMapper = fn(Bucket) -> RowVecIter;
pub type TableRowIter = FlatMap<TableBucketIter, RowVecIter, BucketRowIterMapper>;

impl IntoIterator for Table {
    type Item = Row;
    type IntoIter = TableRowIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets().into_iter().flat_map(Bucket::into_iter)
    }
}

pub type TableBucketRefIter<'a> = SliceIter<'a, Bucket>;
pub type BucketRowRefIterMapper<'a> = fn(&'a Bucket) -> RowSliceIter<'a>;
pub type TableRowRefIter<'a> =
    FlatMap<TableBucketRefIter<'a>, RowSliceIter<'a>, BucketRowRefIterMapper<'a>>;

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Row;
    type IntoIter = TableRowRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets_ref().iter().flat_map(<&Bucket>::into_iter)
    }
}
