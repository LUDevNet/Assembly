use assembly_fdb::mem::{Database, Row, Table, Tables};
use color_eyre::eyre::{eyre, WrapErr};
use latin1str::Latin1Str;
use mapr::Mmap;
use serde::Serialize;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    fdb: PathBuf,
    #[structopt(long)]
    out: Option<PathBuf>,
}

fn get_table<'a>(tables: Tables<'a>, name: &str) -> color_eyre::Result<Table<'a>> {
    let table = tables
        .by_name(name)
        .ok_or_else(|| eyre!("Missing table '{}'", name))??;
    Ok(table)
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[derive(Serialize, Default, Clone, Debug)]
struct HAL<T> {
    _embedded: T,
}

#[allow(non_snake_case)]
#[derive(Serialize, Default, Clone, Debug)]
struct MapShaders {
    mapShaders: Vec<Shader>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Default, Clone, Debug)]
pub struct Shader {
    pub id: i32,
    pub label: Option<String>,
    pub gameValue: i32,
    pub priority: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ShaderLoader {
    ci_id: usize,
    ci_label: usize,
    ci_gameValue: usize,
    ci_priority: usize,
}

impl ShaderLoader {
    pub fn from_table(table: Table<'_>) -> Self {
        let mut res = Self::default();
        for (ci, col) in table.column_iter().enumerate() {
            let name = col.name();
            match name.as_ref() {
                "id" => res.ci_id = ci,
                "label" => res.ci_label = ci,
                "gameValue" => res.ci_gameValue = ci,
                "priority" => res.ci_priority = ci,
                _ => {}
            }
        }
        res
    }

    #[rustfmt::skip]
    pub fn load(&self, row: Row) -> Shader {
        Shader {
            id: row.field_at(self.ci_id).unwrap().into_opt_integer().unwrap(),
            label: row.field_at(self.ci_label).unwrap().into_opt_text().map(decode_to_owned),
            gameValue: row.field_at(self.ci_gameValue).unwrap().into_opt_integer().unwrap(),
            priority: row.field_at(self.ci_priority).unwrap().into_opt_integer(),
        }
    }
}

fn decode_to_owned(input: &Latin1Str) -> String {
    input.decode().into_owned()
}

impl Default for ShaderLoader {
    fn default() -> Self {
        Self {
            ci_id: 0,
            ci_label: 1,
            ci_gameValue: 2,
            ci_priority: 3,
        }
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

    let shaders = get_table(tables, "mapShaders")?;
    let shad_loader = ShaderLoader::from_table(shaders);

    if let Some(out) = &opts.out {
        std::fs::create_dir(out)?;
    }

    let all: Vec<_> = shaders
        .row_iter()
        .map(|row| shad_loader.load(row))
        .collect();
    let hal = HAL {
        _embedded: MapShaders { mapShaders: all },
    };

    let string = serde_json::to_string(&hal)?;
    if let Some(out) = &opts.out {
        let path = out.join("index.json");
        std::fs::write(path, string)?;
    } else {
        println!("{}", string);
    }

    Ok(())
}
