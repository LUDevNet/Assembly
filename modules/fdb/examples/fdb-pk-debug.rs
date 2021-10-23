use assembly_fdb::mem::{Database, Table};
use color_eyre::eyre::{eyre, WrapErr};
use mapr::Mmap;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Shows all rows for a single key in a table
struct Options {
    /// The FDB file
    file: PathBuf,
    /// The table to use
    table: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    // Load the database file
    let file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let table = db
        .tables()?
        .by_name(&opts.table)
        .ok_or_else(|| eyre!("Failed to find table {:?}", &opts.table))?;
    let table: Table = table.wrap_err_with(|| format!("Failed to load table {:?}", &opts.table))?;

    let mut row_count = 0;

    for (bi, bucket) in table.bucket_iter().enumerate() {
        for (ri, row) in bucket.row_iter().enumerate() {
            row_count += 1;
            if let Some(index_field) = row.field_at(0) {
                println!("{},{},{:?}", bi, ri, index_field);
            } else {
                println!("{},{},[index field missing]", bi, ri);
            }
        }
    }

    println!("Printed {} row(s)", row_count);

    Ok(())
}
