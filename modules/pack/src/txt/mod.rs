//! # The manifest (`*.txt`) files

pub mod gen;
mod lines;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fmt, io};

use futures_util::{TryStream, TryStreamExt};
use nom_supreme::final_parser::Location;
use thiserror::Error;

use self::lines::{file_line, version_line};
pub use self::lines::{FileLine, FileMeta, VersionLine};

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
            Self::IO(_e) => write!(f, "I/O error"),
            Self::Nom(_e) => write!(f, "Parser error"),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// The parsed version line
    pub version: VersionLine,
    /// The parsed, sorted list of files
    pub files: BTreeMap<String, FileLine>,
}

fn find_version_line<B: BufRead>(buffer: &mut String, reader: &mut B) -> Result<VersionLine> {
    buffer.clear();
    loop {
        let len = reader.read_line(buffer)?;
        assert_ne!(len, 0);

        if buffer.trim() == Section::Version.as_header() {
            buffer.clear();
            reader.read_line(buffer)?;
            //panic!("{:?}", buffer);
            let line = version_line(buffer.trim())?;
            //panic!("{:?}", &line);
            break Ok(line);
        }
        buffer.clear();
    }
}

fn find_files_lines<B: BufRead>(buffer: &mut String, reader: &mut B) -> Result<()> {
    buffer.clear();
    loop {
        let len = reader.read_line(buffer)?;
        assert_ne!(len, 0);

        if buffer.trim() == Section::Files.as_header() {
            buffer.clear();
            break Ok(());
        }
        buffer.clear();
    }
}

impl Manifest {
    /// Read a manifest from a [BufRead] implementation
    ///
    /// ```
    /// use std::{io::Cursor, collections::BTreeMap};
    /// use assembly_pack::{md5::MD5Sum, txt::{Manifest, VersionLine}};
    ///
    /// let hash = MD5Sum::compute("32");
    /// let text = format!("[version]\n32,{},Name\n[files]\n", hash);
    /// let res = Manifest::from_buf_read(&mut Cursor::new(text));
    ///
    /// match res {
    ///     Err(e) => panic!("{}", e),
    ///     Ok(m) => {
    ///         assert_eq!(m, Manifest {
    ///             version: VersionLine::new(32, String::from("Name")),
    ///             files: BTreeMap::new(),
    ///         });
    ///     }
    /// }
    /// ```
    pub fn from_buf_read<B: BufRead>(reader: &mut B) -> Result<Self> {
        let mut buffer = String::new();
        let mut files = BTreeMap::new();
        let version = find_version_line(&mut buffer, reader)?;
        find_files_lines(&mut buffer, reader)?;

        while reader.read_line(&mut buffer)? > 0 {
            let (name, data) = file_line(buffer.trim())?;
            files.insert(name.to_string(), data);
            buffer.clear();
        }

        Ok(Self { version, files })
    }

    /// Read a Manifest from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_buf_read(&mut reader)
    }
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
