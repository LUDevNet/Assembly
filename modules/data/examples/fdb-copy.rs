use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

use assembly_data::fdb::{core, mem, store};
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
    let start = Instant::now();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let mmap = unsafe { Mmap::map(&src_file)? };
    let buffer: &[u8] = &mmap;

    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);

    let src_db = mem::Database::new(buffer);
    let mut dest_db = store::Database::new();

    for src_table in src_db.tables()?.iter() {
        let src_table = src_table?;

        let mut dest_table = store::Table::new(src_table.bucket_count());

        for src_column in src_table.column_iter() {
            dest_table.push_column(src_column.name_raw(), src_column.value_type());
        }

        let mut row_buffer: Vec<core::Field> = Vec::with_capacity(src_table.column_count());

        for (pk, src_bucket) in src_table.bucket_iter().enumerate() {
            for src_row in src_bucket.row_iter() {
                for field in src_row.field_iter() {
                    row_buffer.push(match field {
                        mem::Field::Nothing => core::Field::Nothing,
                        mem::Field::Integer(v) => core::Field::Integer(v),
                        mem::Field::Float(v) => core::Field::Float(v),
                        mem::Field::Text(v) => core::Field::Text(v.decode().into_owned()),
                        mem::Field::Boolean(v) => core::Field::Boolean(v),
                        mem::Field::BigInt(v) => core::Field::BigInt(v),
                        mem::Field::VarChar(v) => core::Field::VarChar(v.decode().into_owned()),
                    });
                }
                dest_table.push_row(pk, &row_buffer[..]);
                row_buffer.clear();
            }
        }

        dest_db.push_table(src_table.name_raw(), dest_table);
    }

    dest_db
        .write(&mut dest_out)
        .wrap_err("Failed to write copied database")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
