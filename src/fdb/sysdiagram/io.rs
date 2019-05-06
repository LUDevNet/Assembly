//! IO functions
use std::convert::TryFrom;
use std::io::{Read, Seek, Cursor};
use std::io::Error as IoError;
use super::core::{SysDiagram};
use super::parser;

use base64::decode as base64_decode;
use base64::DecodeError as Base64DecodeError;
use cfb::CompoundFile;

/// Error wrapper when loading a sysdiagram
#[derive(Debug)]
pub enum LoadError {
    NotImplemented,
    Base64(Base64DecodeError),
    Cfb(IoError),
    StreamTooLong(std::num::TryFromIntError),
    Nom,
}

/// Result when loading a sysdiagram
pub type LoadResult<T> = Result<T, LoadError>;

/// Try to load a sysdiagram from a base64 encoded cfb file
impl TryFrom<&str> for SysDiagram {
    type Error = LoadError;

    fn try_from(string: &str) -> LoadResult<Self> {
        println!("{}", string.len());
        base64_decode(string)
            .map_err(LoadError::Base64)
            .and_then(|vec| {
                println!("{}", vec.len());
                Self::try_from_cfb(Cursor::new(&vec[..]))
            })
    }
}

/// Trait to load something from an OLE file
trait TryFromCfb<T: Read + Seek> where Self: Sized {
    type Error;

    fn try_from_cfb(buf: T) -> Result<Self, Self::Error>;
}

/// Try to load a sysdiagram from an cfb file
impl<T: Read + Seek> TryFromCfb<T> for SysDiagram {
    type Error = LoadError;

    fn try_from_cfb(buf: T) -> LoadResult<Self> {
        let mut reader = CompoundFile::open(buf).map_err(LoadError::Cfb)?;
        let entries = reader.read_storage("/").map_err(LoadError::Cfb)?;
        for entry in entries {
            println!("{}: {}", entry.name(), entry.path().display());
        }
        if reader.is_stream("/f") {
            let mut f_stream = reader.open_stream("/f").map_err(LoadError::Cfb)?;
            let f_stream_len = usize::try_from(f_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
            f_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;

            println!("{}", bytes[256]);

            let mut data = &bytes[256..];
            for _ in 1..10 {
                let (rest, info) = parser::parse_table_info(data).map_err(|_| LoadError::Nom)?;
                println!("{:?}", info);
                data = rest;
            }
        }
        Err(LoadError::NotImplemented)
    }
}
