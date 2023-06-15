//! # Code that's used in all XML modules

use std::{io::BufRead, todo};

use displaydoc::Display;
use quick_xml::{
    events::{attributes::AttrError, Event},
    Reader,
};
use thiserror::Error;

pub mod exact;

/// A general error type
#[derive(Debug, Display, Error)]
pub enum XmlError {
    /// Failed to read the next XML event
    Reader(#[from] quick_xml::Error),
    /// XML Attribute parsing error
    XmlAttr(#[from] AttrError),
    /// Reached EOF while searching for {0}
    EofWhileExpecting(&'static str),
    /// Expected <?xml declaration
    ExpectedDecl,
}

/// The result type for this module
pub type Result<T> = std::result::Result<T, XmlError>;

/// Expect an `<?xml …` declaration
pub fn expect_decl<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<()> {
    if let Event::Decl(_) = xml.read_event(buf)? {
        buf.clear();
        Ok(())
    } else {
        Err(XmlError::ExpectedDecl)
    }
}

/// Expect an opening tag `<{key} name="…">`
pub fn expect_named_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
    parent: Option<&'static str>,
) -> Result<Option<String>> {
    match xml.read_event(buf)? {
        Event::Start(start) => {
            if start.name() == key.as_bytes() {
                let mut name = String::new();
                for attr in start.attributes() {
                    let attr = attr?;
                    if attr.key == b"name" {
                        name = xml.decode(&attr.value).into_owned();
                        break;
                    }
                }
                buf.clear();
                Ok(Some(name))
            } else {
                todo!();
            }
        }
        Event::End(e) => {
            assert_eq!(e.name(), parent.unwrap().as_bytes());
            buf.clear();
            Ok(None)
        }
        Event::Eof => Err(XmlError::EofWhileExpecting(key)),
        _ => panic!(),
    }
}

/// Expect an opening tag `<{key}>`
pub fn expect_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<()> {
    if let Event::Start(start) = xml.read_event(buf)? {
        if start.name() == key.as_bytes() {
            buf.clear();
            Ok(())
        } else {
            todo!();
        }
    } else {
        todo!()
    }
}

/// Expect a closing tag `</{key}>`
pub fn expect_end<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<()> {
    if let Event::End(end) = xml.read_event(buf)? {
        #[allow(clippy::branches_sharing_code)]
        if end.name() == key.as_bytes() {
            buf.clear();
            Ok(())
        } else {
            buf.clear();
            todo!()
        }
    } else {
        todo!()
    }
}
