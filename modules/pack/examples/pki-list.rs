use assembly_pack::pki::{core::PackIndexFile, io::LoadError};
use getopts::Options;
use std::convert::TryFrom;
use std::env;

#[derive(Debug)]
pub enum MainError {
    Load(LoadError),
}

impl From<LoadError> for MainError {
    fn from(e: LoadError) -> Self {
        MainError::Load(e)
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), MainError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "pack-files", "print all pack files");
    opts.optflag("f", "files", "print all files");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    let file = if !matches.free.is_empty() {
        let filename = matches.free[0].clone();
        PackIndexFile::try_from(filename.as_ref())?
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    if matches.opt_present("p") {
        for pack in file.archives {
            println!("{}", pack.path);
        }
        Ok(())
    } else if matches.opt_present("f") {
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
        print_usage(&program, opts);
        Ok(())
    }
}
