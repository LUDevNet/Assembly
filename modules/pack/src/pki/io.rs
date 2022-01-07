//! # Logic to read a PKI file from a byte stream

use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
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

impl PackIndexFile {
    /// Load the PKI from a file
    pub fn from_file<P: AsRef<Path> + ?Sized>(path: &P) -> LoadResult<Self> {
        let file = File::open(path).map_err(LoadError::FileOpen)?;
        let mut reader = BufReader::new(file);
        Self::from_buf_read(&mut reader)
    }

    /// Load the PKI from a [BufRead] implementation
    pub fn from_buf_read<R: BufRead>(reader: &mut R) -> LoadResult<Self> {
        let mut bytes: Vec<u8> = Vec::new();
        reader.read_to_end(&mut bytes).map_err(LoadError::Read)?;
        let (_rest, pki_file) = parser::parse_pki_file(&bytes)?;
        Ok(pki_file)
    }
}

impl TryFrom<&str> for PackIndexFile {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<PackIndexFile> {
        PackIndexFile::from_file(filename)
    }
}

impl TryFrom<File> for PackIndexFile {
    type Error = LoadError;

    fn try_from(file: File) -> LoadResult<PackIndexFile> {
        let mut reader = BufReader::new(file);
        PackIndexFile::from_buf_read(&mut reader)
    }
}
