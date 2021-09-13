//! # The XML `<database>` format
//!
//! Before the FDB file format was created, older versions of the client used an XML
//! file to store the client database. This was also used for LUPs to add their data
//! independently of the rest of the game.

use assembly_core::displaydoc::Display;
use quick_xml::{events::Event, Reader};
use std::{collections::HashMap, error::Error, fmt, io::BufRead, str::FromStr};

use super::common::{expect_elem, expect_named_elem, XmlError};

#[cfg(feature = "serde-derives")]
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer,
};

/// The value types for the database
///
/// This is a rustic representation of the data types in Transact-SQL that
/// were/are used in the database.
///
/// See: <https://docs.microsoft.com/en-us/sql/t-sql/data-types>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    /// `bit`
    Bit,

    /// `float`
    Float,
    /// `real`
    Real,

    /// `int`
    Int,
    /// `bigint`
    BigInt,
    /// `smallint`
    SmallInt,
    /// `tinyint`
    TinyInt,

    /// `binary`
    Binary,
    /// `varbinary`
    VarBinary,

    /// `char`
    Char,
    /// `varchar`
    VarChar,

    /// `nchar`
    NChar,
    /// `nvarchar`
    NVarChar,

    /// `ntext`
    NText,
    /// `text`
    Text,
    /// `image`
    Image,

    /// `datetime`
    DateTime,
}

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ValueTypeVisitor)
    }
}

#[derive(Debug, Display)]
/// Unknown value type '{0}'
pub struct UnknownValueType(String);

impl Error for UnknownValueType {}

impl FromStr for ValueType {
    type Err = UnknownValueType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bit" => Ok(Self::Bit),

            "float" => Ok(Self::Float),
            "real" => Ok(Self::Real),

            "int" => Ok(Self::Int),
            "bigint" => Ok(Self::BigInt),
            "smallint" => Ok(Self::SmallInt),
            "tinyint" => Ok(Self::TinyInt),

            "binary" => Ok(Self::Binary),
            "varbinary" => Ok(Self::VarBinary),

            "char" => Ok(Self::Char),
            "varchar" => Ok(Self::VarChar),

            "nchar" => Ok(Self::NChar),
            "nvarchar" => Ok(Self::NVarChar),

            "text" => Ok(Self::Text),
            "ntext" => Ok(Self::NText),
            "image" => Ok(Self::Image),

            "datetime" => Ok(Self::DateTime),

            _ => Err(UnknownValueType(s.to_owned())),
        }
    }
}

#[cfg(feature = "serde-derives")]
struct ValueTypeVisitor;

#[cfg(feature = "serde-derives")]
impl<'de> Visitor<'de> for ValueTypeVisitor {
    type Value = ValueType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("T-SQL value type")
    }

    fn visit_str<E>(self, value: &str) -> Result<ValueType, E>
    where
        E: de::Error,
    {
        FromStr::from_str(value)
            .map_err(|_| E::invalid_value(Unexpected::Other(value), &"T-SQL value type"))
    }
}

/// A row of the database
#[cfg_attr(feature = "serde-derives", derive(Deserialize))]
#[derive(Debug, Eq, PartialEq)]
pub struct Row(HashMap<String, String>);

/// Expects an opening `<database>`
pub fn expect_database<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, XmlError> {
    expect_named_elem(xml, buf, "database", None)
}

/// Expects an opening `<table>` tag or a closing `</database>` tag
pub fn expect_table<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<String>, XmlError> {
    expect_named_elem(xml, buf, "table", Some("database"))
}

/// Expects an opening `<columns>` tag
pub fn expect_columns<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<(), XmlError> {
    expect_elem(xml, buf, "columns")
}

/// Expects an opening `<rows>` tag
pub fn expect_rows<B: BufRead>(xml: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<(), XmlError> {
    expect_elem(xml, buf, "rows")
}

/// The information on a column
#[cfg_attr(feature = "serde-derives", derive(Deserialize))]
pub struct Column {
    /// The name of the column
    pub name: String,
    /// The data type of the column
    pub r#type: ValueType,
}

/*#[derive(Deserialize)]
/// The Columns struct
pub struct Columns {
    /// The columns
    columns: Vec<Column>
}*/

/// Expects an empty `<column …/>` tag or a closing `</columns>` tag
pub fn expect_column_or_end_columns<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<Column>, XmlError> {
    match xml.read_event(buf)? {
        Event::Empty(start) => {
            if start.name() == b"column" {
                let mut name = None;
                let mut data_type = None;
                for attr in start.attributes() {
                    let attr = attr?;
                    if attr.key == b"name" {
                        name = Some(xml.decode(&attr.value).into_owned());
                    }

                    if attr.key == b"type" {
                        data_type = Some(
                            xml.decode(&attr.value)
                                .parse()
                                .expect("Expected well-known value type"),
                        );
                    }
                }
                buf.clear();
                Ok(Some(Column {
                    name: name.unwrap(),
                    r#type: data_type.unwrap(),
                }))
            } else {
                todo!();
            }
        }
        Event::End(v) => {
            assert_eq!(v.name(), b"columns");
            Ok(None)
        }
        Event::Eof => Err(XmlError::EofWhileExpecting("column")),
        x => panic!("What? {:?}", x),
    }
}

/// Expects an empty `<row …/>` tag or a closing `</rows>` tag
pub fn expect_row_or_end_rows<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
    load_attrs: bool,
) -> Result<Option<HashMap<String, String>>, XmlError> {
    match xml.read_event(buf)? {
        Event::Empty(start) => {
            if start.name() == b"row" {
                let map = if load_attrs {
                    let mut m = HashMap::new();
                    for attr in start.attributes() {
                        let attr = attr?;
                        let key = xml.decode(attr.key).into_owned();
                        let value = attr.unescape_and_decode_value(xml)?;
                        m.insert(key, value);
                    }
                    m
                } else {
                    HashMap::new()
                };
                buf.clear();
                Ok(Some(map))
            } else {
                todo!();
            }
        }
        Event::End(v) => {
            assert_eq!(v.name(), b"rows");
            Ok(None)
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::{de::Deserializer, DeError};

    use super::*;
    use quick_xml::Error as XmlError;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn test_simple_deserialize() {
        let st = br#"<row foo="bar"/></rows>"#;
        let c = BufReader::new(Cursor::new(st));
        let mut d = Deserializer::from_reader(c);
        let row = Row::deserialize(&mut d).unwrap();
        let mut cmp = HashMap::new();
        let key = String::from("foo");
        let val = String::from("bar");
        cmp.insert(key, val);
        assert_eq!(row.0, cmp);

        if let Err(DeError::Xml(XmlError::EndEventMismatch { expected, found })) =
            Row::deserialize(&mut d)
        {
            assert_eq!(&expected, "");
            assert_eq!(&found, "rows");
        }
    }
}
