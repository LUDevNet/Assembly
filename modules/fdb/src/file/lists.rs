#![allow(clippy::upper_case_acronyms)]
//! # Vectors of file structs
//!
//! This module contains newtype wrappers around vectors of the `file` module.

use super::{FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBRowHeader, FDBTableHeader};

#[derive(Debug)]
/// A vector of [`FDBTableHeader`]
pub struct FDBTableHeaderList(pub Vec<FDBTableHeader>);

#[derive(Debug)]
/// A vector of [`FDBColumnHeader`]
pub struct FDBColumnHeaderList(pub Vec<FDBColumnHeader>);

#[derive(Debug)]
/// A vector of [`FDBBucketHeader`]
pub struct FDBBucketHeaderList(pub Vec<FDBBucketHeader>);

#[derive(Debug)]
/// A vector of [`FDBRowHeader`]
pub struct FDBRowHeaderList(pub Vec<FDBRowHeader>);

#[derive(Debug)]
/// A vector of [`FDBFieldData`]
pub struct FDBFieldDataList(pub Vec<FDBFieldData>);

impl From<FDBTableHeaderList> for Vec<FDBTableHeader> {
    fn from(list: FDBTableHeaderList) -> Self {
        list.0
    }
}

impl From<Vec<FDBTableHeader>> for FDBTableHeaderList {
    fn from(vec: Vec<FDBTableHeader>) -> Self {
        FDBTableHeaderList(vec)
    }
}

impl From<FDBColumnHeaderList> for Vec<FDBColumnHeader> {
    fn from(list: FDBColumnHeaderList) -> Self {
        list.0
    }
}

impl From<Vec<FDBColumnHeader>> for FDBColumnHeaderList {
    fn from(vec: Vec<FDBColumnHeader>) -> Self {
        FDBColumnHeaderList(vec)
    }
}

impl From<FDBBucketHeaderList> for Vec<FDBBucketHeader> {
    fn from(list: FDBBucketHeaderList) -> Vec<FDBBucketHeader> {
        list.0
    }
}

impl From<Vec<FDBBucketHeader>> for FDBBucketHeaderList {
    fn from(vec: Vec<FDBBucketHeader>) -> Self {
        FDBBucketHeaderList(vec)
    }
}

impl IntoIterator for FDBBucketHeaderList {
    type Item = FDBBucketHeader;
    type IntoIter = std::vec::IntoIter<FDBBucketHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<FDBRowHeaderList> for Vec<FDBRowHeader> {
    fn from(list: FDBRowHeaderList) -> Vec<FDBRowHeader> {
        list.0
    }
}

impl From<Vec<FDBRowHeader>> for FDBRowHeaderList {
    fn from(vec: Vec<FDBRowHeader>) -> Self {
        FDBRowHeaderList(vec)
    }
}

impl IntoIterator for FDBRowHeaderList {
    type Item = FDBRowHeader;
    type IntoIter = std::vec::IntoIter<FDBRowHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Implementations FDBFieldDataList
impl From<FDBFieldDataList> for Vec<FDBFieldData> {
    fn from(list: FDBFieldDataList) -> Vec<FDBFieldData> {
        list.0
    }
}

impl From<Vec<FDBFieldData>> for FDBFieldDataList {
    fn from(vec: Vec<FDBFieldData>) -> Self {
        FDBFieldDataList(vec)
    }
}

impl IntoIterator for FDBFieldDataList {
    type Item = FDBFieldData;
    type IntoIter = std::vec::IntoIter<FDBFieldData>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
