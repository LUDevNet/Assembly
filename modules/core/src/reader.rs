//! Common error and result handling facilities
use displaydoc::Display;
use nom::{error::ErrorKind, Offset};
use std::{io, num::TryFromIntError};
use thiserror::Error;

/// Error when parsing a file
#[derive(Error, Debug, Display)]
pub enum FileError {
    /// IO Error {0:?}
    IO(#[from] io::Error),
    /// Count Error {0:?}
    Count(TryFromIntError),
    /// Nom Incomplete
    Incomplete,
    /// Nom Error at {addr}+{offset}: {code:?}
    Parse {
        /// Address of the error
        addr: u64,
        /// How far the parser got beyond addr
        offset: usize,
        /// The nom error kind
        code: ErrorKind,
    },
    /// Encoding {0:?}
    StringEncoding(String),

    #[cfg(debug_assertions)]
    /// Not Implemented
    NotImplemented,
    /// {0}
    Custom(&'static str),
}

/// Trait to hand over a parse error past a buffer
pub trait ParseAt<T>: Sized {
    /// Call this after a <IResult as Finish>::finish
    fn at(self, addr: u64, slice: &[u8]) -> Result<T, FileError>;
}

impl<T> ParseAt<T> for Result<T, nom::error::Error<&[u8]>> {
    fn at(self, addr: u64, slice: &[u8]) -> Result<T, FileError> {
        self.map_err(|e| FileError::Parse {
            addr,
            code: e.code,
            offset: slice.offset(e.input),
        })
    }
}

/*/// Nom error
#[derive(Debug, Error)]
pub enum ParseError {
    /// Parsing was not successful
    #[error("Error at -{0}, {1:?}")]
    Error(usize, ErrorKind),
    /// A parse was recognized but invalid
    #[error("Failure at -{0}, {1:?}")]
    Failure(usize, ErrorKind),
    /// Needs more data
    #[error("Incomplete")]
    Incomplete,
}

impl<I: InputLength> From<nom::Err<nom::error::Error<I>>> for ParseError {
    fn from(error: nom::Err<nom::error::Error<I>>) -> Self {
        match error {
            nom::Err::Error(e) => Self::Error(e.input.input_len(), e.code),
            nom::Err::Failure(e) => Self::Failure(e.input.input_len(), e.code),
            nom::Err::Incomplete(_) => Self::Incomplete,
        }
    }
}*/

/// Result when parsing a file
pub type FileResult<T> = Result<T, FileError>;
