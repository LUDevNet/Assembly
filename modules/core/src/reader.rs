//! Common error and result handling facilities
use std::io::{Error as IoError};
use std::num::TryFromIntError;
use nom::{error::ErrorKind, Err as NomError};
use std::borrow::Cow;

#[derive(Debug)]
pub enum FileError {
    Read(IoError),
    Seek(IoError),
    Count(TryFromIntError),
    Incomplete,
    ParseError(ErrorKind),
    ParseFailure(ErrorKind),
    StringEncoding(String),

    #[cfg(debug_assertions)]
    NotImplemented,
}

impl From<NomError<(&[u8], ErrorKind)>> for FileError {
    fn from(e: NomError<(&[u8], ErrorKind)>) -> FileError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => FileError::Incomplete,
            nom::Err::Error((_,k)) => FileError::ParseError(k),
            nom::Err::Failure((_,k)) => FileError::ParseFailure(k),
        }
    }
}

impl From<Cow<'_, str>> for FileError {
    fn from(e: Cow<'_, str>) -> Self {
        FileError::StringEncoding(String::from(e))
    }
}

pub type FileResult<T> = Result<T, FileError>;
