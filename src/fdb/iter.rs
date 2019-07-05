use super::core::{Field,Row,Bucket,Table};

pub type FieldVecIter = std::vec::IntoIter<Field>;

impl IntoIterator for Row {
    type Item = Field;
    type IntoIter = FieldVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fields().into_iter()
    }
}

pub type FieldRefIter<'a> = std::slice::Iter<'a, Field>;

impl<'a> IntoIterator for &'a Row {
    type Item = &'a Field;
    type IntoIter = FieldRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields_ref().into_iter()
    }
}

pub type RowVecIter = ::std::vec::IntoIter<Row>;
pub type RowSliceIter<'a> = std::slice::Iter<'a, Row>;

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
        self.rows_ref().into_iter()
    }
}

pub type TableBucketIter = ::std::vec::IntoIter<Bucket>;
pub type BucketRowIterMapper = fn(Bucket) -> RowVecIter;
pub type TableRowIter = ::std::iter::FlatMap<TableBucketIter,RowVecIter,BucketRowIterMapper>;

impl IntoIterator for Table {
    type Item = Row;
    type IntoIter = TableRowIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets().into_iter().flat_map(Bucket::into_iter)
    }
}

pub type TableBucketRefIter<'a> = std::slice::Iter<'a, Bucket>;
pub type BucketRowRefIterMapper<'a> = fn(&'a Bucket) -> RowSliceIter<'a>;
pub type TableRowRefIter<'a> = ::std::iter::FlatMap<TableBucketRefIter<'a>,RowSliceIter<'a>,BucketRowRefIterMapper<'a>>;

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Row;
    type IntoIter = TableRowRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets_ref().into_iter().flat_map(<(&Bucket)>::into_iter)
    }
}
