use assembly_fdb::{mem::Database, store};
use color_eyre::eyre::{self, WrapErr};
use mapr::Mmap;
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Creates a FDB file with the same table structure as the input file, but without any rows
struct Options {
    /// Input FDB file
    src: PathBuf,
    /// Destination for template file
    dest: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();
    let start = Instant::now();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);

    println!("Creating template, this may take a few milliseconds...");

    let src_db = Database::new(buffer);
    let mut dest_db = store::Database::new();

    for src_table in src_db.tables()?.iter() {
        let src_table = src_table?;

        let mut dest_table = store::Table::new(0);

        for src_column in src_table.column_iter() {
            dest_table.push_column(src_column.name_raw(), src_column.value_type());
        }

        dest_db.push_table(src_table.name_raw(), dest_table);
    }

    dest_db
        .write(&mut dest_out)
        .wrap_err("Could not write output file")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{:#03}s",
        duration.as_secs(),
        duration.subsec_millis(),
    );

    Ok(())
}
