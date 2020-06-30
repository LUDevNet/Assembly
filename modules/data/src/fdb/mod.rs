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

pub mod builder;
pub mod core;
pub mod de;
pub mod file;
pub mod io;
#[doc(hidden)]
pub mod iter;
pub mod parser;
pub mod query;
pub mod reader;
pub mod store;

pub use self::core::Schema;
