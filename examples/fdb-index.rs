extern crate getopts;
use std::io::{BufReader, Error as IoError};
use std::fs::File;
use assembly::fdb::io::{SchemaLoader, LoaderConfigImpl, LoadError};
use prettytable::{Table as PTable, Row as PRow, Cell as PCell};
use getopts::Options;
use std::env;

#[derive(Debug)]
pub enum MainError {
    Io(IoError),
    Load(LoadError),
    TableNotFound,
}

impl From<IoError> for MainError {
    fn from(e: IoError) -> Self {
        MainError::Io(e)
    }
}

impl From<LoadError> for MainError {
    fn from(e: LoadError) -> Self {
        MainError::Load(e)
    }
}

fn run(filename: &str, tablename: &str, key: &str) -> Result<(), MainError> {
    //println!("Loading tables... (this may take a while)");
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let config = LoaderConfigImpl {
        table_data_policy: |def| def.name == tablename,
    };
    let mut loader = SchemaLoader::open(&mut reader, config);
    let schema = loader.try_load_schema()?;

    match schema.table(tablename) {
        Some(table) => {
            let pk_filter = table.columns_ref()[0].pk_filter(key).unwrap();
            let hash = pk_filter.hash();

            let buckets = table.buckets_ref();
            let index = hash as usize % buckets.len();

            let mut count = 0;
            let mut output = PTable::new();
            output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            output.set_titles(PRow::new(table.columns_ref().iter().map(|c| PCell::new(&c.name)).collect()));

            for row in buckets[index].rows_ref() {
                if pk_filter.filter(&row.fields_ref()[0]) {
                    output.add_row(PRow::new(row.fields_ref().iter().map(|f| PCell::new(&f.to_string())).collect()));
                    count += 1;
                }
            }

            output.printstd();
            println!("Printed {} row(s)", count);

            Ok(())
        },
        None => Err(MainError::TableNotFound),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE TABLE KEY [options]", program);
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
    let (file, table, key) = if matches.free.len() >= 3 {
        (&matches.free[0], &matches.free[1], &matches.free[2])
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    run(file, table, key)
}
