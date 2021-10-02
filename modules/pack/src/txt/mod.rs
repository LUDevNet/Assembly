//! # The manifest (`*.txt`) files

mod lines;

use std::collections::BTreeMap;
use std::{fmt, io};

use futures_util::{TryStream, TryStreamExt};
use nom_supreme::final_parser::Location;
use thiserror::Error;

use self::lines::{file_line, version_line};
pub use self::lines::{FileLine, VersionLine};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// A section of the manifest
pub enum Section {
    /// The `[version]` section
    Version,
    /// The `[files]` section
    Files,
}

impl Section {
    fn as_header(&self) -> &'static str {
        match self {
            Self::Files => "[files]",
            Self::Version => "[version]",
        }
    }
}

#[derive(Debug, Error)]
/// An error from parsing a manifest
pub enum Error {
    /// Unexpected EOF, expected a header
    MissingHeader(&'static str),
    /// Unexpected EOF, expected version line
    MissingVersionLine,
    /// Expected a header but found something else
    ExpectedHeader(&'static str, String),
    /// An IO error
    IO(#[from] std::io::Error),
    /// Failed to parse a line
    Nom(#[from] nom::error::Error<Location>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader(h) => write!(f, "Missing '{}' header", h),
            Self::ExpectedHeader(h, line) => write!(f, "Expected '{}' header, got {:?}", h, line),
            Self::MissingVersionLine => write!(f, "Missing version line"),
            Self::IO(_) => write!(f, "I/O error"),
            Self::Nom(_) => write!(f, "Parser error"),
        }
    }
}

/// The result type for this module
pub type Result<T> = std::result::Result<T, Error>;

async fn expect_header<L>(lines: &mut L, section: Section) -> Result<()>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    let header = section.as_header();
    let res = lines.try_next().await?;
    let line = res.ok_or(Error::MissingHeader(header))?;
    if line != header {
        return Err(Error::ExpectedHeader(header, line));
    }

    Ok(())
}

async fn read_index_version<L>(lines: &mut L) -> Result<VersionLine>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    expect_header(lines, Section::Version).await?;
    let line = lines.try_next().await?.ok_or(Error::MissingVersionLine)?;
    let version = version_line(&line)?;
    Ok(version)
}

/// A manifest file in-memory
pub struct Manifest {
    /// The parsed version line
    pub version: VersionLine,
    /// The parsed, sorted list of files
    pub files: BTreeMap<String, FileLine>,
}

/// Load the manifest from a stream of lines
pub async fn load_manifest<L>(lines: &mut L) -> Result<Manifest>
where
    L: TryStream<Ok = String, Error = io::Error> + Unpin,
{
    let mut files = BTreeMap::new();

    let version = read_index_version(lines).await?;
    expect_header(lines, Section::Files).await?;
    while let Some(item) = lines.try_next().await? {
        let line = item;
        let (filename, data) = file_line(&line)?;
        files.insert(filename.to_owned(), data);
    }

    Ok(Manifest { version, files })
}
