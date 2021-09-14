use assembly_fdb::{
    core,
    mem::{self, Database, Field, Table},
    query::pk_filter,
};
use color_eyre::eyre::{eyre, WrapErr};
use mapr::Mmap;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Shows all rows for a single key in a table
struct Options {
    /// The FDB file
    file: PathBuf,
    /// The table to use
    table: String,
    /// The key to use
    key: String,
}

fn field_to_cell(field: mem::Field) -> PCell {
    match field {
        Field::Nothing => PCell::new("NULL"),
        Field::Integer(v) => PCell::new(&format!("{} (i32)", v)),
        Field::Float(v) => PCell::new(&format!("{} (f32)", v)),
        Field::Text(v) => PCell::new(&format!("{:?}", v)),
        Field::Boolean(v) => PCell::new(if v { "true" } else { "false" }),
        Field::BigInt(v) => PCell::new(&format!("{} (i64)", v)),
        Field::VarChar(v) => PCell::new(&format!("{:?}", v)),
    }
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
    let index_col = table.column_at(0).expect("Table has no columns");

    // Setup key filer
    let value_type = index_col.value_type();
    let filter = pk_filter(opts.key, value_type)?;

    // Find the bucket
    let bucket = table
        .bucket_at(filter.hash() as usize % table.bucket_count())
        .ok_or_else(|| eyre!("Failed to find bucket"))?;

    // Collect relevant rows
    let mut rows: Vec<_> = bucket
        .row_iter()
        .filter(|row| {
            if let Some(index_field) = row.field_at(0) {
                match index_field {
                    Field::Integer(v) => filter.filter(&core::Field::Integer(v)),
                    Field::Text(v) => filter.filter(&core::Field::Text(v.decode().into_owned())),
                    _ => false,
                }
            } else {
                false
            }
        })
        .map(|r| r.field_iter())
        .collect();

    // Prepare output
    let mut output = PTable::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    let row_count = rows.len();
    let column_count = table.column_count();
    if column_count > row_count {
        let mut first = true;
        for col in table.column_iter() {
            let mut out_cells = Vec::with_capacity(row_count + 1);
            out_cells.push(PCell::new(col.name().as_ref()));
            for iter in rows.iter_mut() {
                let field = iter.next().unwrap();
                out_cells.push(field_to_cell(field));
            }
            let prow = PRow::new(out_cells);
            if first {
                output.set_titles(prow);
                first = false;
            } else {
                output.add_row(prow);
            }
        }
    } else {
        let mut title_cells = Vec::with_capacity(column_count);
        for col in table.column_iter() {
            title_cells.push(PCell::new(col.name().as_ref()));
        }
        output.set_titles(PRow::new(title_cells));

        for row_iter in rows {
            let mut out_cells = Vec::with_capacity(column_count);
            for field in row_iter {
                out_cells.push(field_to_cell(field));
            }
            output.add_row(PRow::new(out_cells));
        }
    }

    output.printstd();
    println!("Printed {} row(s)", row_count);

    Ok(())
}
