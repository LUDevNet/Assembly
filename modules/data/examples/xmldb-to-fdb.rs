use assembly_data::{
    fdb::{
        common::{self, Latin1String},
        core, store,
    },
    xml::{
        common::{expect_decl, expect_end},
        database::{
            expect_column_or_end_columns, expect_columns, expect_database, expect_row_or_end_rows,
            expect_rows, expect_table, ValueType,
        },
        quick::Reader,
    },
};
use assembly_fdb::common::str_hash;
use color_eyre::eyre::WrapErr;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::Instant,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Converts an XML database into a FDB file
struct Options {
    /// The XML database file
    src: PathBuf,
    /// The FDB file
    dest: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    let src_file = File::open(&opts.src)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.src.display()))?;
    let reader = BufReader::new(src_file);

    let dest_file = File::create(&opts.dest)
        .wrap_err_with(|| format!("Failed to crate output file '{}'", opts.dest.display()))?;
    let mut dest_out = BufWriter::new(dest_file);

    println!("Copying file, this may take a few seconds...");
    let start = Instant::now();

    let mut dest_db = store::Database::new();

    let mut xml = Reader::from_reader(reader);
    let xml = xml.trim_text(true);

    let mut buf = Vec::new();
    let buf = &mut buf;

    expect_decl(xml, buf)?;
    let db_name = expect_database(xml, buf)?.unwrap();
    println!("Loading database: '{}'", db_name);

    while let Some(table_name) = expect_table(xml, buf)? {
        println!("table '{}'", table_name);
        let mut dest_table = store::Table::new(128);

        expect_columns(xml, buf)?;

        let mut col_map = HashMap::new();

        while let Some(col) = expect_column_or_end_columns(xml, buf)? {
            let data_type = match col.r#type {
                ValueType::Bit => common::ValueType::Boolean,
                ValueType::Float => common::ValueType::Float,
                ValueType::Real => common::ValueType::Float,
                ValueType::Int => common::ValueType::Integer,
                ValueType::BigInt => common::ValueType::BigInt,
                ValueType::SmallInt => common::ValueType::Integer,
                ValueType::TinyInt => common::ValueType::Integer,
                ValueType::Binary => common::ValueType::Text,
                ValueType::VarBinary => common::ValueType::Text,
                ValueType::Char => common::ValueType::Text,
                ValueType::VarChar => common::ValueType::Text,
                ValueType::NChar => common::ValueType::Text,
                ValueType::NVarChar => common::ValueType::Text,
                ValueType::NText => common::ValueType::VarChar,
                ValueType::Text => common::ValueType::VarChar,
                ValueType::Image => common::ValueType::VarChar,
                ValueType::DateTime => common::ValueType::BigInt,
                ValueType::Xml => common::ValueType::VarChar,
                ValueType::Null => common::ValueType::Nothing,
                ValueType::SmallDateTime => common::ValueType::Integer,
            };
            if col_map.is_empty() {
                // first col
                if data_type == common::ValueType::Float {
                    let id_col_name = format!("{}ID", table_name);
                    dest_table.push_column(
                        Latin1String::encode(&id_col_name),
                        common::ValueType::Integer,
                    );
                    col_map.insert(id_col_name, col_map.len());
                }
            }

            dest_table.push_column(Latin1String::encode(&col.name), data_type);
            col_map.insert(col.name, col_map.len());
        }

        expect_rows(xml, buf)?;

        let col_count = dest_table.columns().len();
        let mut auto_inc = 0;
        while let Some(row) = expect_row_or_end_rows(xml, buf, true)? {
            let mut fields = vec![core::Field::Nothing; col_count];
            let mut pk = None;
            for (key, src_value) in row {
                let col_index = *col_map.get(&key).unwrap();
                let value_type = dest_table.columns().get(col_index).unwrap().value_type();
                let dest_value = match value_type {
                    common::ValueType::Nothing => core::Field::Nothing,
                    common::ValueType::Integer => core::Field::Integer(src_value.parse().unwrap()),
                    common::ValueType::Float => core::Field::Float(src_value.parse().unwrap()),
                    common::ValueType::Text => core::Field::Text(src_value),
                    common::ValueType::Boolean => core::Field::Boolean(&src_value != "0"),
                    common::ValueType::BigInt => core::Field::BigInt(src_value.parse().unwrap()),
                    common::ValueType::VarChar => core::Field::VarChar(src_value),
                };

                if col_index == 0 {
                    match &dest_value {
                        core::Field::Integer(i) => {
                            pk = Some((*i % 128) as usize);
                        }
                        core::Field::BigInt(i) => {
                            pk = Some((*i % 128) as usize);
                        }
                        core::Field::Text(text) => {
                            let lat1 = Latin1String::encode(text);
                            pk = Some((str_hash(&lat1) % 128) as usize);
                        }
                        core::Field::VarChar(var_char) => {
                            let lat1 = Latin1String::encode(var_char);
                            pk = Some((str_hash(&lat1) % 128) as usize);
                        }
                        _ => panic!("Can't use {:?} as PK", &dest_value),
                    }
                }

                fields[col_index] = dest_value;
            }
            let pk = if let Some(pk) = pk {
                pk
            } else {
                auto_inc += 1;
                fields[0] = core::Field::Integer(auto_inc);
                (auto_inc as usize) % 128
            };
            dest_table.push_row(pk, &fields);
        }

        expect_end(xml, buf, "table")?;
        dest_db.push_table(Latin1String::encode(&table_name), dest_table);
    }

    dest_db
        .write(&mut dest_out)
        .wrap_err("Failed to write copied database")?;

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );

    Ok(())
}
