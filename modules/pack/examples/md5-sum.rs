use std::{boxed::Box, fs::File, io::Read, path::PathBuf};

use argh::FromArgs;
use assembly_pack::md5::io::IOSum;

#[derive(FromArgs)]
/// calculate an md5 sum
struct Args {
    #[argh(positional)]
    filename: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(&args.filename)?;
    let mut md5sum = IOSum::new(file);

    let mut buf: Box<[u8]> = Box::from([0u8; 1204 * 16]);

    let mut c = 1;
    while c > 0 {
        c = md5sum.read(buf.as_mut())?;
    }

    let (_, digest) = md5sum.into_inner();
    println!("{:?}", digest);

    Ok(())
}
