use std::{fs::File, path::PathBuf, time::Instant};

use assembly_data::fdb::{mem::Database, sqlite::try_export_db};
use color_eyre::eyre::WrapErr;
use mapr::Mmap;
use rusqlite::Connection;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Turns an FDB file into an equivalent SQLite file
struct Options {
    /// The FD source file
    src: PathBuf,
    /// The SQLite destination file
    dest: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();
    let start = Instant::now();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    println!("Copying data, this may take a few seconds...");

    let db = Database::new(buffer);
    let mut conn = Connection::open(opts.dest)?;

    try_export_db(&mut conn, db).wrap_err("Failed to export database to sqlite")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
