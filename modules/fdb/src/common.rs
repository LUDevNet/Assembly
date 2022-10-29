//! # Types that are common to most FDB-APIs
//!
//! This crate module contains rustic representations/types for values that
//! necessarily appear in most of the APIs in this crate.

pub use assembly_fdb_core::value::{Context, UnknownValueType, Value, ValueMapperMut, ValueType};
pub use latin1str::{Latin1Str, Latin1String};

/// Calculates the number of 4-byte units that are needed to store
/// this string with at least one null terminator.
pub fn req_buf_len(s: &Latin1Str) -> usize {
    s.len() / 4 + 1
}
