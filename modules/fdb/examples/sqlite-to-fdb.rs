use assembly_fdb::{common::ValueType, core::Field, mem::Database, store};
use color_eyre::eyre::{self, eyre, WrapErr};
use mapr::Mmap;
use rusqlite::{types::ValueRef, Connection};
use std::{fmt::Write, fs::File, io::BufWriter, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Fills a template FDB file (see template-fdb.rs) with rows from a sqlite database
struct Options {
    /// "Template" FDB file, the table structure from this will be used for the output
    template: PathBuf,
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

    // fdb template
    let template_file = File::open(&opts.template)
        .wrap_err_with(|| format!("Failed to open fdb template '{}'", opts.template.display()))?;
    let mmap = unsafe { Mmap::map(&template_file)? };
    let buffer: &[u8] = &mmap;
    let template_db = Database::new(buffer);

    // sqlite input
    let conn = Connection::open_with_flags(&opts.src, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .wrap_err_with(|| format!("Failed to open sqlite file '{}'", opts.src.display()))?;

    // fdb output
    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);
    let mut dest_db = store::Database::new();

    for template_table in template_db.tables()?.iter() {
        let template_table = template_table?;

        // Find number of unique values in first column of source table
        let unique_key_count = conn.query_row::<u32, _, _>(
            &format!(
                "select count(distinct [{}]) as unique_count from {}",
                template_table
                    .column_at(0)
                    .ok_or_else(|| eyre!(format!(
                        "FDB template contains no columns for table {}",
                        template_table.name()
                    )))?
                    .name(),
                template_table.name()
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

        // Number of columns destination table should have
        let column_count = template_table.column_count();

        // Vector to store target datatypes in as these can't be determined from the sqlite source db
        let mut target_types: Vec<ValueType> = Vec::with_capacity(column_count);

        // Build select query with the same column names and order as the template FDB
        let mut select_query = String::from("select ");

        // Write select query and store target datatypes
        for (index, template_column) in template_table.column_iter().enumerate() {
            let template_column_name = template_column.name();

            write!(select_query, "[{}]", template_column_name)?;

            if index < column_count - 1 {
                write!(select_query, ", ")?;
            }

            target_types.push(template_column.value_type());

            dest_table.push_column(template_column.name_raw(), template_column.value_type());
        }

        select_query.push_str(" from ");
        select_query.push_str(&template_table.name());

        // Execute query
        let mut statement = conn.prepare(&select_query)?;
        let mut rows = statement.query(rusqlite::NO_PARAMS)?;

        // Iterate over rows
        while let Some(sqlite_row) = rows.next()? {
            let mut fields: Vec<Field> = Vec::with_capacity(column_count);

            // Iterate over fields
            for index in 0..column_count {
                fields.push(match sqlite_row.get_raw(index) {
                    ValueRef::Null => Field::Nothing,
                    ValueRef::Integer(i) => match target_types[index] {
                        ValueType::Integer => Field::Integer(i as i32),
                        ValueType::Boolean => Field::Boolean(i == 1),
                        ValueType::BigInt => Field::BigInt(i),
                        _ => {
                            return Err(eyre!(
                                "Invalid target datatype; cannot store SQLite Integer as FDB {:?}",
                                target_types[index]
                            ))
                        }
                    },
                    ValueRef::Real(f) => Field::Float(f as f32),
                    ValueRef::Text(t) => match target_types[index] {
                        ValueType::Text => Field::Text(String::from(std::str::from_utf8(t)?)),
                        ValueType::VarChar => Field::VarChar(String::from(std::str::from_utf8(t)?)),
                        _ => {
                            return Err(eyre!(
                                "Invalid target datatype; cannot store SQLite Text as FDB {:?}",
                                target_types[index]
                            ))
                        }
                    },
                    ValueRef::Blob(_b) => {
                        return Err(eyre!("SQLite Blob datatype cannot be converted"))
                    }
                });
            }

            // Determine primary key to use for bucket index
            let pk = match &fields[0] {
                Field::Integer(i) => *i as usize,
                Field::BigInt(i) => *i as usize,
                Field::Text(t) => (hsieh_hash::digest(t.as_bytes())) as usize,
                _ => return Err(eyre!("Cannot use {:?} as primary key", &fields[0])),
            };

            dest_table.push_row(pk, &fields);
        }

        dest_db.push_table(template_table.name_raw(), dest_table);
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
