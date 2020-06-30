extern crate getopts;
use assembly_core::anyhow;
use assembly_data::fdb::reader::{DatabaseBufReader, DatabaseReader};
use getopts::Options;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::{env, fs::File, io::BufReader};

fn run(filename: &str) -> Result<(), anyhow::Error> {
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
            }
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

pub fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
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
