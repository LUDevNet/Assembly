use assembly_fdb::{common::Latin1String, common::ValueType, core::Field, mem::Database, store};
use color_eyre::eyre::{self, eyre, WrapErr};
use mapr::Mmap;
use rusqlite::{types::ValueRef, Connection};
use std::{fmt::Write, fs::File, io::BufWriter, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "sqlite-to-fdb")]
#[structopt(author = "zaop")]
/// Convert an SQLite database to FDB. By default, type information from the SQLite DB is used; if unavailable, you can specify the target
/// datatypes through a 'template' FDB file with `--template` (see template-fdb.rs).
struct Options {
    /// Input SQLite file
    src: PathBuf,
    /// Output FDB file
    dest: PathBuf,
    /// Optional: an FDB file containing tables with correct columns but no rows used to determine type information
    #[structopt(long)]
    template: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let start = Instant::now();

    // sqlite input
    if !opts.src.exists() {
        return Err(eyre!("SQLite file '{}' not found", opts.src.display()));
    }
    let conn = Connection::open_with_flags(&opts.src, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .wrap_err_with(|| format!("Failed to open SQLite file '{}'", opts.src.display()))?;

    // this .unwrap() won't error because we just checked that the file exists and therefore it has a name
    let src_stem = opts.src.file_stem().unwrap();
    let mut dest_path = opts.dest.clone();

    // check if destination path is a directory
    if dest_path.is_dir() {
        // save to source file name but with .fdb extension
        dest_path = dest_path.join(src_stem).with_extension("fdb");
        if dest_path.exists() {
            return Err(eyre!(
                "A directory was specified as output path, and in this directory the default filename '{}.fdb' is already in use.",
                src_stem.to_str().unwrap_or("<invalid UTF-8>")
            ));
        }
    }

    // fdb output
    let dest_file = File::create(&dest_path)
        .wrap_err_with(|| format!("Failed to create output file '{}'", opts.dest.display()))?;

    let mut dest_out = BufWriter::new(dest_file);
    let mut dest_db = store::Database::new();

    let result;

    if let Some(template) = &opts.template {
        // fdb template
        let template_file = File::open(template)
            .wrap_err_with(|| format!("Failed to open fdb template '{}'", template.display()))?;
        let mmap = unsafe { Mmap::map(&template_file)? };
        let buffer: &[u8] = &mmap;
        let template_db = Database::new(buffer);

        result = convert_with_template(&conn, &mut dest_db, &template_db);
    } else {
        result = convert_without_template(&conn, &mut dest_db);
    }

    match result {
        Ok(()) => {
            dest_db
                .write(&mut dest_out)
                .wrap_err("Could not write output database")?;

            let duration = start.elapsed();
            println!(
                "Finished in {}.{:#03}s",
                duration.as_secs(),
                duration.subsec_millis()
            );

            println!("Output written to '{}'", &dest_path.display());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn convert_without_template(
    conn: &Connection,
    dest_db: &mut assembly_fdb::store::Database,
) -> eyre::Result<()> {
    println!("Using direct SQLite -> FDB conversion.");
    println!("Converting database, this may take a few seconds...");

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
            let decl_type = column
                .decl_type()
                .expect("The SQLite database is missing column type information. Try converting using a template (see sqlite-to-fdb --help).");

            let target_type = ValueType::from_sqlite_type(decl_type)
                .expect("The SQLite database contains an unknown column type. Try converting using a template (see sqlite-to-fdb --help).");

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

    Ok(())
}

fn convert_with_template(
    conn: &Connection,
    dest_db: &mut assembly_fdb::store::Database,
    template_db: &Database,
) -> eyre::Result<()> {
    println!("Using template FDB for conversion.");
    println!("Converting database, this may take a few seconds...");

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
            #[allow(clippy::needless_range_loop)]
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
                Field::Text(t) => (sfhash::digest(t.as_bytes())) as usize,
                _ => return Err(eyre!("Cannot use {:?} as primary key", &fields[0])),
            };

            dest_table.push_row(pk, &fields);
        }

        dest_db.push_table(template_table.name_raw(), dest_table);
    }

    Ok(())
}
