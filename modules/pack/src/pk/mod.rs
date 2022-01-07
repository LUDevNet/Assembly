//! # The pack (`*.pk`) files
//!
//! The game stores most of its resources in custom zip file variants
//! which are called `Pack` files. These contain a sequence of possibly
//! compressed file streams and an index of all contained files streams
//! at the end of the file.
//!
//! Pack files do not contain the actual names of the contained files,
//! but the index is layed out by the CRC value of the respective filenames,
//! so it is trivial to find the correct data for any given filename.
//!
//! When the game was in operation, players could choose between
//! `Full Download`, setting up the complete PKs at load, and `While Playing`,
//! where the resource manager would try to locate a file in the PKs,
//! and load it from the content delivery network if it wasn't found.
//!
//! The file would then be added to the appropriate PK file, as specified in
//! the PKI (Pack-Index) file.

pub mod file;
pub mod fs;
pub mod parser;
pub mod reader;
pub mod writer;
