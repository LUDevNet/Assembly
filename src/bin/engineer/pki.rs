use getopts::Options;
use assembly::pki::{io::LoadError, core::PackIndexFile};
use std::convert::TryFrom;

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

pub fn main(args: Vec<String>) -> Result<(), MainError> {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "pack-files", "print all pack files");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
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
        return Ok(());
    } else {
        print_usage(&program, opts);
        return Ok(());
    }
}
