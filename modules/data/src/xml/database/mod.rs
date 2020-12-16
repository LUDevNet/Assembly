//! # The XML `<database>` format
//!
//! Before the FDB file format was created, older versions of the client used an XML
//! file to store the client database. This was also used for LUPs to add their data
//! independently of the rest of the game.

use assembly_core::displaydoc::Display;
use quick_xml::{events::Event, Reader};
use std::{collections::HashMap, error::Error, io::BufRead, str::FromStr};

use super::common::{expect_elem, expect_named_elem, XmlError};

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
pub struct Column {
    /// The name of the column
    pub name: String,
    /// The data type of the column
    pub data_type: ValueType,
}

/// Expects an empty `<column …/>` tag or a closing `</columns>` tag
pub fn expect_column_or_end_columns<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<Column>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
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
                    break Ok(Some(Column {
                        name: name.unwrap(),
                        data_type: data_type.unwrap(),
                    }));
                } else {
                    todo!();
                }
            }
            Event::End(v) => {
                assert_eq!(v.name(), b"columns");
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting("column")),
            x => panic!("What? {:?}", x),
        }
        buf.clear();
    }
}

/// Expects an empty `<row …/>` tag or a closing `</rows>` tag
pub fn expect_row_or_end_rows<B: BufRead>(
    xml: &mut Reader<B>,
    buf: &mut Vec<u8>,
) -> Result<Option<HashMap<String, String>>, XmlError> {
    loop {
        match xml.read_event(buf)? {
            Event::Text(_) => {}
            Event::Empty(start) => {
                if start.name() == b"row" {
                    let /*mut*/ map = HashMap::new();
                    /*for attr in start.attributes() {
                        let attr = attr?;
                        let key = xml.decode(&attr.key).into_owned();
                        let value = xml.decode(&attr.value).into_owned();
                        map.insert(key, value);
                    }*/
                    buf.clear();
                    break Ok(Some(map));
                } else {
                    todo!();
                }
            }
            Event::End(v) => {
                assert_eq!(v.name(), b"rows");
                return Ok(None);
            }
            Event::Eof => return Err(XmlError::EofWhileExpecting("row")),
            x => panic!("What? {:?}", x),
        }
        buf.clear();
    }
}
