use anyhow::Error;
use assembly_data::fdb::de::buffer::Buffer;
use memmap::Mmap;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    file: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();

    assembly_core::time(|| {
        let file = File::open(&opt.file)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let buf: &[u8] = &mmap;
        let db = Buffer::new(buf);

        let header = db.header_ref()?;
        println!("{:?}", header);

        let table_headers = db.table_headers(header)?;
        println!("{:?}", table_headers);

        Ok(())
    })
}
