//! # Tools to handle a file system
use std::{
    fs::{DirEntry, Metadata},
    io,
    path::Path,
};

/// Information on a file
pub trait FileInfo {
    /// Return just the filename
    fn name(&self) -> &str;

    /// Return the full "local" path
    fn path(&self) -> String;

    /// Return the "real" path
    fn real(&self) -> &Path;

    /// Return the metadata for this file
    fn metadata(&self) -> io::Result<Metadata>;
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
    fn visit_file<F: FileInfo>(&mut self, info: F);

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

/// Information on a file used by [`scan_dir`]
struct ScanFileInfo<'a> {
    _path: &'a str,
    _name: String,
    _real: &'a Path,
    _entry: DirEntry,
}

impl<'a> FileInfo for ScanFileInfo<'a> {
    fn name(&self) -> &str {
        self._name.as_ref()
    }

    fn path(&self) -> String {
        win_join(self._path, &self._name)
    }

    fn real(&self) -> &Path {
        self._real
    }

    fn metadata(&self) -> io::Result<Metadata> {
        self._entry.metadata()
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
    for e in rd {
        match e {
            Ok(_entry) => {
                let new_real_path = _entry.path();
                let name = new_real_path
                    .file_name()
                    .expect("file_name on dir entry path")
                    .to_string_lossy()
                    .into_owned();
                match _entry.file_type() {
                    Ok(t) => {
                        if t.is_file() {
                            visitor.visit_file(ScanFileInfo {
                                _path: &path,
                                _name: name,
                                _real: &new_real_path,
                                _entry,
                            });
                        } else if t.is_dir() && recurse {
                            let new_path = win_join(&path, &name);
                            scan_dir(visitor, new_path, &new_real_path, recurse);
                        }
                    }
                    Err(e) => visitor.failed_next_dir_entry(&new_real_path, e),
                }
            }
            Err(e) => {
                visitor.failed_next_dir_entry(real, e);
                return;
            }
        };
    }
}
