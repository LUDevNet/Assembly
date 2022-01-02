//! # Generating PKI files

use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
};

use crate::crc::calculate_crc;

use super::core::{FileRef, PackFileRef, PackIndexFile};

/// Config for creating a pack file
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The directory to pull from
    pub directory: PathBuf,
    /// The file to output the PKI to
    pub output: PathBuf,
    /// The manifest file (e.g. trunk.txt)
    pub manifest: PathBuf,
    /// Prefix
    pub prefix: String,
    /// The list of pack files
    pub pack_files: Vec<PackFileConfig>,
}

fn extend_path_buf(mut p: PathBuf, e: &str) -> PathBuf {
    p.push(e);
    p
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Filter<'a> {
    None,
    Exact(&'a str),
    StartsWith(&'a str),
    EndsWith(&'a str),
    Contains(&'a str),
}

impl<'a> Filter<'a> {
    fn matches(self, other: &'a str) -> bool {
        match self {
            Self::None => true,
            Self::Exact(v) => other == v,
            Self::StartsWith(v) => other.starts_with(v),
            Self::EndsWith(v) => other.ends_with(v),
            Self::Contains(v) => other.contains(v),
        }
    }
}

impl<'a> From<&'a str> for Filter<'a> {
    fn from(text: &'a str) -> Self {
        let mut chr = text.chars();
        let start = chr.next();
        let end = chr.next_back();
        let len = text.len();
        match (start, end) {
            (Some('*'), Some('*')) => Filter::Contains(&text[1..(len - 1)]),
            (Some('*'), _) => Filter::EndsWith(&text[1..]),
            (_, Some('*')) => Filter::StartsWith(&text[..len - 1]),
            (None, None) => Filter::None,
            _ => Filter::Exact(text),
        }
    }
}

impl Config {
    /// Run the given config
    pub fn run(&self) -> PackIndexFile {
        let root: &Path = self.directory.as_ref();
        let mut archives = Vec::with_capacity(self.pack_files.len());
        let mut files = BTreeMap::new();

        let mut index = 0u32;
        for pack_file in self.pack_files.iter() {
            let mut crc_set = HashSet::<u32>::new();

            for arg in &pack_file.args {
                let path = {
                    let mut p = self.prefix.clone();
                    p.push_str(&arg.name);
                    p
                };
                match &arg.kind {
                    ArgKind::File => {
                        let crc = calculate_crc(path.as_bytes());
                        println!("file {}", path);
                        match arg.effect {
                            ArgEffect::Include => {
                                crc_set.insert(crc);
                            }
                            ArgEffect::Exclude => {
                                crc_set.remove(&crc);
                            }
                        }
                    }
                    ArgKind::Dir { recurse, filter } => {
                        let real_path = arg.name.split('\\').fold(root.to_owned(), extend_path_buf);

                        let filter = Filter::from(filter.as_ref());

                        let mut todo = vec![];
                        todo.push((path, real_path));

                        while let Some((path, real_path)) = todo.pop() {
                            match std::fs::read_dir(&real_path) {
                                Ok(rd) => {
                                    for e in rd {
                                        match e {
                                            Ok(e) => {
                                                let new_real_path = e.path();
                                                let t = e.file_type().unwrap();
                                                let name = new_real_path
                                                    .file_name()
                                                    .unwrap()
                                                    .to_string_lossy();

                                                if t.is_file() && filter.matches(name.as_ref()) {
                                                    let new_path = format!("{}\\{}", path, name);
                                                    let crc = calculate_crc(new_path.as_bytes());
                                                    println!("dir-file {}", new_path);
                                                    match arg.effect {
                                                        ArgEffect::Include => {
                                                            crc_set.insert(crc);
                                                        }
                                                        ArgEffect::Exclude => {
                                                            crc_set.remove(&crc);
                                                        }
                                                    }
                                                } else if t.is_dir() && *recurse {
                                                    let new_path = format!("{}\\{}", path, name);
                                                    todo.push((new_path, new_real_path));
                                                }
                                            }
                                            Err(e) => {
                                                log::error!(
                                                    "Failed to read_dir entry {}: {}",
                                                    real_path.display(),
                                                    e
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to read_dir {}: {}",
                                        real_path.display(),
                                        e
                                    );
                                    // FIXME: Handle Error
                                }
                            }
                        }
                    }
                }
            }

            let has_entries = !crc_set.is_empty();
            for crc in crc_set {
                files.insert(
                    crc,
                    FileRef {
                        category: if pack_file.compressed { 1 } else { 0 },
                        pack_file: index,
                    },
                );
            }

            if has_entries {
                archives.push(PackFileRef {
                    path: pack_file.name.clone(),
                });
                index += 1;
            }
        }

        PackIndexFile { archives, files }
    }
}

/// Config for a single pack file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackFileConfig {
    /// name of the file
    pub name: String,
    /// Whether to compress files
    pub compressed: bool,
    /// what files to include
    pub args: Vec<PackFileArg>,
}

/// Whether to include or exclude the specific files
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgEffect {
    /// Include matching files
    Include,
    /// Exclude matching files
    Exclude,
}

/// Kind of an argument
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgKind {
    /// The name represents a file
    File,
    /// The name represents a directory
    Dir {
        /// Whether to recurse into subdirectories
        recurse: bool,
        /// A filter for the specific file name
        filter: String,
    },
}

/// Argument to a pack file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackFileArg {
    /// Include or Exclude
    pub effect: ArgEffect,
    /// The name
    pub name: String,
    /// The kind of the argument
    pub kind: ArgKind,
}
