//! # Logic to read a PKI file from a byte stream

use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Error as IoError, Read};
use std::path::Path;
use thiserror::Error;

use super::core::PackIndexFile;
use super::parser;

use nom::{self, error::ErrorKind, Err as NomErr};

#[derive(Debug, Error)]
/// Failed to load a PKI file
pub enum LoadError {
    /// Failed to open the file
    FileOpen(#[source] IoError),
    /// Failed to read from the file
    Read(#[source] IoError),
    /// EOF while parsing
    Incomplete,
    /// File did not match parser
    ParseError(ErrorKind),
    /// Valid file but invalid data
    ParseFailure(ErrorKind),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::FileOpen(_) => write!(f, "Failed to open file"),
            LoadError::Read(_) => write!(f, "Failed to read file"),
            LoadError::Incomplete => write!(f, "Unexpected EOF"),
            LoadError::ParseError(e) => write!(f, "File not recognized: {:?}", e),
            LoadError::ParseFailure(e) => write!(f, "File corrupt: {:?}", e),
        }
    }
}

type LoadResult<T> = Result<T, LoadError>;

// Generates a LoadError from a nom error
impl From<NomErr<nom::error::Error<&[u8]>>> for LoadError {
    fn from(e: NomErr<nom::error::Error<&[u8]>>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            NomErr::Incomplete(_) => LoadError::Incomplete,
            NomErr::Error(e) => LoadError::ParseError(e.code),
            NomErr::Failure(e) => LoadError::ParseFailure(e.code),
        }
    }
}

impl TryFrom<&Path> for PackIndexFile {
    type Error = LoadError;

    fn try_from(filename: &Path) -> LoadResult<PackIndexFile> {
        let file = File::open(filename).map_err(LoadError::FileOpen)?;
        PackIndexFile::try_from(file)
    }
}

impl TryFrom<&str> for PackIndexFile {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<PackIndexFile> {
        PackIndexFile::try_from(Path::new(filename))
    }
}

impl TryFrom<File> for PackIndexFile {
    type Error = LoadError;

    fn try_from(file: File) -> LoadResult<PackIndexFile> {
        let mut buf = BufReader::new(file);
        let mut bytes: Vec<u8> = Vec::new();
        buf.read_to_end(&mut bytes).map_err(LoadError::Read)?;
        let (_rest, pki_file) = parser::parse_pki_file(&bytes)?;
        Ok(pki_file)
    }
}
