use assembly_core::reader::FileResult;
use assembly_pack::pk::file::PKEntry;
use assembly_pack::pk::reader::{PackEntryAccessor, PackFile};
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn print_usage(program: &str) {
    println!("Usage: {} FILE", program);
}

fn print_entry<'b, 'a, T>(
    entries: &'b mut PackEntryAccessor<'b, 'a, T>,
    entry: Option<FileResult<PKEntry>>,
    crc: u32,
) where
    T: BufRead + Seek,
{
    match entry {
        Some(Ok(data)) => {
            match data.crc.cmp(&crc) {
                Ordering::Less => {
                    let right = entries.get_entry(data.right);
                    print_entry(entries, right, crc);
                }
                Ordering::Greater => {
                    let left = entries.get_entry(data.left);
                    print_entry(entries, left, crc);
                }
                Ordering::Equal => {
                    let mut stream = entries.get_file_mut().get_file_data(data).unwrap();
                    //let mut null = OpenOptions::new().write(true).read(false).open("/dev/null").unwrap();
                    let mut stdout = std::io::stdout();
                    //let mut out = File::create("test.bin").unwrap();
                    std::io::copy(&mut stream, &mut stdout).unwrap();
                }
            }
        }
        Some(Err(e)) => println!("{:?}", e),
        None => {}
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() <= 2 {
        print_usage(&program);
        Ok(())
    } else {
        let filename = args[1].clone();
        let crc = str::parse::<u32>(&args[2]).unwrap();
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut pack = PackFile::open(&mut reader);

        let header = pack.get_header()?;

        let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
        let root = entries.get_root_entry();
        print_entry(&mut entries, root, crc);
        Ok(())
    }
}
