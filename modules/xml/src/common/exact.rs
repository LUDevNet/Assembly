//! # Functions to expect exactly one event
use {
    quick_xml::{
        events::{BytesEnd as XmlBytesEnd, BytesStart as XmlBytesStart, Event as XmlEvent},
        Error as XmlError, Reader as XmlReader,
    },
    std::{error::Error as StdError, io::BufRead, str::FromStr},
};

use displaydoc::Display;
use quick_xml::events::attributes::AttrError;
use thiserror::Error;

/// The kind of an XML event
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XmlEventKind {
    /// Start Tag `<tag>`
    Start,
    /// End Tag `</tag>`
    End,
    /// Empty tag `<tag/>`
    Empty,
    /// Plain Text
    Text,
    /// Comment `<!-- ... -->`
    Comment,
    /// Literal Character data <![CDATA[ ... ]]>
    CData,
    /// XML Declaration `<?xml ... ?>`
    Decl,
    /// Processing instruction `<?xyz ... ?>`
    PI,
    /// Document type declaration `<!DOCTYPE xyz>`
    DocType,
    /// End of file
    Eof,
}

impl From<&XmlEvent<'_>> for XmlEventKind {
    fn from(e: &XmlEvent) -> Self {
        match e {
            XmlEvent::Start(_) => Self::Start,
            XmlEvent::End(_) => Self::End,
            XmlEvent::Empty(_) => Self::Empty,
            XmlEvent::Text(_) => Self::Text,
            XmlEvent::Comment(_) => Self::Comment,
            XmlEvent::CData(_) => Self::CData,
            XmlEvent::Decl(_) => Self::Decl,
            XmlEvent::PI(_) => Self::PI,
            XmlEvent::DocType(_) => Self::DocType,
            XmlEvent::Eof => Self::Eof,
        }
    }
}

/// The errors for this module
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum Error {
    /// Malformed XML
    Xml(#[from] XmlError),
    /// Malformed XML Attribute
    XmlAttr(#[from] AttrError),
    /// Generic
    Generic(#[from] Box<dyn StdError + Send + Sync>),
    /// Expected tag `{0}`, found `{1:?}`
    ExpectedTag(String, String),
    /// Expected end tag `{0}`, found `{1:?}`
    ExpectedEndTag(String, String),
    /// Expected <{0}> or </{1}>, but found {2:?}
    ExpectedStartEndTag(&'static str, &'static str, XmlEventKind),
    /// Missing tag `{0}`
    MissingTag(String),
    /// Missing end tag `{0}`
    MissingEndTag(String),
    /// Missing text or end tag `{0}`
    MissingTextOrEndTag(String),
    /// Missing text
    MissingText,
    /// Missing Attribute `{0}`
    MissingAttribute(String),
    /// Expected Attribute `{0}`, found `{1:?}`
    ExpectedAttribute(String, String),
}

/// The result type for this module
pub type Result<T> = std::result::Result<T, Error>;

/// Check whether a start tag matches the provided `key`
fn check_start<'a, B: BufRead>(
    e: XmlBytesStart<'a>,
    key: &str,
    reader: &XmlReader<B>,
) -> Result<XmlBytesStart<'a>> {
    if e.name() == key.as_bytes() {
        Ok(e)
    } else {
        Err(Error::ExpectedTag(
            key.to_owned(),
            reader.decode(e.name()).into_owned(),
        ))
    }
}

/// Check whether an end tag matches the provided `key`
fn check_end<'a, B: BufRead>(
    e: XmlBytesEnd<'a>,
    key: &str,
    reader: &XmlReader<B>,
) -> Result<XmlBytesEnd<'a>> {
    if e.name() == key.as_bytes() {
        Ok(e)
    } else {
        Err(Error::ExpectedEndTag(
            key.to_owned(),
            reader.decode(e.name()).into_owned(),
        ))
    }
}

/// Expect an opening tag and return it
pub fn expect_start<'a, 'b, 'c, B: BufRead>(
    key: &'a str,
    reader: &'b mut XmlReader<B>,
    buf: &'c mut Vec<u8>,
) -> Result<XmlBytesStart<'c>> {
    if let Ok(XmlEvent::Start(e)) = reader.read_event(buf) {
        check_start(e, key, reader)
    } else {
        Err(Error::MissingTag(key.to_owned()))
    }
}

/// Expect a closing tag and return it
pub fn expect_end<'a, 'b, 'c, B: BufRead>(
    key: &'a str,
    reader: &'b mut XmlReader<B>,
    buf: &'c mut Vec<u8>,
) -> Result<XmlBytesEnd<'c>> {
    if let Ok(XmlEvent::End(e)) = reader.read_event(buf) {
        check_end(e, key, reader)
    } else {
        Err(Error::MissingEndTag(key.to_owned()))
    }
}

/// Expect an
pub fn expect_child_or_end<'a, B: BufRead>(
    start_key: &'static str,
    end_key: &'static str,
    reader: &mut XmlReader<B>,
    buf: &'a mut Vec<u8>,
) -> Result<Option<XmlBytesStart<'a>>> {
    match reader.read_event(buf)? {
        XmlEvent::Start(s) => check_start(s, start_key, reader).map(Some),
        XmlEvent::End(e) => {
            check_end(e, end_key, reader)?;
            Ok(None)
        }
        e => Err(Error::ExpectedStartEndTag(
            start_key,
            end_key,
            XmlEventKind::from(&e),
        )),
    }
}

/// Expect some text and return it
pub fn expect_text<B: BufRead>(reader: &mut XmlReader<B>, buf: &mut Vec<u8>) -> Result<String> {
    if let Ok(XmlEvent::Text(e)) = reader.read_event(buf) {
        let text = e.unescape_and_decode(reader)?;
        Ok(text)
    } else {
        Err(Error::MissingText)
    }
}

/// Expect either a text node or an end tag
pub fn expect_text_or_end<B: BufRead>(
    key: &str,
    reader: &mut XmlReader<B>,
    buf: &mut Vec<u8>,
) -> Result<String> {
    match reader.read_event(buf)? {
        XmlEvent::Text(t) => {
            let text = t.unescape_and_decode(reader)?;
            buf.clear();
            expect_end(key, reader, buf)?;
            Ok(text)
        }
        XmlEvent::End(e) => {
            check_end(e, key, reader)?;
            Ok(String::new())
        }
        _ => Err(Error::MissingTextOrEndTag(key.to_string())),
    }
}

/// Expect an attribute on an opening tag and return a parsed value
pub fn expect_attribute<T: FromStr, B: BufRead>(
    key: &str,
    reader: &XmlReader<B>,
    event: &XmlBytesStart,
) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + Sized + 'static,
{
    let attr = event
        .attributes()
        .next()
        .ok_or_else(|| Error::MissingAttribute(key.to_owned()))??;

    if attr.key == key.as_bytes() {
        let attr_unesc = attr.unescaped_value()?;
        let attr_str = reader.decode(&attr_unesc);
        let value = attr_str.parse().map_err(|e| {
            let b: Box<dyn StdError + Sync + Send> = Box::new(e);
            b
        })?;
        Ok(value)
    } else {
        Err(Error::ExpectedAttribute(
            key.to_owned(),
            reader.decode(attr.key).into_owned(),
        ))
    }
}
