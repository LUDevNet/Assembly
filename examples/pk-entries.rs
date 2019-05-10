use assembly::pk::reader::{PackFile, PackEntryAccessor};
use assembly::pk::file::{PKEntry};
use assembly::core::reader::{FileError, FileResult};
use std::env;
use std::fs::File;
use std::io::{BufRead, Seek, BufReader, Error as IoError};

fn print_usage(program: &str) {
    println!("Usage: {} FILE", program);
}

#[derive(Debug)]
enum MainError {
    Io(IoError),
    File(FileError),
}

impl From<FileError> for MainError {
    fn from(e: FileError) -> Self {
        MainError::File(e)
    }
}

fn print_entries<'b,'a,T>(
    entries: &mut PackEntryAccessor<'b,'a,T>,
    entry: Option<FileResult<PKEntry>>
)
where T: BufRead + Seek, {
    match entry {
        Some(Ok(data)) => {
            {
                let left = entries.get_entry(data.left);
                print_entries(entries, left);
            }
            println!("{:?}", data);
            {
                let right = entries.get_entry(data.right);
                print_entries(entries, right);
            }
        },
        Some(Err(e)) => println!("{:?}", e),
        None => {}
    }
}

fn main() -> Result<(),MainError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() <= 1 {
        print_usage(&program);
        Ok(())
    } else {
        let filename = args[1].clone();
        let file = File::open(filename).map_err(MainError::Io)?;
        let mut reader = BufReader::new(file);
        let mut pack = PackFile::open(&mut reader);

        let header = pack.get_header()?;
        println!("{}, {}", header.file_list_base_addr, header.value_1);

        let mut entries = pack.get_entry_accessor(header.file_list_base_addr)?;
        let root = entries.get_root_entry();
        print_entries(&mut entries, root);
        Ok(())
    }
}
