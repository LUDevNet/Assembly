use super::core::{Field,Row,Bucket,Table};

type FieldVecIter = ::std::vec::IntoIter<Field>;

impl IntoIterator for Row {
    type Item = Field;
    type IntoIter = FieldVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fields().into_iter()
    }
}

type RowVecIter = ::std::vec::IntoIter<Row>;

impl IntoIterator for Bucket {
    type Item = Row;
    type IntoIter = RowVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows().into_iter()
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
