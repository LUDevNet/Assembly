use anyhow::Error;
use assembly_core::buffer::Unaligned;
use assembly_data::fdb::de::align::FDBHeaderC;
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

    let header = FDBHeaderC::cast(buffer, 0);
    println!("#Tables: {}", header.table_count());

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
