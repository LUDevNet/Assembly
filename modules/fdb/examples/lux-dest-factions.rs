use assembly_fdb::{
    common::Latin1Str,
    mem::{Database, Row, Table, Tables},
};
use color_eyre::eyre::{eyre, WrapErr};
use mapr::Mmap;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    fdb: PathBuf,
}

fn get_table<'a>(tables: Tables<'a>, name: &str) -> color_eyre::Result<Table<'a>> {
    let table = tables
        .by_name(name)
        .ok_or_else(|| eyre!("Missing table '{}'", name))??;
    Ok(table)
}

fn get_column_index(table: Table<'_>, name: &str) -> color_eyre::Result<usize> {
    let index = table
        .column_iter()
        .position(|col| col.name() == name)
        .ok_or_else(|| eyre!("Missing columns '{}'.'{}'", table.name(), name))?;
    Ok(index)
}
struct DestructibleComponent<'a> {
    inner: Row<'a>,
}

impl<'a> DestructibleComponent<'a> {
    fn faction_list(&self) -> Option<&'a Latin1Str> {
        self.inner.field_at(2).unwrap().into_opt_text()
    }

    fn faction(&self) -> Option<i32> {
        self.inner.field_at(1).unwrap().into_opt_integer()
    }

    fn id(&self) -> i32 {
        self.inner.field_at(0).unwrap().into_opt_integer().unwrap()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    // Load the database file
    let file = File::open(&opts.fdb)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.fdb.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let tables = db.tables()?;

    let destructible_component_table = get_table(tables, "DestructibleComponent")?;
    assert_eq!(
        1,
        get_column_index(destructible_component_table, "faction")?
    );
    assert_eq!(
        2,
        get_column_index(destructible_component_table, "factionList")?
    );

    let mut map: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();

    for inner in destructible_component_table.row_iter() {
        let component = DestructibleComponent { inner };

        let id = component.id();
        if let Some(faction) = component.faction() {
            if faction > -1 {
                map.entry(faction).or_default().insert(id);
            }
        }
        if let Some(faction_list) = component.faction_list() {
            for elem in faction_list.decode().split(',') {
                let faction: i32 = elem.parse()?;
                if faction > -1 {
                    map.entry(faction).or_default().insert(id);
                }
            }
        }
    }

    let string = serde_json::to_string(&map)?;
    println!("{}", string);

    Ok(())
}
