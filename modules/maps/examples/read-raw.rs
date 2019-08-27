extern crate structopt;

use assembly_maps::raw::reader::*;
use assembly_core::byteorder::{LE, ReadBytesExt};

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "read-raw", about = "Analyze a LU Terrain File.")]
struct Opt {
    /// Input file (`*raw`)
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    FileNotFound,
}

pub fn main() -> Result<(),Error> {
    let opt = Opt::from_args();

    if !opt.input.exists() || !opt.input.is_file() {
        return Err(Error::FileNotFound);
    }

    let file = File::open(opt.input.as_path()).unwrap();
    let mut buf = BufReader::new(file);
    let header = buf.read_terrain_header().unwrap();
    let chunk1 = buf.read_terrain_chunk().unwrap();
    let hmh = buf.read_height_map_header().unwrap();
    let _hm_data = buf.read_height_map_data(hmh.width, hmh.height).unwrap();
    let _cm_data = buf.read_color_map_data().unwrap();
    let lm_data = buf.read_embedded_file().unwrap();
    let _cm2_data = buf.read_color_map_data().unwrap();

    let _1 = buf.read_u8().unwrap();
    let lm2_data = buf.read_embedded_file().unwrap();
    let _2 = buf.read_i32::<LE>().unwrap();

    println!("{:?}", header);
    println!("{:?}", chunk1);
    dbg!(_1);
    dbg!(_2);

    let mut out = File::create("out.dds").unwrap();
    out.write(lm_data.as_slice()).unwrap();
    let mut out = File::create("out2.dds").unwrap();
    out.write(lm2_data.as_slice()).unwrap();
    Ok(())
}
