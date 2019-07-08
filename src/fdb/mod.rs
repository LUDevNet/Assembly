//! # The database (`*.fdb`) file format used for the core database (`CDClient`)
//!
//! The game client is published with a copy of the core game database. This copy
//! resides in `/res/CDClient.fdb` in an unpacked client. The file uses a custom
//! database format which is essentially a list of hash maps.
//!
//! If you just want to load the database from a file, use the following:
//!
//! ```rust,ignore
//! use assembly::fdb::core::Schema;
//!
//! match Schema::try_from("some/path") {
//!     Ok(schema) => {...},
//!     Err(error) => {...},
//! }
//! ```

pub mod core;
/// Implementations for iterators on the data structures.
pub mod iter;
/// Reading of the database file.
pub mod io;
/// The data structures that make up the file.
pub mod file;
/// Parser functions for reading an FDB file.
pub mod parser;
pub mod reader;
pub mod sysdiagram;
pub mod query;
pub mod builder;

pub use self::core::Schema;
