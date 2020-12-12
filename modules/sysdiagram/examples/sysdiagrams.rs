use anyhow::{anyhow, Context};
use assembly_data::fdb::core::Field;
use assembly_data::fdb::io::{LoaderConfigImpl, SchemaLoader};
use assembly_sysdiagram::core::SysDiagram;
use getopts::Options;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn load_database(filename: &str) -> Result<(), anyhow::Error> {
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
                match &row.fields()[4] {
                    Field::Text(text) => {
                        let sysdiagram = SysDiagram::try_from(&text[..])?;
                        /*for table in sysdiagram.tables {
                            println!("{}.{}", table.sch_grid.schema, table.sch_grid.name);
                        }
                        for relationship in sysdiagram.relationships {
                            println!("{:60} {:25} {:25}", relationship.name, relationship.from, relationship.to);
                        }*/
                        for (key, value) in sysdiagram.dsref_schema_contents.settings.iter() {
                            println!("{:25}: {}", key, value);
                        }
                    }
                    data => println!("Wrong data: {:?}", data),
                }
            }
            Ok(())
        }
        None => Err(anyhow!("Table not found!")),
    }
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
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    load_database(&input).with_context(|| "Loading database failed!")
}
