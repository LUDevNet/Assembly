use std::path::PathBuf;

use argh::FromArgs;
use assembly_pack::txt::Manifest;

#[derive(FromArgs)]
/// Sort entries in a manifest file
struct Args {
    /// a manifest file (*.txt)
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let mf = Manifest::from_file(&args.file)?;

    println!("[version]");
    println!("{}", mf.version);

    println!("[files]");
    for (k, (meta, hash)) in mf.files {
        println!("{},{},{}", k, meta, hash);
    }

    Ok(())
}
