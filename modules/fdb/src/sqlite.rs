//! # SQLite conversions and tooling

use std::fmt::Write;

use rusqlite::params_from_iter;
pub use rusqlite::{Connection, Error, Result};

use super::mem::Database;

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
            stmt.execute(params_from_iter(row.field_iter()))?;
        }
    }

    conn.execute("COMMIT", rusqlite::params![])?;
    Ok(())
}
