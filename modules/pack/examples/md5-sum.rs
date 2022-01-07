use std::path::PathBuf;

use argh::FromArgs;
use assembly_pack::md5::md5sum;

#[derive(FromArgs)]
/// calculate an md5 sum
struct Args {
    #[argh(positional)]
    filename: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let meta = md5sum(&args.filename)?;
    println!("{:?}", meta.hash);

    Ok(())
}
