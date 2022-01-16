use assembly_fdb::{common::Latin1String, common::ValueType, core::Field, store};
use color_eyre::eyre::{self, eyre, WrapErr};
use rusqlite::{types::ValueRef, Connection};
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Fills a template FDB file (see template-fdb.rs) with rows from a sqlite database
struct Options {
    /// Input sqlite database
    src: PathBuf,
    /// Output FDB file
    dest: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();
    let start = Instant::now();

    println!("Converting database, this may take a few seconds...");

    // sqlite input
    let conn = Connection::open_with_flags(&opts.src, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .wrap_err_with(|| format!("Failed to open sqlite file '{}'", opts.src.display()))?;

    // fdb output
    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);
    let mut dest_db = store::Database::new();

    let tables_query = String::from("select name from sqlite_master where type='table'");
    let mut statement = conn.prepare(&tables_query)?;
    let table_names = statement.query_map::<String, _, _>(rusqlite::NO_PARAMS, |row| row.get(0))?;

    for table_name in table_names {
        let table_name = table_name?;

        // Query used for getting column info and the actual data
        let select_query = format!("select * from {}", &table_name);

        // Prepare statement
        let mut statement = conn.prepare(&select_query)?;

        // Number of columns destination table should have
        let column_count = statement.columns().len();

        // Vector to store target datatypes in
        let mut target_types: Vec<ValueType> = Vec::with_capacity(column_count);

        // Get column types
        for column in &statement.columns() {
            let decl_type = column.decl_type().expect("Failed to get column type");

            let target_type =
                ValueType::from_sqlite_type(decl_type).expect("Failed to convert column type");

            target_types.push(target_type);
        }

        // Find number of unique values in first column of source table
        let unique_key_count = conn.query_row::<u32, _, _>(
            &format!(
                "select count(distinct [{}]) as unique_count from {}",
                statement.columns().get(0).unwrap().name(),
                table_name
            ),
            rusqlite::NO_PARAMS,
            |row| row.get(0),
        )?;

        // Bucket count should be 0 or a power of two
        let new_bucket_count = if unique_key_count == 0 {
            0
        } else {
            u32::next_power_of_two(unique_key_count)
        };

        // Create destination table
        let mut dest_table = store::Table::new(new_bucket_count as usize);

        // Add columns to destination table
        for (i, column) in statement.columns().iter().enumerate() {
            dest_table.push_column(
                Latin1String::encode(column.name()),
                *target_types.get(i).unwrap(),
            );
        }

        // Execute query
        let mut rows = statement.query(rusqlite::NO_PARAMS)?;

        // Iterate over rows
        while let Some(sqlite_row) = rows.next()? {
            let mut fields: Vec<Field> = Vec::with_capacity(column_count);

            // Iterate over fields
            for index in 0..column_count {
                let value = sqlite_row.get_raw(index);
                fields.push(match value {
                    ValueRef::Null => Field::Nothing,
                    _ => match target_types[index] {
                        ValueType::Nothing => Field::Nothing,
                        ValueType::Integer => Field::Integer(value.as_i64().unwrap() as i32),
                        ValueType::Float => Field::Float(value.as_f64().unwrap() as f32),
                        ValueType::Text => Field::Text(String::from(value.as_str().unwrap())),
                        ValueType::Boolean => Field::Boolean(value.as_i64().unwrap() != 0),
                        ValueType::BigInt => Field::BigInt(value.as_i64().unwrap()),
                        ValueType::VarChar => {
                            Field::VarChar(String::from(value.as_str().unwrap() as &str))
                        }
                    },
                });
            }

            // Determine primary key to use for bucket index
            let pk = match &fields[0] {
                Field::Integer(i) => *i as usize,
                Field::BigInt(i) => *i as usize,
                Field::Text(t) => (sfhash::digest(t.as_bytes())) as usize,
                _ => return Err(eyre!("Cannot use {:?} as primary key", &fields[0])),
            };

            dest_table.push_row(pk, &fields);
        }

        dest_db.push_table(Latin1String::encode(&table_name), dest_table);
    }

    dest_db
        .write(&mut dest_out)
        .wrap_err("Could not write output database")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{:#03}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
