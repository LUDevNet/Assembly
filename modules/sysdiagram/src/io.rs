//! IO functions
use crate::parse_dsref_schema_contents;

use super::core::{Relationship, SysDiagram, Table};
use super::parser;
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::{Cursor, Read, Seek};

use base64::decode as base64_decode;
use base64::DecodeError as Base64DecodeError;
use cfb::CompoundFile;
use displaydoc::Display;
use ms_oforms::controls::form::parse_form_control;
use ms_oforms::controls::form::Site;
use ms_oforms::controls::ole_site_concrete::Clsid;
use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};
use nom::InputLength;
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
    /// Nom parsing error: {0:?} at -{1}
    ParseError(ErrorKind, usize),
    /// Nom parsing failure: {0:?} at -{1}
    ParseFailure(ErrorKind, usize),
    /// Nom parsing error: {0:#?}
    ParseErrorVerbose(Vec<(VerboseErrorKind, usize)>),
    /// Nom parsing failure: {0:#?}
    ParseFailureVerbose(Vec<(VerboseErrorKind, usize)>),
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

impl<I: InputLength> From<nom::Err<nom::error::Error<I>>> for LoadError {
    fn from(e: nom::Err<nom::error::Error<I>>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error(e) => LoadError::ParseError(e.code, e.input.input_len()),
            nom::Err::Failure(e) => LoadError::ParseFailure(e.code, e.input.input_len()),
        }
    }
}

impl<I: InputLength> From<nom::Err<VerboseError<I>>> for LoadError {
    fn from(e: nom::Err<VerboseError<I>>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error(e) => LoadError::ParseErrorVerbose(
                e.errors
                    .into_iter()
                    .map(|e| (e.1, e.0.input_len()))
                    .collect(),
            ),
            nom::Err::Failure(e) => LoadError::ParseFailureVerbose(
                e.errors
                    .into_iter()
                    .map(|e| (e.1, e.0.input_len()))
                    .collect(),
            ),
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
        eprintln!("CFB Streams:");
        for entry in entries {
            println!("- {:?}: {}", entry.name(), entry.path().display());
        }

        let form_control = if reader.is_stream("/f") {
            let mut f_stream = reader.open_stream("/f").map_err(LoadError::Cfb)?;
            let f_stream_len = usize::try_from(f_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
            f_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;
            let (_rest, form_control) =
                parse_form_control::<VerboseError<&[u8]>>(&bytes[..]).map_err(LoadError::from)?;
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
            let (_, dsref_schema_contents) = parse_dsref_schema_contents(&bytes[..])?;
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
