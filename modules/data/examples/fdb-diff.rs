use std::{fs::File, path::PathBuf, time::Instant};

use assembly_data::fdb::mem;
use mapr::Mmap;
use structopt::StructOpt;

use color_eyre::eyre::{self, WrapErr};

#[derive(StructOpt)]
/// Finds differences in FDB files
struct Options {
    /// The 'left' FDB file
    left: PathBuf,
    /// The 'right' FDB file
    right: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();
    let start = Instant::now();

    // load left file
    let left_file = File::open(&opts.left)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.left.display()))?;
    let left_mmap = unsafe { Mmap::map(&left_file)? };
    let left_buffer: &[u8] = &left_mmap;

    // load right file
    let right_file = File::open(&opts.right)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.right.display()))?;
    let right_mmap = unsafe { Mmap::map(&right_file)? };
    let right_buffer: &[u8] = &right_mmap;

    let left_db = mem::Database::new(left_buffer);
    let right_db = mem::Database::new(right_buffer);

    // FIXME: check whether the DB is sorted correctly

    let mut _left_table_iter = left_db.tables()?.iter();
    let mut _right_table_iter = right_db.tables()?.iter();

    // FIXME: write diff tool

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
