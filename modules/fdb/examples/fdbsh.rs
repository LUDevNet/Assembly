use std::{fmt::Write, fs::File, path::PathBuf};

use argh::FromArgs;
use assembly_fdb::{mem, sqlite};
use color_eyre::eyre::Context;
use mapr::Mmap;
use rusqlite::Connection;
use rustyline::error::ReadlineError;

#[derive(FromArgs)]
/// turns an FDB file into an equivalent SQLite file
struct Options {
    /// the FD source file
    #[argh(positional)]
    src: PathBuf,
}

fn exec(conn: &rusqlite::Connection, sql: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(sql)?;
    let col_count = stmt.column_count();
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(prettytable::Row::new(
        stmt.column_names()
            .into_iter()
            .map(prettytable::Cell::new)
            .collect(),
    ));

    let mut rows = stmt.query([])?;

    let mut buf = String::new();
    while let Some(row) = rows.next()? {
        let mut cells = Vec::with_capacity(col_count);
        for idx in 0..col_count {
            let value = row.get_ref(idx)?;
            match value {
                rusqlite::types::ValueRef::Null => cells.push(prettytable::Cell::new("null")),
                rusqlite::types::ValueRef::Integer(i) => {
                    let _ = write!(buf, "{}", i);
                    cells.push(prettytable::Cell::new(&buf));
                }
                rusqlite::types::ValueRef::Real(r) => {
                    let _ = write!(buf, "{}", r);
                    cells.push(prettytable::Cell::new(&buf));
                }
                rusqlite::types::ValueRef::Text(t) => {
                    let text = std::str::from_utf8(t).unwrap();
                    let _ = write!(buf, "{:?}", text);
                    cells.push(prettytable::Cell::new(&buf));
                }
                rusqlite::types::ValueRef::Blob(bytes) => {
                    let _ = write!(buf, "{:?}", bytes);
                    cells.push(prettytable::Cell::new(&buf));
                }
            }
            buf.clear();
        }
        table.add_row(prettytable::Row::new(cells));
    }
    table.printstd();
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    let opts: Options = argh::from_env();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let mmap: &'static Mmap = Box::leak(Box::new(mmap));
    let buffer: &'static [u8] = mmap;

    let db = mem::Database::new(buffer);
    let conn = Connection::open_in_memory()?;
    sqlite::load_module(&conn, db)?;

    for table in db.tables()?.iter() {
        let table = table?;
        let name = table.name();

        if name == "DBExclude" {
            println!("Skipping {:?}", name); // FIXME
            continue;
        }

        let sql = format!("CREATE VIRTUAL TABLE {} USING fdb;", name);
        //println!("Running: {}", sql);
        conn.execute(&sql, [])?;
    }

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if let Some(x) = line.strip_prefix('.') {
                    match x {
                        "exit" => break,
                        "help" => {
                            println!("Commands:");
                            println!(" exit");
                            println!(" help");
                        }
                        _ => println!("Unknown command {:?}", x),
                    }
                } else {
                    match exec(&conn, &line) {
                        Ok(()) => {}
                        Err(e) => println!("ERROR: {}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(_) => println!("No input"),
        }
    }

    Ok(())
}
