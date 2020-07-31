use anyhow::{anyhow, Result};
use assembly_maps::lvl::{parser::parse_objects_chunk_data, reader::LevelReader};
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

    let header_1000 = lvl.get_chunk_header()?;
    println!("{:?}", header_1000);

    if !header_1000.id == 1000 {
        return Err(anyhow!("Expected first chunk to be of type 1000"));
    }

    lvl.seek_to(&header_1000)?;
    let meta = lvl.get_meta_chunk_data()?;

    println!("{:?}", meta);
    if let Some(res) = lvl.seek_meta(&meta, 2001) {
        res?;
        let header_2001 = lvl.get_chunk_header()?;
        println!("{:?}", header_2001);

        if !header_2001.id == 20001 {
            return Err(anyhow!("Expected 2001 chunk to be of type 2001"));
        }

        let buf = lvl.load_buf(meta.chunk_2001_offset, &header_2001)?;
        let obj = parse_objects_chunk_data(meta.version, &buf)
            .map_err(|e| anyhow!("Could not parse objects chunk:\n{}", e))?
            .1;

        let obj = obj
            .parse_settings()
            .map_err(|_| anyhow!("Failed to parse object settings"))?;

        println!("{:#?}", obj);
    }

    Ok(())
}
