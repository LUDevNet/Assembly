use assembly_core::reader::FileResult;
use assembly_pack::pk::file::PKEntry;
use assembly_pack::pk::reader::{PackEntryAccessor, PackFile};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn print_usage(program: &str) {
    println!("Usage: {} FILE", program);
}

fn print_entries<'b, 'a, T>(
    entries: &mut PackEntryAccessor<'b, 'a, T>,
    entry: Option<FileResult<PKEntry>>,
) where
    T: BufRead + Seek,
{
    match entry {
        Some(Ok(data)) => {
            {
                let left = entries.get_entry(data.left);
                print_entries(entries, left);
            }
            println!(
                "{:10} {:9} {:9} {} {}",
                data.crc,
                data.orig_file_size,
                data.compr_file_size,
                data.orig_file_hash,
                data.compr_file_hash
            );
            {
                let right = entries.get_entry(data.right);
                print_entries(entries, right);
            }
        }
        Some(Err(e)) => println!("{:?}", e),
        None => {}
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() <= 1 {
        print_usage(&program);
        Ok(())
    } else {
        let filename = args[1].clone();
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut pack = PackFile::open(&mut reader);

        let header = pack.get_header()?;

        let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
        let root = entries.get_root_entry();
        print_entries(&mut entries, root);
        Ok(())
    }
}
