//! IO functions
use std::convert::TryFrom;
use std::io::{Read, Seek, Cursor};
use std::io::Error as IoError;
use super::core::{SysDiagram, Table, Relationship};
use super::parser;

use base64::decode as base64_decode;
use base64::DecodeError as Base64DecodeError;
use cfb::CompoundFile;
use nom::{Context,ErrorKind};
use ms_oforms::controls::form::parser::parse_form_control;
use ms_oforms::controls::form::Site;
use ms_oforms::controls::ole_site_concrete::Clsid;
use std::borrow::Cow;

/// Error wrapper when loading a sysdiagram
#[derive(Debug)]
pub enum LoadError {
    NotImplemented,
    Base64(Base64DecodeError),
    Cfb(IoError),
    StreamTooLong(std::num::TryFromIntError),
    SiteTooLong(std::num::TryFromIntError),
    BufTooLong(std::num::TryFromIntError),
    Incomplete,
    ParseError(ErrorKind),
    ParseFailure,
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
trait TryFromCfb<T: Read + Seek> where Self: Sized {
    type Error;

    fn try_from_cfb(buf: T) -> Result<Self, Self::Error>;
}

impl From<nom::Err<&[u8]>> for LoadError {
    fn from(e: nom::Err<&[u8]>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error(Context::Code(_,k)) => LoadError::ParseError(k),
            nom::Err::Failure(_) => LoadError::ParseFailure,
        }
    }
}

impl From<nom::Err<&str>> for LoadError {
    fn from(e: nom::Err<&str>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error(Context::Code(_,k)) => LoadError::ParseError(k),
            nom::Err::Failure(_) => LoadError::ParseFailure,
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

        let form_control_opt = if reader.is_stream("/f") {
            let mut f_stream = reader.open_stream("/f").map_err(LoadError::Cfb)?;
            let f_stream_len = usize::try_from(f_stream.len()).map_err(LoadError::StreamTooLong)?;
            let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
            f_stream.read_to_end(&mut bytes).map_err(LoadError::Cfb)?;
            let (_rest, form_control) = parse_form_control(&bytes[..]).map_err(LoadError::from)?;
            Some(form_control)
        } else {
            None
        };

        if let Some(form_control) = form_control_opt {
            if reader.is_stream("/o") {
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
                            let site_len = usize::try_from(ole_site.object_stream_size).map_err(LoadError::SiteTooLong)?;
                            match ole_site.clsid_cache_index {
                                Clsid::ClassTable(index) => {
                                    let caption = ole_site.control_tip_text.clone();
                                    /*
                                    println!("===============");
                                    println!("CLASS [{}] {:#?}", index, form_control.site_classes[usize::from(index)].cls_id);
                                    println!("{}", ole_site.name);
                                    println!("{}", ole_site.control_tip_text);
                                    println!("{:#?}", site_len);
                                    println!("---------------");
                                    */
                                    let data = &bytes[offset..];
                                    if index == 0 {
                                        // Table
                                        let (_, sch_grid) = parser::parse_sch_grid(data)?;
                                        tables.push(Table{sch_grid, caption});
                                        /*
                                        println!("{:?}", sch_grid.schema);
                                        println!("{:?}", sch_grid.table);
                                        println!("{:?}", sch_grid.name);
                                        println!("{:?}", sch_grid.size1);
                                        println!("{:?}", sch_grid.size2);
                                        println!(" d1 {:?}", sch_grid.d1);
                                        println!(" d2 {:?}", sch_grid.d2);
                                        println!(" d3 {:?}", sch_grid.d3);
                                        println!(" d4 {:?}", sch_grid.d4);
                                        println!(" d5 {:?}", sch_grid.d5);
                                        println!(" d6 {:?}", sch_grid.d6);
                                        println!(" d7 {:?}", sch_grid.d7);
                                        println!(" d8 {:?}", sch_grid.d8);
                                        println!(" d9 {:?}", sch_grid.d9);
                                        println!("d10 {:?}", sch_grid.d10);
                                        println!("d11 {:?}", sch_grid.d11);
                                        println!("d12 {:?}", sch_grid.d12);
                                        println!("d13 {:?}", sch_grid.d13);
                                        println!("d14 {:?}", sch_grid.d14);
                                        */
                                    } else if index == 1 {
                                        // Foreign Key
                                        let (_, control) = parser::parse_control1(data)?;
                                        let (_, (name, from, to)) = parser::parse_relationship(&caption[..])?;
                                        relationships.push(Relationship{
                                            control, caption, name, from, to,
                                        });
                                        /*
                                        println!("{:#?}", control.positions);
                                        println!("{:?}", control.pos);
                                        println!(" d1 {:?}", control.d1);
                                        println!(" d2 {:?}", control.d2);
                                        println!(" d3 {:?}", control.d3);
                                        println!(" d4 {:?}", control.d4);
                                        println!(" d5 {:?}", control.d5);
                                        println!(" d6 {:?}", control.d6);
                                        println!(" d7 {:?}", control.d7);
                                        println!(" d8 {:?}", control.d8);
                                        println!(" d9 {:?}", control.d9);
                                        */
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
                return Ok(SysDiagram{
                    tables, relationships,
                })
            }
        }
        Err(LoadError::NotImplemented)
    }
}
