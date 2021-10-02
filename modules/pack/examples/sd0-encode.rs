use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use argh::FromArgs;
use assembly_pack::sd0::write::SegmentedEncoder;
use color_eyre::eyre::Context;
use flate2::Compression;

#[derive(Debug, FromArgs)]
/// decompress an sd0 file
struct Args {
    /// the input file
    #[argh(positional)]
    input: PathBuf,

    /// the output file
    #[argh(positional)]
    output: PathBuf,

    /// the output file
    #[argh(option, short = 'l', default = "9")]
    level: u32,
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let file = File::open(&args.input)?;
    let mut reader = BufReader::new(file);

    let out = File::create(args.output)?;
    let mut writer = BufWriter::with_capacity(1024 << 4, out);

    let level = Compression::new(args.level);
    let mut stream = SegmentedEncoder::new(&mut writer, level)?;

    std::io::copy(&mut reader, &mut stream).context("Streaming sd0 file")?;

    Ok(())
}
