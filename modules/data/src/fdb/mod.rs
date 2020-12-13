//! # The database (`*.fdb`) file format used for the core database (`CDClient`)
//!
//! The game client is published with a copy of the core game database. This copy
//! resides in `/res/CDClient.fdb` in an unpacked client. The file uses a custom
//! database format which is essentially a list of hash maps.
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
//! ## Using this library
//!
//! You can use the `mem` module to load a database from an in-memory buffer:
//!
//! ```
//! use assembly_data::fdb::mem::Database;
//!
//! let file: &[u8] = &[0,0,0,0,8,0,0,0];
//! let db = Database::new(file);
//! let tables = db.tables().unwrap();
//!
//! assert_eq!(0, tables.len());
//! ```

#![warn(missing_docs)]

pub mod core;
pub mod file;
pub mod io;
pub mod mem;
pub mod parser;
pub mod query;
pub mod reader;
pub mod ro;
pub mod store;

pub use self::core::Schema;
