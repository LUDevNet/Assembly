use argh::FromArgs;
use assembly_pack::pk::reader::PackFile;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(FromArgs)]
/// List the entries in a PK file
struct Args {
    #[argh(positional)]
    /// the PK file
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(args.file)?;
    let mut reader = BufReader::new(file);
    let mut pack = PackFile::open(&mut reader);

    let header = pack.get_header()?;
    println!("{:#?}", header);

    let list = pack.get_entry_list(header.file_list_base_addr)?;
    println!("count = {}", list.len());
    Ok(())
}
