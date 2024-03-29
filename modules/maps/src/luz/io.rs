use super::core::ZoneFile;
use super::parser;
use assembly_core::nom::{error::Error as NomError, error::ErrorKind, Err as NomErr};
use displaydoc::Display;
use std::convert::TryFrom;
use std::io::Read;
use std::{fs, io};
use thiserror::Error;

/// Error when loading a LUZ file
#[derive(Debug, Error, Display)]
pub enum LoadError {
    /// Failed to open the file
    FileOpen(io::Error),
    /// Failed to read from the file
    Read(io::Error),
    /// Missing bytes
    Incomplete,
    /// Failed to parse (recoverable)
    ParseError(ErrorKind),
    /// Failed to parse (fatal)
    ParseFailure(ErrorKind),
}

type LoadResult<T> = Result<T, LoadError>;

// Generates a LoadError from a nom error
impl From<NomErr<NomError<&[u8]>>> for LoadError {
    fn from(e: NomErr<NomError<&[u8]>>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            NomErr::Incomplete(_) => LoadError::Incomplete,
            NomErr::Error(e) => LoadError::ParseError(e.code),
            NomErr::Failure(e) => LoadError::ParseFailure(e.code),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait TryFromLUZ<T>
where
    T: Read,
    Self: Sized,
{
    type Error;

    fn try_from_luz(buf: &mut T) -> Result<Self, Self::Error>;
}

impl TryFrom<&str> for ZoneFile<Vec<u8>> {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<Self> {
        fs::File::open(filename)
            .map_err(LoadError::FileOpen)
            .and_then(ZoneFile::try_from)
    }
}

impl TryFrom<fs::File> for ZoneFile<Vec<u8>> {
    type Error = LoadError;

    fn try_from(file: fs::File) -> LoadResult<Self> {
        ZoneFile::try_from_luz(&mut io::BufReader::new(file))
    }
}

impl<T> TryFromLUZ<T> for ZoneFile<Vec<u8>>
where
    T: Read,
{
    type Error = LoadError;

    fn try_from_luz(buf: &mut T) -> Result<Self, Self::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        buf.read_to_end(&mut bytes)
            .map_err(LoadError::Read)
            .and_then(|_| {
                parser::parse_zone_file(&bytes)
                    .map_err(LoadError::from)
                    .map(|r| r.1)
            })
    }
}
