use std::path::PathBuf;

use structopt::StructOpt;

use color_eyre::eyre;

#[derive(StructOpt)]
/// Copies one FDB file to another
struct Options {
    /// The FDB file to copy from
    src: PathBuf,
    /// The FDB file to create
    dest: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let _ = opts.src;
    let _ = opts.dest;

    Ok(())
}
