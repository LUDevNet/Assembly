use std::env;
use assembly::pki::core::PackIndexFile;
use assembly::pki::io::{LoadError};
use std::convert::TryFrom;
use std::num::TryFromIntError;

#[derive(Debug)]
enum MainError {
    Load(LoadError),
    TFI(TryFromIntError),
}

impl From<LoadError> for MainError {
    fn from(e: LoadError) -> Self {
        MainError::Load(e)
    }
}

impl From<TryFromIntError> for MainError {
    fn from(e: TryFromIntError) -> Self {
        MainError::TFI(e)
    }
}

fn print_usage(program: &str) {
    println!("Usage: {} PATH CRC", program);
}

fn main() -> Result<(),MainError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() <= 2 {
        print_usage(&program);
        Ok(())
    } else {
        let filename = args[1].clone();
        let crc = str::parse::<u32>(&args[2]).unwrap();

        let pki = PackIndexFile::try_from(filename.as_ref())?;

        match pki.files.get(&crc) {
            Some(file_ref) => {
                let pack_index = usize::try_from(file_ref.pack_file)?;
                match pki.archives.get(pack_index) {
                    Some(pack_ref) => {
                        println!("{:x} {}", file_ref.category, pack_ref.path);
                    }
                    None => println!("Pack ID {} out of bounds", pack_index),
                }
            },
            None => println!("File not found"),
        }
        Ok(())
    }
}
