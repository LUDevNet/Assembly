use argh::FromArgs;
use assembly_pack::crc::CRC;
use assembly_pack::pki::core::PackIndexFile;
use color_eyre::eyre::eyre;
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;

#[derive(FromArgs)]
/// print the entry for a specific CRC in the PKI
struct Args {
    /// the PKI file
    #[argh(positional)]
    pki_file: PathBuf,

    /// the (decimal) CRC value
    #[argh(positional)]
    crc: u32,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let filename = args.pki_file;
    let crc = args.crc;

    let file = File::open(filename)?;
    let pki = PackIndexFile::try_from(file)?;

    match pki.files.get(&CRC::from_raw(crc)) {
        Some(file_ref) => {
            let pack_index = usize::try_from(file_ref.pack_file)?;
            match pki.archives.get(pack_index) {
                Some(pack_ref) => {
                    println!("{:08x} {}", file_ref.category, pack_ref.path);
                    Ok(())
                }
                None => Err(eyre!("Pack ID {} out of bounds", pack_index)),
            }
        }
        None => Err(eyre!("File not found")),
    }
}
