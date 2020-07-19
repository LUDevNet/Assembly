use anyhow::Error;
use assembly_data::fdb::align::{Database, Tables};
use memmap::Mmap;
use std::{fs::File, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    file: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    let start = Instant::now();

    let file = File::open(&opt.file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    let db = Database::new(buffer);
    let tables: Tables<'_> = db.tables();
    println!("#Tables: {}", tables.len());

    for table in tables.iter() {
        let table_name = table.name();
        println!("{}", table_name);

        for column in table.column_iter() {
            let name = column.name();
            println!("- {}: {:?}", name, column.value_type());
        }

        for bucket in table.bucket_iter() {
            print!("|");
            for _row in bucket.row_iter() {
                print!(".");
            }
        }

        println!("# {}", table.bucket_count());
    }

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
