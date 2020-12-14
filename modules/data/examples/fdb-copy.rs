use std::{fs::File, io::BufWriter, path::PathBuf};

use assembly_data::fdb::{mem, store};
use mapr::Mmap;
use structopt::StructOpt;

use color_eyre::eyre::{self, WrapErr};

#[derive(StructOpt)]
/// Copies one FDB file to another
struct Options {
    /// The FDB file to copy from
    src: PathBuf,
    /// The FDB file to create
    dest: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);

    let _src_db = mem::Database::new(buffer);
    let dest_db = store::Database::new();

    // TODO

    dest_db
        .write(&mut dest_out)
        .wrap_err("Failed to write copied database")?;

    Ok(())
}
