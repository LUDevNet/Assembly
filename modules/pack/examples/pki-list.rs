use argh::FromArgs;
use assembly_pack::pki::core::PackIndexFile;
use std::{convert::TryFrom, fs::File, path::PathBuf};

#[derive(FromArgs)]
/// Show contents of a PKI file
struct Args {
    /// print all pack files
    #[argh(switch, short = 'p')]
    pack_files: bool,

    /// print all files
    #[argh(switch, short = 'f')]
    files: bool,

    /// the PKI file
    #[argh(positional)]
    path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(&args.path)?;
    let file = PackIndexFile::try_from(file)?;

    if args.pack_files {
        for pack in file.archives {
            println!("{}", pack.path);
        }
        Ok(())
    } else if args.files {
        for (key, file_ref) in file.files {
            let pack_index = file_ref.pack_file as usize;
            match file.archives.get(pack_index) {
                Some(pack_ref) => {
                    println!("{:>10} {:08x} {}", key, file_ref.category, pack_ref.path);
                }
                None => println!("Pack ID {} out of bounds", pack_index),
            }
        }
        Ok(())
    } else {
        eprintln!("Please specify either `-f` or `-p`");
        Ok(())
    }
}
