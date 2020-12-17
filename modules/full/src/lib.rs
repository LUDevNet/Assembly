//! This crate is a collection of parsers and data types
//! To enable reading of data from LEGO Universe game files.

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
#[cfg(feature = "pack")]
pub use assembly_pack::pk;
#[cfg(feature = "pack")]
pub use assembly_pack::pki;
#[cfg(feature = "pack")]
pub use assembly_pack::sd0;
