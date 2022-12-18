//! # Datastructures for working with the game packaging

#![doc(html_logo_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![doc(html_favicon_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![warn(missing_docs)]

pub mod common;
pub mod crc;
pub mod md5;
#[cfg(feature = "pk")]
pub mod pk;
#[cfg(feature = "pki")]
pub mod pki;
#[cfg(feature = "sd0")]
pub mod sd0;
pub mod txt;
