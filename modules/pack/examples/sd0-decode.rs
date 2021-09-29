use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use argh::FromArgs;
use assembly_pack::sd0::stream::SegmentedStream;
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
    let mut stream = SegmentedStream::new(&mut buf)?;

    let out = File::create(args.output)?;
    let mut writer = BufWriter::new(out);

    std::io::copy(&mut stream, &mut writer).context("Streaming sd0 file")?;

    Ok(())
}
