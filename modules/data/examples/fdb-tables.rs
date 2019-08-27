extern crate getopts;
use std::io::{BufReader, Error as IoError};
use std::fs::File;
use assembly_data::fdb::reader::{DatabaseReader, DatabaseBufReader};
use assembly_data::fdb::builder::{BuildError};
use assembly_core::reader::FileError;
use prettytable::{Table as PTable, Row as PRow, Cell as PCell};
use getopts::Options;
use std::env;

#[derive(Debug)]
pub enum MainError {
    Io(IoError),
    Build(BuildError),
    LowLevel(FileError),
    TableNotFound,
}

impl From<IoError> for MainError {
    fn from(e: IoError) -> Self {
        MainError::Io(e)
    }
}

impl From<BuildError> for MainError {
    fn from(e: BuildError) -> Self {
        MainError::Build(e)
    }
}

impl From<FileError> for MainError {
    fn from(e: FileError) -> Self {
        MainError::LowLevel(e)
    }
}

fn run(filename: &str) -> Result<(), MainError> {
    //println!("Loading tables... (this may take a while)");
    let file = File::open(filename)?;
    let mut loader = BufReader::new(file);

    let h = loader.get_header()?;
    let thl = loader.get_table_header_list(h)?;

    let thlv: Vec<_> = thl.into();
    let mut iter = thlv.iter();

    let mut count = 0;
    let mut output = PTable::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    output.set_titles(PRow::new(vec![PCell::new("Name")]));

    loop {
        match iter.next() {
            Some(th) => {
                let tdh = loader.get_table_def_header(th.table_def_header_addr)?;
                let name = loader.get_string(tdh.table_name_addr)?;

                let mut fv = Vec::with_capacity(2);
                fv.push(PCell::new(&name));
                output.add_row(PRow::new(fv));

                count += 1;
            },
            None => break,
        }
    }

    output.printstd();
    println!("Printed {} row(s)", count);

    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn main() -> Result<(), MainError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    let file = if matches.free.len() >= 1 {
        &matches.free[0]
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    run(file)
}
