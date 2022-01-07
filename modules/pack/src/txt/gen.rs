//! # PKI-File Generator

use std::path::PathBuf;

use crate::pki::gen::{ArgEffect, Config, PackFileArg, PackFileConfig};

/// Specification on which directory to include/exclude
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirSpec {
    /// Relative path to the directory
    pub directory: String,
    /// Whether to recurse subdirectories
    pub recurse_subdirectories: bool,
    /// Optional glob / filter
    pub filter_wildcard: String,
}

impl DirSpec {
    fn from_arg(rest: &str) -> Self {
        let (name, rest) = split_first_eq(rest);
        let (recurse_subdirectories, filter_wildcard) = if let Some(rest) = rest {
            let (recurse, rest) = split_first_eq(rest);
            (
                (recurse == "1"),
                rest.map(str::to_owned).unwrap_or_else(String::new),
            )
        } else {
            (true, String::new())
        };
        Self {
            directory: name.to_owned(),
            recurse_subdirectories,
            filter_wildcard,
        }
    }
}

/// Push a command to the config struct
pub fn push_command(config: &mut Config, cmd: Command) {
    match cmd {
        Command::CurrentDirectory(v) => {
            config.directory = PathBuf::from(v);
        }
        Command::PackIndex(v) => {
            config.output = PathBuf::from(v);
        }
        Command::ManifestFile(v) => {
            config.manifest = PathBuf::from(v);
        }
        Command::Pack {
            filename,
            force_compression,
        } => config.pack_files.push(PackFileConfig {
            name: filename,
            compressed: force_compression,
            args: vec![],
        }),
        Command::AddDir(d) => {
            let pack = config.pack_files.iter_mut().next_back().unwrap();
            pack.args.push(PackFileArg {
                effect: ArgEffect::Include,
                name: d.directory,
                kind: crate::pki::gen::ArgKind::Dir {
                    recurse: d.recurse_subdirectories,
                    filter: d.filter_wildcard,
                },
            })
        }
        Command::RemDir(d) => {
            let pack = config.pack_files.iter_mut().next_back().unwrap();
            pack.args.push(PackFileArg {
                effect: ArgEffect::Exclude,
                name: d.directory,
                kind: crate::pki::gen::ArgKind::Dir {
                    recurse: d.recurse_subdirectories,
                    filter: d.filter_wildcard,
                },
            })
        }
        Command::AddFile { filename } => {
            let pack = config.pack_files.iter_mut().next_back().unwrap();
            pack.args.push(PackFileArg {
                effect: ArgEffect::Include,
                name: filename,
                kind: crate::pki::gen::ArgKind::File,
            })
        }
        Command::RemFile { filename } => {
            let pack = config.pack_files.iter_mut().next_back().unwrap();
            pack.args.push(PackFileArg {
                effect: ArgEffect::Include,
                name: filename,
                kind: crate::pki::gen::ArgKind::File,
            })
        }
        Command::EndPack => {
            // NO-OP?
        }
    }
}

/// A single command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Change the current directory
    CurrentDirectory(String),
    /// Set the name of the pack index file
    PackIndex(String),
    /// Set the name of the manifest file
    ManifestFile(String),

    /// Initialize a new pack file
    Pack {
        /// Specify the filename
        filename: String,
        /// Activate compression for this package
        force_compression: bool,
    },
    /// Include a directory
    AddDir(DirSpec),
    /// Exclude a directory
    RemDir(DirSpec),
    /// Add a file
    AddFile {
        /// The name of that file
        filename: String,
    },
    /// Remove a file
    RemFile {
        /// The name of that file
        filename: String,
    },
    /// Complete the pack file
    EndPack,
}

fn split_first_eq(input: &str) -> (&str, Option<&str>) {
    match input.split_once('=') {
        Some((l, r)) => (l, Some(r)),
        None => (input, None),
    }
}

/// Parse a single line of text as a command
pub fn parse_line(line: &str) -> Option<Command> {
    let wo_comment = line.split_once('#').map(|x| x.0).unwrap_or(line);

    if wo_comment.trim().is_empty() {
        return None;
    }

    let (cmd, arg) = match wo_comment.split_once('=') {
        Some((l, r)) => (l.trim(), Some(r.trim())),
        None => (wo_comment.trim(), None),
    };

    match cmd {
        "current_directory" => {
            if let Some(value) = arg {
                return Some(Command::CurrentDirectory(value.to_owned()));
            }
        }
        "pack_index" => {
            if let Some(value) = arg {
                return Some(Command::PackIndex(value.to_owned()));
            }
        }
        "manifest_file" => {
            if let Some(value) = arg {
                return Some(Command::ManifestFile(value.to_owned()));
            }
        }
        "pack" => {
            if let Some(rest) = arg {
                let (name, rest) = split_first_eq(rest);
                let force_compression = rest.map(str::trim_end) == Some("1");
                return Some(Command::Pack {
                    filename: name.trim().to_owned(),
                    force_compression,
                });
            }
        }
        "add_dir" => {
            if let Some(rest) = arg {
                return Some(Command::AddDir(DirSpec::from_arg(rest)));
            }
        }
        "rem_dir" => {
            if let Some(rest) = arg {
                return Some(Command::RemDir(DirSpec::from_arg(rest)));
            }
        }
        "add_file" => {
            if let Some(value) = arg {
                return Some(Command::AddFile {
                    filename: value.to_owned(),
                });
            }
        }
        "rem_file" => {
            if let Some(value) = arg {
                return Some(Command::RemFile {
                    filename: value.to_owned(),
                });
            }
        }
        "end_pack" => return Some(Command::EndPack),
        _ => {
            log::error!("Invalid command {} w/ arg {:?}", cmd, arg)
        }
    }

    None
}
