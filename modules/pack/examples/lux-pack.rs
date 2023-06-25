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
    txt::manifest::Manifest,
};

#[derive(FromArgs)]
/// print the entry for a specific CRC in the PKI
struct Args {
    /// the directory with all files
    #[argh(positional)]
    path: PathBuf,

    /// the directory of the cache
    #[argh(positional)]
    cache: PathBuf,

    /// the directory for manifests and PKI (default to current dir)
    #[argh(positional)]
    versions: Option<PathBuf>,

    /// path to trunk.txt, relative to `versions`
    #[argh(option, default = "String::from(\"trunk.txt\")")]
    manifest: String,

    /// path to primary.pki, relative to `versions`
    #[argh(option, default = "String::from(\"primary.pki\")")]
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

fn win_join(base: &Path, path: &str) -> PathBuf {
    path.split('\\').fold(base.to_owned(), |mut l, r| {
        l.push(r);
        l
    })
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let base = args.path;

    let versions = args
        .versions
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let manifest_path = versions.join(&args.manifest);
    println!("manifest: {}", manifest_path.display());
    let manifest = Manifest::from_file(&manifest_path)?;

    let pack_index_path = versions.join(&args.pki);
    println!("pack index: {}", pack_index_path.display());
    let pack_index = PackIndexFile::from_file(&pack_index_path)?;

    let cachedir = args.cache;
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

    for (name, (meta, _hash)) in manifest.files {
        let crc = calculate_crc(name.as_bytes());

        if let Some(lookup) = pack_index.files.get(&crc) {
            // File is to be packed
            let pk_id = lookup.pack_file as usize;
            if export.contains(&pk_id) {
                // File is in a pack we want
                let pk = pack_files.entry(pk_id).or_insert_with(|| {
                    let name = &pack_index.archives[pk_id];
                    let path = win_join(&base, &name.path);
                    println!("Opening PK {}", path.display());
                    PKHandle::open(&path).unwrap()
                });

                let is_compressed = lookup.category & 0xFF > 0;

                let path = if is_compressed {
                    patchdir.join(meta.to_path())
                } else {
                    win_join(&base, &name)
                };

                let mut writer = Writer { path: &path };
                pk.put_file(crc, &mut writer, meta, is_compressed)?;
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
