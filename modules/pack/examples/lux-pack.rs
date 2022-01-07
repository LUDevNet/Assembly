//! This tool is used to pre-package PK files given
//! a patcher dir with sd0 files, a trunk manifest file
//! and a package index.
//!
//! It outputs a filtered
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use argh::FromArgs;
use assembly_pack::{
    crc::calculate_crc,
    pk::fs::{PKHandle, PKWriter},
    pki::core::PackIndexFile,
    txt::{FileMeta, Manifest},
};

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

    /// path to primary.pki, relative to `path`
    #[argh(option, default = "String::from(\"versions/primary.pki\")")]
    pki: String,

    /// name of the patcher directory
    #[argh(option, default = "String::from(\"luclient\")")]
    patcherdir: String,
}

struct Writer<'a> {
    path: &'a Path,
}

impl<'a> PKWriter for Writer<'a> {
    fn write<W: std::io::Write>(&mut self, writer: &mut W) -> std::io::Result<()> {
        let file = File::open(self.path)?;
        let mut reader = BufReader::new(file);
        std::io::copy(&mut reader, writer)?;
        Ok(())
    }
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let base = args.path;

    let manifest_path = base.join(&args.manifest);
    println!("manifest: {}", manifest_path.display());
    let manifest = Manifest::from_file(&manifest_path)?;

    let pack_index_path = base.join(&args.pki);
    println!("pack index: {}", pack_index_path.display());
    let pack_index = PackIndexFile::from_file(&pack_index_path)?;

    let cachedir = args
        .cache
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    let patchdir = cachedir.join(args.patcherdir);
    println!("patchdir: {}", patchdir.display());

    let export: HashSet<usize> = pack_index
        .archives
        .iter()
        .enumerate()
        .filter_map(|(index, archive)| {
            if archive.path.contains("front") {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    let mut pack_files = BTreeMap::new();

    for (name, file) in manifest.files {
        let crc = calculate_crc(name.as_bytes());

        if let Some(lookup) = pack_index.files.get(&crc) {
            // File is to be packed
            let pk_id = lookup.pack_file as usize;
            if export.contains(&pk_id) {
                // File is in a pack we want
                let pk = pack_files.entry(pk_id).or_insert_with(|| {
                    let name = &pack_index.archives[pk_id];
                    let path = base.join(&name.path);
                    println!("Opening PK {}", path.display());
                    PKHandle::open(&path).unwrap()
                });

                let is_compressed = lookup.category & 0xFF > 0;
                let raw = FileMeta {
                    size: file.filesize,
                    hash: file.hash,
                };
                let compressed = FileMeta {
                    size: file.compressed_filesize,
                    hash: file.compressed_hash,
                };

                let path = if is_compressed {
                    patchdir.join(file.to_path())
                } else {
                    base.join(name)
                };

                let mut writer = Writer { path: &path };
                pk.put_file(crc, &mut writer, raw, compressed, is_compressed)?;
            }
        }
    }

    for (k, mut pk) in pack_files.into_iter() {
        let path = &pack_index.archives[k].path;
        println!("Closing out PK {}", path);
        pk.finish()?;
    }

    Ok(())
}
