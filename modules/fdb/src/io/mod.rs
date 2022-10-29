//! # Code for use with `std::io::{Read, Write}`
#[cfg(feature = "io-read")]
pub mod read;
#[cfg(feature = "io-write")]
pub mod write;
