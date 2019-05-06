use super::core::{Field,Row,Bucket,Table};

type FieldVecIter = std::vec::IntoIter<Field>;

impl IntoIterator for Row {
    type Item = Field;
    type IntoIter = FieldVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fields().into_iter()
    }
}

type FieldRefIter<'a> = std::slice::Iter<'a, Field>;

impl<'a> IntoIterator for &'a Row {
    type Item = &'a Field;
    type IntoIter = FieldRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields_ref().into_iter()
    }
}

type RowVecIter = ::std::vec::IntoIter<Row>;
type RowSliceIter<'a> = std::slice::Iter<'a, Row>;

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

type TableBucketIter = ::std::vec::IntoIter<Bucket>;
type BucketRowIterMapper = fn(Bucket) -> RowVecIter;
type TableRowIter = ::std::iter::FlatMap<TableBucketIter,RowVecIter,BucketRowIterMapper>;

impl IntoIterator for Table {
    type Item = Row;
    type IntoIter = TableRowIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets().into_iter().flat_map(Bucket::into_iter)
    }
}

type TableBucketRefIter<'a> = std::slice::Iter<'a, Bucket>;
type BucketRowRefIterMapper<'a> = fn(&'a Bucket) -> RowSliceIter<'a>;
type TableRowRefIter<'a> = ::std::iter::FlatMap<TableBucketRefIter<'a>,RowSliceIter<'a>,BucketRowRefIterMapper<'a>>;

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Row;
    type IntoIter = TableRowRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets_ref().into_iter().flat_map(<(&Bucket)>::into_iter)
    }
}
