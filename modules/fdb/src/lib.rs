//! # The database (`*.fdb`) file format used for the core database (`CDClient`)
//!
//! Among the resource files distributed with the LEGO® Universe game client is
//! a copy of the core database. This database includes information on all zones,
//! objects, components, behaviors, items, loot, currency, missions, scripts, …
//!
//! This (unpacked) name of this file is `/res/CDClient.fdb`. The file uses a custom
//! database format which is essentially a sorted list of hash maps.
//!
//! ## Terminology
//!
//! - **Database**: The whole file, a collection of tables
//! - **Table**: A collection of rows, implemented as an array of buckets
//! - **Column**: A name and default type for the fields in every row
//! - **Bucket**: A linked-list of rows for one value of the primary-key hash
//! - **Row**: A list of fields, corresponding to the columns of the table definition
//! - **Field**: A value with a type marker
//!
//! ## File format
//!
//! The file format is constructed from a bunch of structs made out of 32-bit words
//! that may reference other structs by offset from the start of the file. These structs
//! form a tree without circular references.
//!
//! These basic structs are implemented in the [`assembly_fdb_core`] crate.
//!
//! ## Using this library
//!
//! You can use the `mem` module to load a database from an in-memory buffer:
//!
//! ```
//! use assembly_fdb::mem::Database;
//!
//! let file: &[u8] = &[0,0,0,0,8,0,0,0];
//! let db = Database::new(file);
//! let tables = db.tables().unwrap();
//!
//! assert_eq!(0, tables.len());
//! ```

#![doc(html_logo_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![doc(html_favicon_url = "https://assembly.lu-dev.net/rust-logo-lu-256.png")]
#![warn(missing_docs)]

pub mod common;
pub mod core;
pub mod io;
pub mod mem;
pub mod parser;
pub mod query;
#[cfg(feature = "ro")]
pub mod ro;
pub mod store;

mod handle;
mod util;

pub use assembly_fdb_core::FdbHash;

#[cfg(feature = "sqlite")]
pub mod sqlite;
