use argh::FromArgs;
use assembly_core::reader::FileResult;
use assembly_pack::pk::file::PKEntry;
use assembly_pack::pk::reader::{PackEntryAccessor, PackFile};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::path::PathBuf;

fn print_entries<T>(
    args: &Args,
    entries: &mut PackEntryAccessor<'_, '_, T>,
    entry: Option<FileResult<PKEntry>>,
) where
    T: BufRead + Seek,
{
    match entry {
        Some(Ok(data)) => {
            {
                let left = entries.get_entry(data.left);
                print_entries(args, entries, left);
            }
            if args.json {
                let json = if args.pretty {
                    serde_json::to_string_pretty(&data)
                } else {
                    serde_json::to_string(&data)
                }
                .unwrap();
                println!("{}", json);
            } else {
                println!(
                    "{:10} {:9} {:9} {} {} {:08x}",
                    data.crc,
                    data.orig_file_size,
                    data.compr_file_size,
                    data.orig_file_hash,
                    data.compr_file_hash,
                    data.is_compressed,
                );
            }
            {
                let right = entries.get_entry(data.right);
                print_entries(args, entries, right);
            }
        }
        Some(Err(e)) => println!("{:?}", e),
        None => {}
    }
}

#[derive(FromArgs)]
/// List the entries in a PK file
struct Args {
    #[argh(positional)]
    /// the PK file
    file: PathBuf,

    // #[argh(option)]
    // /// the manifest to use
    // manifest: Option<PathBuf>,
    #[argh(switch)]
    /// output as json
    json: bool,

    #[argh(switch)]
    /// make the output pretty
    pretty: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Args = argh::from_env();

    let file = File::open(&args.file)?;
    let mut reader = BufReader::new(file);
    let mut pack = PackFile::open(&mut reader);

    let header = pack.get_header()?;

    //if let Some(m) = args.manifest {
    //    println!("Manifest: {}", m.display());
    //}

    let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
    let root = entries.get_root_entry();
    print_entries(&args, &mut entries, root);
    Ok(())
}
