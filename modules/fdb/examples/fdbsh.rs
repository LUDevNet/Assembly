use std::{fs::File, path::PathBuf};

use argh::FromArgs;
use assembly_fdb::mem;
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
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        for idx in 0..col_count {
            let value = row.get_ref(idx)?;
            match value {
                rusqlite::types::ValueRef::Null => print!(" null"),
                rusqlite::types::ValueRef::Integer(i) => print!(" {}", i),
                rusqlite::types::ValueRef::Real(r) => print!(" {}", r),
                rusqlite::types::ValueRef::Text(t) => {
                    print!(" {:?}", std::str::from_utf8(t).unwrap())
                }
                rusqlite::types::ValueRef::Blob(b) => print!(" {:?}", b),
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    let opts: Options = argh::from_env();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    let _db = mem::Database::new(buffer);
    let conn = Connection::open_in_memory()?;

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
