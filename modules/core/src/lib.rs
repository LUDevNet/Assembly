//! # Common datastructures and methods
//!
//! This module implements core traits for this library
#![warn(missing_docs)]

use anyhow::Result;
use std::time::Instant;

pub mod borrow;
pub mod buffer;
pub mod ldf;
#[doc(hidden)]
pub mod nom_ext;
pub mod parser;
pub mod reader;
pub mod types;

#[macro_use]
#[doc(hidden)]
pub extern crate num_derive;
#[doc(hidden)]
pub extern crate nom;
#[doc(hidden)]
pub use anyhow;
#[doc(hidden)]
pub use byteorder;
#[doc(hidden)]
pub use displaydoc;
//#[doc(hidden)]
//pub use encoding;
#[doc(hidden)]
pub use num_traits;

/// Run the function `run` and print the how much time the execution took.
pub fn time<F>(run: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    let start = Instant::now();
    let res = run();
    let duration = start.elapsed();

    println!(
        "{} in {}.{}s",
        if res.is_ok() { "Finished" } else { "Failed" },
        duration.as_secs(),
        duration.subsec_millis(),
    );

    res
}
