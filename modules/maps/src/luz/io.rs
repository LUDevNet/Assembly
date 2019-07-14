use std::{fs, io};
use std::io::{Read};
use std::convert::TryFrom;
use super::core::ZoneFile;
use super::parser;
use assembly_core::nom::{
    error::ErrorKind, Err as NomErr
};

/// Error when loading a LUZ file
#[derive(Debug)]
pub enum LoadError {
    FileOpen(io::Error),
    Read(io::Error),
    Incomplete,
    ParseError(ErrorKind),
    ParseFailure(ErrorKind),
}

type LoadResult<T> = Result<T, LoadError>;

// Generates a LoadError from a nom error
impl From<NomErr<(&[u8], ErrorKind)>> for LoadError {
    fn from(e: NomErr<(&[u8], ErrorKind)>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            NomErr::Incomplete(_) => LoadError::Incomplete,
            NomErr::Error((_,k)) => LoadError::ParseError(k),
            NomErr::Failure((_,k)) => LoadError::ParseFailure(k),
        }
    }
}

pub trait TryFromLUZ<T>
where T: Read, Self: Sized {
    type Error;

    fn try_from_luz(buf: &mut T) -> Result<Self, Self::Error>;
}

impl TryFrom<&str> for ZoneFile {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<ZoneFile> {
        fs::File::open(filename)
            .map_err(LoadError::FileOpen)
            .and_then(ZoneFile::try_from)
    }
}

impl TryFrom<fs::File> for ZoneFile {
    type Error = LoadError;

    fn try_from(file: fs::File) -> LoadResult<ZoneFile> {
        ZoneFile::try_from_luz(&mut io::BufReader::new(file))
    }
}

impl<T> TryFromLUZ<T> for ZoneFile
where T: Read {
    type Error = LoadError;

    fn try_from_luz(buf: &mut T) -> Result<Self, Self::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        buf.read_to_end(&mut bytes)
            .map_err(LoadError::Read)
            .and_then(|_| parser::parse_zone_file(&bytes)
                .map_err(LoadError::from).map(|r| r.1))
    }
}
