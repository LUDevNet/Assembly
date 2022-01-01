//! # SQLite conversions and tooling

use std::fmt::Write;

use rusqlite::{types::ToSqlOutput, ToSql};
pub use rusqlite::{Connection, Error, Result};

use super::{
    common::ValueType,
    mem::{Database, Field},
};

impl<'a> ToSql for Field<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        use rusqlite::types::Value;
        let r = match *self {
            Field::Nothing => Value::Null,
            Field::Integer(i) => Value::Integer(i.into()),
            Field::Float(f) => Value::Real(f.into()),
            Field::Text(s) => Value::Text(s.decode().into_owned()),
            Field::Boolean(b) => Value::Integer(if b { 1 } else { 0 }),
            Field::BigInt(i) => Value::Integer(i),
            Field::VarChar(b) => Value::Text(b.decode().into_owned()),
        };
        Ok(ToSqlOutput::Owned(r))
    }
}

impl ValueType {
    /// Get the canonical SQLite name of this data type
    pub fn to_sqlite_type(self) -> &'static str {
        match self {
            ValueType::Nothing => "BLOB_NONE",
            ValueType::Integer => "INT32",
            ValueType::Float => "REAL",
            ValueType::Text => "TEXT4",
            ValueType::Boolean => "INT_BOOL",
            ValueType::BigInt => "INT64",
            ValueType::VarChar => "TEXT_XML",
        }
    }

    /// Take an SQLite column declaration type and guess the ValueType
    ///
    /// This function does a proper round-trip with `to_sqlite_type`
    ///
    /// ```
    /// use assembly_fdb::common::ValueType;
    ///
    /// fn round_trip(v: ValueType) -> Option<ValueType> {
    ///     ValueType::from_sqlite_type(v.to_sqlite_type())
    /// }
    ///
    /// // Check whether the round-trip works
    /// assert_eq!(round_trip(ValueType::Nothing), Some(ValueType::Nothing));
    /// assert_eq!(round_trip(ValueType::Integer), Some(ValueType::Integer));
    /// assert_eq!(round_trip(ValueType::Float), Some(ValueType::Float));
    /// assert_eq!(round_trip(ValueType::Text), Some(ValueType::Text));
    /// assert_eq!(round_trip(ValueType::Boolean), Some(ValueType::Boolean));
    /// assert_eq!(round_trip(ValueType::BigInt), Some(ValueType::BigInt));
    /// assert_eq!(round_trip(ValueType::VarChar), Some(ValueType::VarChar));
    ///
    /// // Check whether lcdr's names work
    /// assert_eq!(ValueType::from_sqlite_type("none"), Some(ValueType::Nothing));
    /// assert_eq!(ValueType::from_sqlite_type("int32"), Some(ValueType::Integer));
    /// assert_eq!(ValueType::from_sqlite_type("real"), Some(ValueType::Float));
    /// assert_eq!(ValueType::from_sqlite_type("text_4"), Some(ValueType::Text));
    /// assert_eq!(ValueType::from_sqlite_type("int_bool"), Some(ValueType::Boolean));
    /// assert_eq!(ValueType::from_sqlite_type("int64"), Some(ValueType::BigInt));
    /// assert_eq!(ValueType::from_sqlite_type("text_8"), Some(ValueType::VarChar));
    /// ```
    pub fn from_sqlite_type(decl_type: &str) -> Option<Self> {
        match decl_type {
            "BLOB_NONE" | "blob_none" | "none" | "NULL" => Some(ValueType::Nothing),
            "INT32" | "int32" | "TINYINT" | "SMALLINT" => Some(ValueType::Integer),
            "real" | "REAL" | "FLOAT" => Some(ValueType::Float),
            "TEXT4" | "text_4" | "TEXT" => Some(ValueType::Text),
            "BIT" | "INT_BOOL" | "int_bool" => Some(ValueType::Boolean),
            "INT64" | "int64" | "BIGINT" | "INTEGER" => Some(ValueType::BigInt),
            "XML" | "TEXT_XML" | "xml" | "text_8" | "text_xml" | "VARCHAR" => {
                Some(ValueType::VarChar)
            }
            _ => None,
        }
    }
}

/// Try to export a database to a SQL connection
///
/// This function does the following:
///
/// 1. `BEGIN`s a transaction
/// 2. For every table:
///   a. Run `CREATE TABLE IF NOT EXISTS`
///   b. Prepares an `INSERT` statement
///   c. Runs the insert with data from every row
/// 3. `COMMIT`s the transaction
pub fn try_export_db(conn: &mut Connection, db: Database) -> rusqlite::Result<()> {
    conn.execute("BEGIN", rusqlite::params![])?;

    let tables = db.tables().unwrap();
    for table in tables.iter() {
        let table = table.unwrap();
        let mut create_query = format!("CREATE TABLE IF NOT EXISTS \"{}\"\n(\n", table.name());
        let mut insert_query = format!("INSERT INTO \"{}\" (", table.name());
        let mut first = true;
        for col in table.column_iter() {
            if first {
                first = false;
            } else {
                writeln!(create_query, ",").unwrap();
                write!(insert_query, ", ").unwrap();
            }
            let typ = col.value_type().to_sqlite_type();
            write!(create_query, "    [{}] {}", col.name(), typ).unwrap();
            write!(insert_query, "[{}]", col.name()).unwrap();
        }
        create_query.push_str(");");
        insert_query.push_str(") VALUES (?1");
        for i in 2..=table.column_count() {
            write!(insert_query, ", ?{}", i).unwrap();
        }
        insert_query.push_str(");");
        conn.execute(&create_query, rusqlite::params![])?;

        let mut stmt = conn.prepare(&insert_query)?;
        for row in table.row_iter() {
            stmt.execute(row)?;
        }
    }

    conn.execute("COMMIT", rusqlite::params![])?;
    Ok(())
}
