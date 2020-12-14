use assembly_data::fdb::mem::{Database, Tables};
use mapr::Mmap;
use std::{fs::File, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prints the names of all tables and their columns
struct Options {
    /// The FDB file
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let opt = Options::from_args();
    let start = Instant::now();

    let file = File::open(&opt.file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    let db = Database::new(buffer);
    let tables: Tables<'_> = db.tables()?;
    println!("#Tables: {}", tables.len());

    for table in tables.iter() {
        let table = table?;
        let table_name = table.name();
        println!("{} ({})", table_name, table.bucket_count());

        for column in table.column_iter() {
            let name = column.name();
            println!("- {}: {:?}", name, column.value_type());
        }
    }

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
