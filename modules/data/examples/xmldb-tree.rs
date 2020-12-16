use assembly_data::xml::{
    common::{expect_decl, expect_end},
    database::{
        expect_column_or_end_columns, expect_columns, expect_database, expect_row_or_end_rows,
        expect_rows, expect_table,
    },
};
use color_eyre::eyre::WrapErr;
use std::{fs::File, io::BufReader, path::PathBuf};

use quick_xml::Reader;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prints the names of all tables and their columns
struct Options {
    /// The FDB file
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let src_file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let reader = BufReader::new(src_file);

    let mut xml = Reader::from_reader(reader);
    let xml = &mut xml;

    let mut buf = Vec::new();
    let buf = &mut buf;

    expect_decl(xml, buf)?;
    let db_name = expect_database(xml, buf)?.unwrap();
    println!("Loading database: '{}'", db_name);

    while let Some(table_name) = expect_table(xml, buf)? {
        println!("table '{}'", table_name);

        expect_columns(xml, buf)?;

        while let Some(col) = expect_column_or_end_columns(xml, buf)? {
            println!("column '{}' ({:?})", col.name, col.data_type);
        }

        expect_rows(xml, buf)?;

        while let Some(_row) = expect_row_or_end_rows(xml, buf)? {
            //println!("row {:?}", row);
        }

        expect_end(xml, buf, "table")?;
    }

    Ok(())
}
