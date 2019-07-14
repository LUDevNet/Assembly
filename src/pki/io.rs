use std::fs::File;
use std::io::{Error as IoError, BufReader, Read};
use std::convert::TryFrom;
use super::core::PackIndexFile;
use super::parser;
use nom::error::ErrorKind;

#[derive(Debug)]
pub enum LoadError {
    FileOpen(IoError),
    Read(IoError),
    Incomplete,
    ParseError(ErrorKind),
    ParseFailure(ErrorKind),
}

type LoadResult<T> = Result<T, LoadError>;

// Generates a LoadError from a nom error
impl From<nom::Err<(&[u8], ErrorKind)>> for LoadError {
    fn from(e: nom::Err<(&[u8], ErrorKind)>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error((_,k)) => LoadError::ParseError(k),
            nom::Err::Failure((_,k)) => LoadError::ParseFailure(k),
        }
    }
}

impl TryFrom<&str> for PackIndexFile {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<PackIndexFile> {
        let file = File::open(filename).map_err(LoadError::FileOpen)?;
        PackIndexFile::try_from(file)
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
