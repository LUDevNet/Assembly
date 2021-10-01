//! # The segmented (`*.sd0`) compression format
//!
//! This format is used to deflate (zlib) the data
//! served from the server to the client, and to
//! use less space in the pack archives.

pub mod read;
