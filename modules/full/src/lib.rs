//! This crate is a collection of parsers and data types
//! To enable reading of data from LEGO Universe game files.

#![doc(html_logo_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![doc(html_favicon_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]

#[cfg(feature = "core")]
pub use assembly_core as core;
#[cfg(feature = "data")]
pub use assembly_data::fdb;
#[cfg(feature = "data")]
pub use assembly_data::xml;
#[cfg(feature = "maps")]
pub use assembly_maps::luz;
#[cfg(feature = "maps")]
pub use assembly_maps::lvl;
#[cfg(feature = "pk")]
pub use assembly_pack::pk;
#[cfg(feature = "pki")]
pub use assembly_pack::pki;
#[cfg(feature = "sd0")]
pub use assembly_pack::sd0;
