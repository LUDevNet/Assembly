//! # The zone/world (`*.luz`) file format
//!
//! This module can be used to read the zone/world file format
//! used in the game LEGO Universe.

/// Data definitions for zone files
pub mod core;
/// Reading of zone files
pub mod io;
/// Parser functions for zone file data
pub mod parser;
/// Module for reading the path data in a zone file
pub mod paths;
