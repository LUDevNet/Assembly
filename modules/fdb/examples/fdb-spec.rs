use argh::FromArgs;
use assembly_fdb::mem::{Database, Tables};
use assembly_fdb_core::value::mem;
use assembly_fdb_core::value::ValueType;
use latin1str::Latin1Str;
use mapr::Mmap;
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    iter::FromIterator,
    path::PathBuf,
};

#[derive(Serialize)]
pub struct Spec<'a> {
    tables: BTreeMap<&'a Latin1Str, TableSpec<'a>>,
}

#[derive(Serialize)]
pub struct TableSpec<'a> {
    columns: Vec<ColumnSpec<'a>>,
}

#[derive(Serialize)]
pub struct ColumnSpec<'a> {
    name: &'a Latin1Str,
    ty: ValueType,
    nullable: bool,
}

#[derive(Debug, FromArgs)]
/// Prints the names of all tables and their columns
struct Options {
    /// the FDB file
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts: Options = argh::from_env();

    let file = File::open(opts.file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    let db = Database::new(buffer);
    let tables: Tables<'_> = db.tables()?;

    let mut spec = Spec {
        tables: BTreeMap::new(),
    };
    for table in tables.iter() {
        let table = table?;
        let table_name = table.name_raw();

        let mut columns = Vec::new();
        for column in table.column_iter() {
            let name = column.name_raw();
            columns.push(ColumnSpec {
                name,
                ty: column.value_type(),
                nullable: false,
            });
        }

        let mut test_set: HashSet<usize> = HashSet::from_iter(0..columns.len());
        let mut checked = HashSet::new();

        for row in table.row_iter() {
            for index in test_set.clone() {
                if let Some(f) = row.field_at(index) {
                    if f == mem::Field::Nothing {
                        columns[index].nullable = true;
                        checked.insert(index);
                    }
                }
            }
            for index in checked.drain() {
                test_set.remove(&index);
            }
            if test_set.is_empty() {
                break;
            }
        }

        spec.tables.insert(table_name, TableSpec { columns });
    }

    println!("{}", serde_json::to_string_pretty(&spec)?);

    Ok(())
}
