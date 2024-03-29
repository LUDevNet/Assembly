use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use argh::FromArgs;
use assembly_pack::sd0::read::SegmentedDecoder;
use color_eyre::eyre::Context;

#[derive(Debug, FromArgs)]
/// decompress an sd0 file
struct Args {
    /// the input file
    #[argh(positional)]
    input: PathBuf,
    /// the output file
    #[argh(positional)]
    output: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let file = File::open(&args.input)?;
    let mut buf = BufReader::new(file);
    let mut stream = SegmentedDecoder::new(&mut buf)?;

    let out = File::create(args.output)?;
    let mut writer = BufWriter::with_capacity(1024 << 4, out);

    std::io::copy(&mut stream, &mut writer).context("Streaming sd0 file")?;

    Ok(())
}
