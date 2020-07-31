use anyhow::Result;
use assembly_maps::lvl::reader::LevelReader;
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// The lvl file to analyze
    file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let file = File::open(&opt.file)?;
    let br = BufReader::new(file);
    let mut lvl = LevelReader::new(br);

    let level = lvl.read_level_file()?;
    println!("{:#?}", level);

    Ok(())
}
