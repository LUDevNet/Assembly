//! # Common datastructures and methods
//!
//! This module implements core traits for this library

pub mod types;
pub mod parser;
pub mod reader;
pub mod borrow;
pub mod nom_ext;

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate nom;
