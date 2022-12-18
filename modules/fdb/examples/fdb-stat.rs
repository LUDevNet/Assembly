use argh::FromArgs;
use assembly_fdb::mem::{Database, Tables};
use assembly_fdb_core::value::{Value, ValueType};
use mapr::Mmap;
use std::{fs::File, path::PathBuf, time::Instant};

#[derive(Debug, FromArgs)]
/// Prints statistics on an FDB file
struct Options {
    /// the FDB file
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let opts: Options = argh::from_env();
    let start = Instant::now();

    let file = File::open(opts.file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    println!("Scanning database, this may take a while...");

    let db = Database::new(buffer);
    let tables: Tables<'_> = db.tables()?;

    let table_count = tables.len();
    let mut column_count = 0;
    let mut field_count = 0;
    let mut row_count = 0;
    let mut bucket_count = 0;

    let mut null_field_count = 0;
    let mut int_field_count = 0;
    let mut float_field_count = 0;
    let mut text_field_count = 0;
    let mut bool_field_count = 0;
    let mut bigint_field_count = 0;
    let mut xml_field_count = 0;

    let mut null_column_count = 0;
    let mut int_column_count = 0;
    let mut float_column_count = 0;
    let mut text_column_count = 0;
    let mut bool_column_count = 0;
    let mut bigint_column_count = 0;
    let mut xml_column_count = 0;

    for table in tables.iter() {
        let table = table?;

        for column in table.column_iter() {
            column_count += 1;

            match column.value_type() {
                ValueType::Nothing => null_column_count += 1,
                ValueType::Integer => int_column_count += 1,
                ValueType::Float => float_column_count += 1,
                ValueType::Text => text_column_count += 1,
                ValueType::Boolean => bool_column_count += 1,
                ValueType::BigInt => bigint_column_count += 1,
                ValueType::VarChar => xml_column_count += 1,
            }
        }

        for bucket in table.bucket_iter() {
            bucket_count += 1;

            for row in bucket.row_iter() {
                row_count += 1;

                for field in row.field_iter() {
                    field_count += 1;

                    match field {
                        Value::Nothing => null_field_count += 1,
                        Value::Integer(_) => int_field_count += 1,
                        Value::Float(_) => float_field_count += 1,
                        Value::Text(_) => text_field_count += 1,
                        Value::Boolean(_) => bool_field_count += 1,
                        Value::BigInt(_) => bigint_field_count += 1,
                        Value::VarChar(_) => xml_field_count += 1,
                    }
                }
            }
        }
    }

    let string_count = text_field_count + column_count + table_count;

    println!();
    println!("# General");
    println!("Tables:  {}", table_count);
    println!("Columns: {}", column_count);
    println!("Fields:  {}", field_count);
    println!("Rows:    {}", row_count);
    println!("Buckets: {}", bucket_count);
    println!("Strings: {}", string_count);
    println!();
    println!("# Column Types");
    println!("null:   {}", null_column_count);
    println!("int:    {}", int_column_count);
    println!("float:  {}", float_column_count);
    println!("text:   {}", text_column_count);
    println!("bool:   {}", bool_column_count);
    println!("bigint: {}", bigint_column_count);
    println!("xml:  {}", xml_column_count);
    println!();
    println!("# Field Types");
    println!("null:   {}", null_field_count);
    println!("int:    {}", int_field_count);
    println!("float:  {}", float_field_count);
    println!("text:   {}", text_field_count);
    println!("bool:   {}", bool_field_count);
    println!("bigint: {}", bigint_field_count);
    println!("xml:  {}", xml_field_count);
    println!();

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
