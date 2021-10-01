//! # `serde` helper for binary formats
//!
//! Use `#[serde(with = "assembly_data::md5::padded")]` to serialize hashes with 4 trailing
//! NULL bytes in non-human-readable formats.

use serde::{Deserialize, Serialize};

use crate::md5::MD5Sum;

/// Serialize with 4 trailing NULL bytes if not human readable
pub fn serialize<S>(hash: &MD5Sum, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if serializer.is_human_readable() {
        hash.serialize(serializer)
    } else {
        (hash, 0u32).serialize(serializer)
    }
}

/// Deserialize with 4 trailing NULL bytes if not human readable
pub fn deserialize<'de, D>(deserializer: D) -> Result<MD5Sum, D::Error>
where
    D: serde::Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        MD5Sum::deserialize(deserializer)
    } else {
        <(MD5Sum, u32)>::deserialize(deserializer).map(|(hash, _)| hash)
    }
}
