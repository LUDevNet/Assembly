//! # Virtual Table implementation

use rusqlite::vtab::{read_only_module, CreateVTab, VTab, VTabCursor};

use crate::mem;

#[repr(C)]
struct FdbTab<'vtab> {
    /// Base class. Must be first
    base: rusqlite::vtab::sqlite3_vtab,
    /* Virtual table implementations will typically add additional fields */
    table: mem::Table<'vtab>,
}

/// Register the module
pub fn load_module(
    conn: &rusqlite::Connection,
    db: mem::Database<'static>,
) -> rusqlite::Result<()> {
    conn.create_module("fdb", read_only_module::<FdbTab>(), Some(db))
}

struct BufferedIter<'vtab> {
    /// The backing iterator
    iter: mem::iter::TableRowIter<'vtab>,
    /// The item at the front
    next: Option<mem::Row<'vtab>>,
    /// The rowid
    rowid: i64,
}

impl<'vtab> BufferedIter<'vtab> {
    pub fn new(table: mem::Table<'vtab>) -> Self {
        let mut iter = table.row_iter();
        let next = iter.next();
        Self {
            iter,
            next,
            rowid: 0,
        }
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.next.is_none()
    }

    pub fn next(&mut self) {
        self.next = self.iter.next();
        self.rowid += 1;
    }
}

#[repr(C)]
struct FdbTabCursor<'vtab> {
    /// Base class. Must be first
    base: rusqlite::vtab::sqlite3_vtab_cursor,
    /* Virtual table implementations will typically add additional fields */
    iter: BufferedIter<'vtab>,
}

unsafe impl<'vtab> VTabCursor for FdbTabCursor<'vtab> {
    fn filter(
        &mut self,
        _idx_num: std::os::raw::c_int,
        _idx_str: Option<&str>,
        _args: &rusqlite::vtab::Values<'_>,
    ) -> rusqlite::Result<()> {
        Ok(())
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.iter.next(); // Just drop the next one
        Ok(())
    }

    fn eof(&self) -> bool {
        self.iter.eof()
    }

    fn column(
        &self,
        ctx: &mut rusqlite::vtab::Context,
        i: std::os::raw::c_int,
    ) -> rusqlite::Result<()> {
        // Get the row
        let row = self.iter.next.ok_or_else(|| {
            rusqlite::Error::ModuleError(format!(
                "FDB: no data for col {} past the end of the table",
                i
            ))
        })?;

        // Get the field
        let field = row.field_at(i as usize).ok_or_else(|| {
            rusqlite::Error::ModuleError(format!("FDB: column {} out of bounds", i))
        })?;

        // Return the result
        ctx.set_result(&field)?;

        Ok(())
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.iter.rowid)
    }
}

unsafe impl<'vtab> VTab<'vtab> for FdbTab<'vtab> {
    type Aux = mem::Database<'vtab>;

    type Cursor = FdbTabCursor<'vtab>;

    fn connect(
        _db: &mut rusqlite::vtab::VTabConnection,
        aux: Option<&Self::Aux>,
        args: &[&[u8]],
    ) -> rusqlite::Result<(String, Self)> {
        match *args {
            [] => Err(rusqlite::Error::InvalidParameterCount(0, 1)),
            [mod_name, db_name, table_name] => {
                let name = std::str::from_utf8(table_name).map_err(|e| {
                    rusqlite::Error::ModuleError(format!("FDB: Table name is invalid UTF-8: {}", e))
                })?;

                let table = aux
                    .ok_or_else(|| {
                        rusqlite::Error::ModuleError(format!(
                            "Missing 'aux' for FDB vtab ({:?},{:?},{:?})",
                            mod_name, db_name, table_name
                        ))
                    })?
                    .tables()
                    .map_err(|e| {
                        rusqlite::Error::ModuleError(format!("FDB: Failed to get [Tables]: {}", e))
                    })?
                    .by_name(name)
                    .ok_or_else(|| {
                        rusqlite::Error::ModuleError(format!("FDB: Unknown table {:?}", name))
                    })?
                    .map_err(|e| {
                        rusqlite::Error::ModuleError(format!("FDB: Failed to get [Tables]: {}", e))
                    })?;

                let mut schema = String::from("CREATE TABLE x(");
                for (index, col) in table.column_iter().enumerate() {
                    if index > 0 {
                        schema.push_str(", ");
                    }
                    schema.push_str(col.name().as_ref());
                    schema.push(' ');
                    schema.push_str(col.value_type().to_sqlite_type());
                }
                schema.push_str(");");

                let vtab = FdbTab {
                    base: rusqlite::vtab::sqlite3_vtab::default(),
                    table,
                };

                Ok((schema, vtab))
            }
            _ => Err(rusqlite::Error::InvalidParameterCount(args.len(), 1)),
        }
    }

    fn best_index(&self, info: &mut rusqlite::vtab::IndexInfo) -> rusqlite::Result<()> {
        info.set_estimated_cost(1_000_000.0);
        Ok(())
    }

    fn open(&'vtab self) -> rusqlite::Result<Self::Cursor> {
        Ok(FdbTabCursor {
            base: rusqlite::vtab::sqlite3_vtab_cursor::default(),
            iter: BufferedIter::new(self.table),
        })
    }
}

impl<'vtab> CreateVTab<'vtab> for FdbTab<'vtab> {
    fn create(
        db: &mut rusqlite::vtab::VTabConnection,
        aux: Option<&Self::Aux>,
        args: &[&[u8]],
    ) -> rusqlite::Result<(String, Self)> {
        Self::connect(db, aux, args)
    }

    fn destroy(&self) -> rusqlite::Result<()> {
        Ok(())
    }
}
