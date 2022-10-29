#![allow(clippy::upper_case_acronyms)]
//! The structures, as they are serialized
//!
//! This module contains the low-level structs that make up the FDB file. These
//! structures are annotated with `#[repr(C)]` and can be used to read directly
//! from a memory-mapped file on a little-endian machine.
//!
//! Not all values of these structs are valid for FDB files, but all well-formed
//! FDB-files can be represented by these values. Most importantly, the
//! [`FDBColumnHeader::column_data_type`] only has a limited amount of defined values but
//! covers the whole 32 bits.

// pub mod lists;

pub use assembly_fdb_core::file::*;
pub use assembly_fdb_core::value::file::{FDBFieldValue, FileContext, IndirectValue};
