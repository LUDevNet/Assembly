use argh::FromArgs;
use assembly_pack::pk::reader::PackFile;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Print a single entry from a PK file
struct Args {
    /// an ndpk file
    #[argh(positional)]
    path: PathBuf,

    /// the CRC of a resource path
    #[argh(positional)]
    crc: u32,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(args.path)?;
    let mut reader = BufReader::new(file);
    let mut pack = PackFile::open(&mut reader);

    let header = pack.get_header()?;

    let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
    if let Some(entry) = entries.find_entry(args.crc)? {
        let mut stream = entries.get_file_mut().get_file_data(entry).unwrap();
        let mut stdout = std::io::stdout();
        std::io::copy(&mut stream, &mut stdout).unwrap();
    }

    Ok(())
}
