//! Common error and result handling facilities
use displaydoc::Display;
use nom::{error::ErrorKind, Err as NomError};
use std::{error::Error, io::Error as IoError, num::TryFromIntError};
use thiserror::Error;

#[derive(Debug, Display)]
pub enum FileError {
    /// Read Error {0:?}
    Read(IoError),
    /// Seek Error {0:?}
    Seek(IoError),
    /// Count Error {0:?}
    Count(TryFromIntError),
    /// Nom Incomplete
    Incomplete,
    /// Nom Error {0:?}
    ParseError(ErrorKind),
    /// Nom Failure {0:?}
    ParseFailure(ErrorKind),
    /// Encoding {0:?}
    StringEncoding(String),

    #[cfg(debug_assertions)]
    /// Not Implemented
    NotImplemented,
}

impl Error for FileError {}

impl From<NomError<(&[u8], ErrorKind)>> for FileError {
    fn from(e: NomError<(&[u8], ErrorKind)>) -> FileError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => FileError::Incomplete,
            nom::Err::Error((_, k)) => FileError::ParseError(k),
            nom::Err::Failure((_, k)) => FileError::ParseFailure(k),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Error at -{0}, {1:?}")]
    Error(usize, ErrorKind),
    #[error("Failure at -{0}, {1:?}")]
    Failure(usize, ErrorKind),
    #[error("Incomplete")]
    Incomplete,
}

impl From<NomError<(&[u8], ErrorKind)>> for ParseError {
    fn from(e: NomError<(&[u8], ErrorKind)>) -> ParseError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => ParseError::Incomplete,
            nom::Err::Error((r, k)) => ParseError::Error(r.len(), k),
            nom::Err::Failure((r, k)) => ParseError::Failure(r.len(), k),
        }
    }
}

/*impl From<Cow<'_, str>> for FileError {
    fn from(e: Cow<'_, str>) -> Self {
        FileError::StringEncoding(String::from(e))
    }
}*/

pub type FileResult<T> = Result<T, anyhow::Error>;
