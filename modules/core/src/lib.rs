//! # Common datastructures and methods
//!
//! This module implements core traits for this library
#![doc(html_logo_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![doc(html_favicon_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![warn(missing_docs)]
use std::time::Instant;

pub mod borrow;
pub mod buffer;
pub mod ldf;
#[doc(hidden)]
pub mod nom_ext;
pub mod parser;
pub mod reader;
pub mod types;

#[doc(hidden)]
pub extern crate nom;
#[doc(hidden)]
pub use displaydoc;

/// Run the function `run` and print the how much time the execution took.
pub fn time<F, E>(run: F) -> Result<(), E>
where
    F: FnOnce() -> Result<(), E>,
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
