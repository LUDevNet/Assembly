//! Values as stored in byte slices

use latin1str::Latin1Str;

use super::{Context, Value};

#[derive(Debug, PartialEq)]
/// The context for `mem::Field`
pub struct MemContext<'a> {
    _m: std::marker::PhantomData<fn() -> &'a ()>,
}

impl<'a> Context for MemContext<'a> {
    type String = &'a Latin1Str;
    type I64 = i64;
    type XML = &'a Latin1Str;
}

/// Value of or reference to a field value
pub type Field<'a> = Value<MemContext<'a>>;

#[cfg(feature = "sqlite")]
impl<'a> rusqlite::ToSql for Field<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::ToSqlOutput;
        use std::borrow::Cow;
        Ok(match *self {
            Field::Nothing => ToSqlOutput::Owned(rusqlite::types::Value::Null),
            Field::Integer(i) => i.into(),
            Field::Float(f) => f.into(),
            Field::Boolean(b) => b.into(),
            Field::BigInt(i) => i.into(),
            Field::Text(s) | Field::VarChar(s) => match s.decode() {
                Cow::Owned(s) => ToSqlOutput::from(s),
                Cow::Borrowed(s) => ToSqlOutput::from(s),
            },
        })
    }
}
