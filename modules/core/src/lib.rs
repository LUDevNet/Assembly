//! # Common datastructures and methods
//!
//! This module implements core traits for this library

pub mod types;
pub mod parser;
pub mod reader;
pub mod borrow;
#[doc(hidden)]
pub mod nom_ext;

#[macro_use]
#[doc(hidden)]
pub extern crate num_derive;
#[doc(hidden)]
#[macro_use]
pub extern crate nom;
#[doc(hidden)]
pub use encoding;
#[doc(hidden)]
pub use num_traits;
