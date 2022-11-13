use argh::FromArgs;
use assembly_fdb::mem::Database;
use mapr::Mmap;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::fs::File;

/// Shows all tables in an FDB file
#[derive(FromArgs)]
struct Options {
    /// a database file in FDB format
    #[argh(positional)]
    file: String,
}

pub fn main() -> color_eyre::Result<()> {
    // Load the options
    let opts: Options = argh::from_env();
    assembly_core::time(|| {
        // load the file
        let file = File::open(&opts.file)?;
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
        let tables = db.tables()?;
        for table in tables.iter() {
            let table = table?;

            // get the name
            let name = table.name();

            // add a row
            let fv = vec![PCell::new(&name)];
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
