use anyhow::{anyhow, Context};
use assembly_fdb::{
    mem::{Database, Table},
    value::Value,
};
use assembly_sysdiagram::SysDiagram;
use mapr::Mmap;
use std::fs::File;
use std::{convert::TryFrom, path::PathBuf};

#[derive(argh::FromArgs)]
/// parse a sysdiagram from a FDB file
struct Options {
    /// path to the FDB file
    #[argh(positional)]
    file: PathBuf,
}

fn load_database(opts: &Options) -> Result<(), anyhow::Error> {
    println!("Loading tables... (this may take a while)");

    // Load the database file
    let file = File::open(&opts.file)
        .with_context(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let table = db
        .tables()?
        .by_name("sysdiagrams")
        .ok_or_else(|| anyhow!("Failed to find table sysdiagrams"))?;
    let table: Table = table.context("Failed to load table 'sysdiagrams'")?;

    for row in table.row_iter() {
        match row.field_at(4) {
            Some(Value::Text(text)) => {
                let text = text.decode();
                let sysdiagram = SysDiagram::try_from(text.as_ref())?;
                for table in sysdiagram.tables {
                    println!("{}.{}", table.sch_grid.schema, table.sch_grid.name);
                }
                for relationship in sysdiagram.relationships {
                    println!(
                        "{:60} {:25} {:25}",
                        relationship.name, relationship.from, relationship.to
                    );
                }
                for (key, value) in sysdiagram.dsref_schema_contents.settings.iter() {
                    println!("{:25}: {}", key, value);
                }
            }
            data => println!("Wrong data: {:?}", data),
        }
    }
    Ok(())
}

pub fn main() -> Result<(), anyhow::Error> {
    let opts: Options = argh::from_env();
    load_database(&opts).with_context(|| "Loading database failed!")
}
