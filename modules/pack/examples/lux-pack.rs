//! This tool is used to pre-package PK files given
//! a patcher dir with sd0 files, a trunk manifest file
//! and a package index.
//!
//! It outputs a filtered
use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
/// print the entry for a specific CRC in the PKI
struct Args {
    /// the directory with all files
    #[argh(positional)]
    path: PathBuf,

    /// the directory of the cache (default to current dir)
    #[argh(positional)]
    cache: Option<PathBuf>,

    /// path to trunk.txt, relative to `path`
    #[argh(option, default = "String::from(\"versions/trunk.txt\")")]
    manifest: String,

    /// name of the patcher directory
    #[argh(option, default = "String::from(\"luclient\")")]
    patcherdir: String,
}

fn main() -> color_eyre::Result<()> {
    Ok(())
}
