//! # Functions to expect exactly one event
use {
    quick_xml::{
        events::{BytesEnd as XmlBytesEnd, BytesStart as XmlBytesStart, Event as XmlEvent},
        Error as XmlError, Reader as XmlReader,
    },
    std::{error::Error as StdError, io::BufRead, str::FromStr},
};

use displaydoc::Display;
use thiserror::Error;

/// The errors for this module
#[derive(Debug, Display, Error)]
pub enum Error {
    /// Malformed XML
    Xml(#[from] XmlError),
    /// Generic
    Generic(#[from] Box<dyn StdError + Send + Sync>),
    /// Expected tag `{0}`, found `{1:?}`
    ExpectedTag(String, String),
    /// Expected tag `{0}`, found `{1:?}`
    ExpectedEndTag(String, String),
    /// Missing tag `{0}`
    MissingTag(String),
    /// Missing end tag `{0}`
    MissingEndTag(String),
    /// Missing text
    MissingText,
    /// Missing Attribute `{0}`
    MissingAttribute(String),
    /// Expected Attribute `{0}`, found `{1:?}`
    ExpectedAttribute(String, String),
}

/// The result type for this module
pub type Result<T> = std::result::Result<T, Error>;

/// Expect an opening tag and return it
pub fn expect_start<'a, 'b, 'c, B: BufRead>(
    key: &'a str,
    reader: &'b mut XmlReader<B>,
    buf: &'c mut Vec<u8>,
) -> Result<XmlBytesStart<'c>> {
    if let Ok(XmlEvent::Start(e)) = reader.read_event(buf) {
        if e.name() == key.as_bytes() {
            Ok(e)
        } else {
            Err(Error::ExpectedTag(
                key.to_owned(),
                reader.decode(e.name()).into_owned(),
            ))
        }
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
        if e.name() == key.as_bytes() {
            Ok(e)
        } else {
            Err(Error::ExpectedEndTag(
                key.to_owned(),
                reader.decode(e.name()).into_owned(),
            ))
        }
    } else {
        Err(Error::MissingEndTag(key.to_owned()))
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
