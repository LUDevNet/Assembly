//! # Value as stored in owned allocation

use super::{Context, Value};

/// The `Value` context for `core::Field`
#[derive(Debug, PartialEq, Eq)]
pub struct OwnedContext;

impl Context for OwnedContext {
    type String = String;
    type I64 = i64;
    type XML = String;
}

/// An owned field value
pub type Field = Value<OwnedContext>;

#[cfg(feature = "sqlite")]
use rusqlite::types::ToSqlOutput;
#[cfg(feature = "sqlite")]
impl rusqlite::ToSql for Field {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(match self {
            Field::Nothing => ToSqlOutput::Owned(rusqlite::types::Value::Null),
            Field::Integer(i) => (*i).into(),
            Field::Float(f) => (*f).into(),
            Field::Boolean(b) => (*b).into(),
            Field::BigInt(i) => (*i).into(),
            Field::Text(s) => s.as_str().into(),
            Field::VarChar(s) => s.as_str().into(),
        })
    }
}
