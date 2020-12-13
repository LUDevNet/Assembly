use assembly_data::fdb::ro::buffer::Buffer;
use mapr::Mmap;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
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
