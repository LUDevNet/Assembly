use assembly_data::fdb::mem::{Database, Table};
use color_eyre::eyre::{eyre, WrapErr};
use mapr::Mmap;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

/// Show all columns and their types for some table
#[derive(StructOpt)]
struct Options {
    /// The FDB file
    file: PathBuf,
    /// The name of a table
    table: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    // Load the database file
    let file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let table = db
        .tables()?
        .by_name(&opts.table)
        .ok_or_else(|| eyre!("Failed to find table {:?}", &opts.table))?;
    let table: Table = table.wrap_err_with(|| format!("Failed to load table {:?}", &opts.table))?;

    let mut count = 0;
    let mut output = PTable::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    output.set_titles(PRow::new(vec![PCell::new("Name"), PCell::new("Type")]));

    for column in table.column_iter() {
        let column_name = column.name();
        let value_type = column.value_type();

        let cr = PRow::new(vec![
            PCell::new(column_name.as_ref()),
            PCell::new(value_type.static_name()),
        ]);

        output.add_row(cr);
        count += 1;
    }

    output.printstd();
    println!("Printed {} row(s)", count);

    Ok(())
}
