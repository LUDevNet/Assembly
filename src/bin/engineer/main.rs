extern crate getopts;
use std::io::{BufReader, Error as IoError};
use std::fs::File;
use assembly::fdb::core::{Schema, Field, ValueType};
use assembly::fdb::io::{SchemaLoader, LoaderConfigImpl, LoadError};
use assembly::fdb::sysdiagram::core::SysDiagram;
use assembly::fdb::sysdiagram::io::LoadError as SysDiagramError;
use std::convert::TryFrom;
use getopts::Options;
use std::env;


#[derive(Debug)]
enum MainError {
    Io(IoError),
    Load(LoadError),
    WrongData(Field),
    SysDiagram(SysDiagramError),
    TableNotFound,
    NotImplemented,
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

impl From<SysDiagramError> for MainError {
    fn from(e: SysDiagramError) -> Self {
        MainError::SysDiagram(e)
    }
}

fn load_database(filename: &str) -> Result<(), MainError> {
    println!("Loading tables... (this may take a while)");
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let config = LoaderConfigImpl {
        table_data_policy: |def| def.name == "sysdiagrams",
    };
    let mut loader = SchemaLoader::open(&mut reader, config);
    let schema = loader.try_load_schema()?;

    match schema.table("sysdiagrams") {
        Some(table) => {
            for row in table {
                match &row.fields_ref()[4] {
                    Field::Text(text) => {
                        let sysdiagram = SysDiagram::try_from(&text[..])?;
                        println!("{:?}", sysdiagram);
                    },
                    data => println!("Wrong data: {:?}", data),
                }
            }
            Ok(())
        },
        None => Err(MainError::TableNotFound),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), MainError> {
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
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    load_database(&input)
}
