//! IO functions
use super::core::{Relationship, SysDiagram, Table};
use super::parser;
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::{Cursor, Read, Seek};

use base64::decode as base64_decode;
use base64::DecodeError as Base64DecodeError;
use cfb::CompoundFile;
use displaydoc::Display;
use ms_oforms::controls::form::parser::parse_form_control;
use ms_oforms::controls::form::Site;
use ms_oforms::controls::ole_site_concrete::Clsid;
use nom::error::ErrorKind;
use std::borrow::Cow;
use thiserror::Error;

/// Error wrapper when loading a sysdiagram
#[derive(Debug, Error, Display)]
pub enum LoadError {
    /// Not implemented
    NotImplemented,
    /// Could not decode base64 value
    Base64(#[from] Base64DecodeError),
    /// CFB Error
    Cfb(#[from] IoError),
    /// Stream is too long
    StreamTooLong(std::num::TryFromIntError),
    /// Stream is too short
    SiteTooLong(std::num::TryFromIntError),
    /// Buffer is too long
    BufTooLong(std::num::TryFromIntError),
    /// Missing a stream with the filename
    MissingStream(String),
    /// Parsing incomplete
    Incomplete,
    /// Nom parsing error: {0:?}
    ParseError(ErrorKind),
    /// Nom parsing failure: {0:?}
    ParseFailure(ErrorKind),
    /// String encoding error: {0:?}
    StringEncoding(String),
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
trait TryFromCfb<T: Read + Seek>
where
    Self: Sized,
{
    type Error;

    fn try_from_cfb(buf: T) -> Result<Self, Self::Error>;
}

impl From<nom::Err<(&[u8], ErrorKind)>> for LoadError {
    fn from(e: nom::Err<(&[u8], ErrorKind)>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error((_, k)) => LoadError::ParseError(k),
            nom::Err::Failure((_, k)) => LoadError::ParseFailure(k),
        }
    }
}

impl From<nom::Err<(&str, ErrorKind)>> for LoadError {
    fn from(e: nom::Err<(&str, ErrorKind)>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error((_, k)) => LoadError::ParseError(k),
            nom::Err::Failure((_, k)) => LoadError::ParseFailure(k),
        }
    }
}

impl From<Cow<'_, str>> for LoadError {
    fn from(e: Cow<'_, str>) -> Self {
        LoadError::StringEncoding(String::from(e))
    }
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

        let form_control = if reader.is_stream("/f") {
            let mut f_stream = reader.open_stream("/f").map_err(LoadError::Cfb)?;
            let f_stream_len = usize::try_from(f_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
            f_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;
            let (_rest, form_control) = parse_form_control(&bytes[..]).map_err(LoadError::from)?;
            Ok(form_control)
        } else {
            Err(LoadError::MissingStream("f".to_string()))
        }?;

        let dsref_schema_contents = if reader.is_stream("/DSREF-SCHEMA-CONTENTS") {
            let mut r_stream = reader
                .open_stream("/DSREF-SCHEMA-CONTENTS")
                .map_err(LoadError::Cfb)?;
            let r_stream_len = usize::try_from(r_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(r_stream_len);
            r_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;
            let (_, dsref_schema_contents) = parser::parse_dsref_schema_contents(&bytes[..])?;
            Ok(dsref_schema_contents)
        } else {
            Err(LoadError::MissingStream(
                "DSREF-SCHEMA-CONTENTS".to_string(),
            ))
        }?;

        let (tables, relationships) = if reader.is_stream("/o") {
            let mut o_stream = reader.open_stream("/o").map_err(LoadError::Cfb)?;
            let o_stream_len = usize::try_from(o_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(o_stream_len);
            o_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;

            let mut offset = 0;
            let mut tables = Vec::new();
            let mut relationships = Vec::new();
            for site in &form_control.sites[..] {
                match site {
                    Site::Ole(ref ole_site) => {
                        let site_len = usize::try_from(ole_site.object_stream_size)
                            .map_err(LoadError::SiteTooLong)?;
                        match ole_site.clsid_cache_index {
                            Clsid::ClassTable(index) => {
                                let caption = ole_site.control_tip_text.clone();
                                let data = &bytes[offset..];
                                if index == 0 {
                                    // Table
                                    let (_, sch_grid) = parser::parse_sch_grid(data)?;
                                    tables.push(Table { sch_grid, caption });
                                } else if index == 1 {
                                    // Foreign Key
                                    let (_, control) = parser::parse_control1(data)?;
                                    let (_, (name, from, to)) =
                                        parser::parse_relationship(&caption[..])?;
                                    relationships.push(Relationship {
                                        control,
                                        caption,
                                        name,
                                        from,
                                        to,
                                    });
                                } else if index == 2 {
                                    // Control?
                                    // TODO
                                }
                            }
                            Clsid::Invalid => println!("Invalid Class"),
                            Clsid::Global(index) => println!("GLOBAL {}", index),
                        };
                        offset += site_len;
                    }
                }
            }
            Ok((tables, relationships))
        } else {
            Err(LoadError::MissingStream("o".to_string()))
        }?;

        Ok(SysDiagram {
            tables,
            relationships,
            dsref_schema_contents,
        })
    }
}
