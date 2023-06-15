#![cfg(feature = "pki")]
//! # The pack index (`*.pki`) files
//!
//! This module is used to read pack index files, which specify
//! the list of pack files used as well as which pack file
//! a specific file resides in.

pub mod core;
pub mod gen;
pub mod io;
pub mod parser;
pub mod writer;
