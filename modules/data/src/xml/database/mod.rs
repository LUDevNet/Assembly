//! # The XML `<database>` format
//!
//! Before the FDB file format was created, older versions of the client used an XML
//! file to store the client database. This was also used for LUPs to add their data
//! independently of the rest of the game.

use assembly_core::displaydoc::Display;
use std::{error::Error, str::FromStr};

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
