extern crate getopts;
use assembly_core::anyhow;
use assembly_data::fdb::align::Database;
use memmap::Mmap;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::fs::File;
use structopt::StructOpt;

/// Shows all tables in an FDB file
#[derive(StructOpt)]
struct Options {
    /// A database file in FDB format
    file: String,
}

pub fn main() -> Result<(), anyhow::Error> {
    // Load the options
    let opt = Options::from_args();
    assembly_core::time(|| {
        // load the filed
        let file = File::open(&opt.file)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let buffer: &[u8] = &mmap;

        // create the database handle
        let db = Database::new(buffer);

        // prepare the output
        let mut count = 0;
        let mut output = PTable::new();
        output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        output.set_titles(PRow::new(vec![PCell::new("Name")]));

        // loop through all tables
        let tables = db.tables();
        for table in tables.iter() {
            // get the name
            let name = table.name();

            // add a row
            let mut fv = Vec::with_capacity(1);
            fv.push(PCell::new(&name));
            output.add_row(PRow::new(fv));

            count += 1;
        }

        // print the table
        output.printstd();
        // print the summary
        println!("Printed {} row(s)", count);

        Ok(())
    })
}
