//! # Code that's used in all XML modules

use std::io::BufRead;

use assembly_core::displaydoc::Display;
use quick_xml::{events::Event, Reader};
use thiserror::Error;

/// A general error type
#[derive(Debug, Display, Error)]
pub enum XmlError {
    /// Failed to read the next XML event
    Reader(#[from] quick_xml::Error),
    /// Reached EOF while searching for {0}
    EofWhileExpecting(&'static str),
}

/// Expect an opening tag `<{key} name="…">`
pub fn expect_named_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
    parent: Option<&'static str>,
) -> Result<Option<String>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
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
                    break Ok(Some(name));
                } else {
                    todo!();
                }
            }
            Event::End(e) => {
                assert_eq!(e.name(), parent.unwrap().as_bytes());
                buf.clear();
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}

/// Expect an opening tag `<{key}>`
pub fn expect_elem<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<(), XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Start(start) => {
                if start.name() == key.as_bytes() {
                    buf.clear();
                    break Ok(());
                } else {
                    todo!();
                }
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}

/// Expect a closing tag `</{key}>`
pub fn expect_end<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    key: &'static str,
) -> Result<(), XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::End(end) => {
                if end.name() == key.as_bytes() {
                    buf.clear();
                    break Ok(());
                } else {
                    todo!();
                }
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting(key)),
            _ => panic!(),
        }
        buf.clear();
    }
}
