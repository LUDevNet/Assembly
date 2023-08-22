//! # Generating PKI files

use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
};

use crate::{
    common::fs::{scan_dir, FileInfo, FsVisitor},
    crc::CRC,
};

use super::core::PackIndexFile;

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

fn join_with_str(base: &Path, e: &str) -> PathBuf {
    e.split('\\').fold(base.to_owned(), extend_path_buf)
}

fn path_locale(path: &str) -> Option<&str> {
    path.split('\\').skip_while(|x| *x != "_loc").nth(1)
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

struct Visitor<'c, 'f> {
    filter: Filter<'f>,
    effect: ArgEffect,
    state: &'c mut RunState,
}

impl<'c, 'f> FsVisitor for Visitor<'c, 'f> {
    fn visit_file<F: FileInfo>(&mut self, info: F) {
        if self.filter.matches(info.name()) {
            self.state.on_file(&info.path(), self.effect)
        }
    }
}

#[derive(Default)]
struct RunState {
    crc_set: HashSet<CRC>,
    _loc_map: BTreeMap<String, HashSet<CRC>>,
}

impl RunState {
    fn on_file(&mut self, path: &str, effect: ArgEffect) {
        let crc = CRC::from_path(path);
        let _loc = path_locale(path);
        #[cfg(feature = "log")]
        log::debug!("file {}", path);
        let crc_set = match _loc {
            Some(_loc) => self._loc_map.entry(_loc.to_ascii_lowercase()).or_default(),
            None => &mut self.crc_set,
        };
        match effect {
            ArgEffect::Include => {
                crc_set.insert(crc);
            }
            ArgEffect::Exclude => {
                crc_set.remove(&crc);
            }
        }
    }
}

impl Config {
    /// Run the given config
    pub fn run(&self) -> PackIndexFile {
        let root: &Path = self.directory.as_ref();
        let mut pki = PackIndexFile {
            archives: Vec::with_capacity(self.pack_files.len()),
            files: BTreeMap::new(),
        };

        for pack_file in self.pack_files.iter() {
            let mut state = RunState::default();

            for arg in &pack_file.args {
                let path = {
                    let mut p = self.prefix.clone();
                    p.push_str(&arg.name);
                    p
                };
                match &arg.kind {
                    ArgKind::File => state.on_file(&path, arg.effect),
                    ArgKind::Dir { recurse, filter } => {
                        let real_path = join_with_str(root, &arg.name);

                        let filter = Filter::from(filter.as_ref());
                        let mut visitor = Visitor {
                            filter,
                            effect: arg.effect,
                            state: &mut state,
                        };
                        scan_dir(&mut visitor, path, &real_path, *recurse);
                    }
                }
            }

            let path = format!("{}{}", self.prefix, &pack_file.name);
            let components = path.rsplit_once('\\');
            for (_loc, crc_set) in &state._loc_map {
                // insert _loc\{_loc}\ before the filename, after any prefix
                let path = match components {
                    Some((base, filename)) => format!("{base}\\_loc\\{_loc}\\{filename}"),
                    None => format!("_loc\\{_loc}\\{path}"),
                };
                pki.add_pack(path, pack_file.compressed)
                    .add_files(crc_set.iter().copied());
            }
            if !state.crc_set.is_empty() {
                pki.add_pack(path, pack_file.compressed)
                    .add_files(state.crc_set.iter().copied());
            }
        }

        pki
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_locale() {
        assert_eq!(
            super::path_locale("texture\\_loc\\de_DE\\test.txt"),
            Some("de_DE")
        )
    }
}
