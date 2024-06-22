use std::{fs::File, path::PathBuf, time::Instant};

use argh::FromArgs;
use assembly_fdb::{mem::Database, sqlite::try_export_db};
use color_eyre::eyre::WrapErr;
use mapr::Mmap;
use rusqlite::Connection;

#[derive(FromArgs)]
/// Turns an FDB file into an equivalent SQLite file
struct Options {
    /// the FD source file
    #[argh(positional)]
    src: PathBuf,
    /// the SQLite destination file
    #[argh(positional)]
    dest: PathBuf,
    /// optional SQL schema to be used instead of FDB schema
    #[argh(option)]
    schema: Option<PathBuf>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts: Options = argh::from_env();
    let start = Instant::now();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    println!("Copying data, this may take a few seconds...");

    let db = Database::new(buffer);
    let mut conn = Connection::open(opts.dest)?;

    if let Some(path) = opts.schema {
        let schema = std::fs::read_to_string(path)?;
        conn.execute_batch(&schema)?;
    }

    conn.execute_batch("PRAGMA foreign_keys = off")?;
    try_export_db(&mut conn, db).wrap_err("Failed to export database to sqlite")?;
    conn.execute_batch("PRAGMA foreign_keys = on")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
