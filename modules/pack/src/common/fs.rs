//! # Tools to handle a file system
use std::{collections::BTreeMap, io, path::Path};

/// Information on a file
pub struct FileInfo<'a> {
    _path: &'a str,
    _name: String,
    _real: &'a Path,
}

impl<'a> FileInfo<'a> {
    /// Return just the filename
    pub fn name(&self) -> &str {
        self._name.as_ref()
    }

    /// return the full "local" path
    pub fn path(&self) -> String {
        win_join(self._path, &self._name)
    }

    /// Return the "real" path
    pub fn real(&self) -> &Path {
        self._real
    }
}

// Join a windows path to a prefix
fn win_join(base: &str, name: &str) -> String {
    if base.is_empty() {
        name.to_string()
    } else if base.ends_with('\\') {
        format!("{}{}", base, name)
    } else {
        format!("{}\\{}", base, name)
    }
}

/// A trait to scan a directory of files
pub trait FsVisitor {
    /// Called when a file is visited
    fn visit_file(&mut self, info: FileInfo);

    /// Called when read-dir fails
    #[allow(unused_variables)]
    fn failed_read_dir(&mut self, real: &Path, e: io::Error) {
        #[cfg(feature = "log")]
        log::error!("Failed to read_dir {}: {}", real.display(), e);
    }

    /// Called when read-dir fails
    #[allow(unused_variables)]
    fn failed_next_dir_entry(&mut self, real: &Path, e: io::Error) {
        #[cfg(feature = "log")]
        log::error!("Failed next dir entry {}: {}", real.display(), e);
    }
}

/// Scan a directory and call [FsVisitor::visit_file] for all files
///
/// ## Parameters
///
/// - *path*: a relative path with windows-style separators (i.e `\`)
/// - *read*: the real path of the directory
/// - *recurse*: Whether to recurse into subdirectories
pub fn scan_dir<V: FsVisitor>(visitor: &mut V, path: String, real: &Path, recurse: bool) {
    let rd = match std::fs::read_dir(real) {
        Ok(rd) => rd,
        Err(e) => {
            visitor.failed_read_dir(real, e);
            return;
        }
    };

    // collect all entries
    let mut entries = BTreeMap::new();
    for e in rd {
        match e {
            Ok(e) => {
                let new_real_path = e.path();
                let name = new_real_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                entries.insert(name, (new_real_path, e));
            }
            Err(e) => {
                visitor.failed_next_dir_entry(real, e);
                return;
            }
        };
    }

    // loop over entries
    for (name, (new_real_path, e)) in entries {
        let t = e.file_type().unwrap();

        if t.is_file() {
            visitor.visit_file(FileInfo {
                _path: &path,
                _name: name,
                _real: &new_real_path,
            });
        } else if t.is_dir() && recurse {
            let new_path = win_join(&path, &name);
            scan_dir(visitor, new_path, &new_real_path, recurse);
        }
    }
}
