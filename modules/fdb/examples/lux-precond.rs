use assembly_fdb::{
    common::Latin1Str,
    mem::{Database, Row, Table, Tables},
};
use color_eyre::eyre::{eyre, WrapErr};
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

#[allow(non_snake_case)]
#[derive(Serialize, Default, Clone, Debug)]
pub struct Precondition {
    pub id: i32,
    pub r#type: Option<i32>,
    pub targetLOT: Option<String>,
    pub targetGroup: Option<String>,
    pub targetCount: Option<i32>,
    pub iconID: Option<i32>,
    pub localize: bool,
    pub validContexts: i64,
    pub locStatus: i32,
    pub gate_version: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PreconditionLoader {
    ci_id: usize,
    ci_type: usize,
    ci_targetLOT: usize,
    ci_targetGroup: usize,
    ci_targetCount: usize,
    ci_iconID: usize,
    ci_localize: usize,
    ci_validContexts: usize,
    ci_locStatus: usize,
    ci_gate_version: usize,
}

impl PreconditionLoader {
    pub fn from_table(table: Table<'_>) -> Self {
        let mut res = Self::default();
        for (ci, col) in table.column_iter().enumerate() {
            let name = col.name();
            match name.as_ref() {
                "id" => res.ci_id = ci,
                "type" => res.ci_type = ci,
                "targetLOT" => res.ci_targetLOT = ci,
                "targetGroup" => res.ci_targetGroup = ci,
                "targetCount" => res.ci_targetCount = ci,
                "iconID" => res.ci_iconID = ci,
                "localize" => res.ci_localize = ci,
                "validContexts" => res.ci_validContexts = ci,
                "locStatus" => res.ci_locStatus = ci,
                "gate_version" => res.ci_gate_version = ci,
                _ => {}
            }
        }
        res
    }

    #[rustfmt::skip]
    pub fn load(&self, row: Row) -> Precondition {
        Precondition {
            id: row.field_at(self.ci_id).unwrap().into_opt_integer().unwrap(),
            r#type: row.field_at(self.ci_type).unwrap().into_opt_integer(),
            targetLOT: row.field_at(self.ci_targetLOT).unwrap().into_opt_text().map(decode_to_owned),
            targetGroup: row.field_at(self.ci_targetGroup).unwrap().into_opt_text().map(decode_to_owned),
            targetCount: row.field_at(self.ci_targetCount).unwrap().into_opt_integer(),
            iconID: row.field_at(self.ci_iconID).unwrap().into_opt_integer(),
            localize: row.field_at(self.ci_localize).unwrap().into_opt_boolean().unwrap(),
            validContexts: row.field_at(self.ci_validContexts).unwrap().into_opt_big_int().unwrap(),
            locStatus: row.field_at(self.ci_locStatus).unwrap().into_opt_integer().unwrap(),
            gate_version: row.field_at(self.ci_gate_version).unwrap().into_opt_text().map(decode_to_owned),
        }
    }
}

fn decode_to_owned(input: &Latin1Str) -> String {
    input.decode().into_owned()
}

impl Default for PreconditionLoader {
    fn default() -> Self {
        Self {
            ci_id: 0,
            ci_type: 1,
            ci_targetLOT: 2,
            ci_targetGroup: 3,
            ci_targetCount: 4,
            ci_iconID: 5,
            ci_localize: 6,
            ci_validContexts: 7,
            ci_locStatus: 8,
            ci_gate_version: 9,
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

    let preconditions = get_table(tables, "Preconditions")?;
    let prec_loader = PreconditionLoader::from_table(preconditions);

    if let Some(out) = &opts.out {
        std::fs::create_dir(out)?;
    }

    for row in preconditions.row_iter() {
        let prec = prec_loader.load(row);
        let string = serde_json::to_string(&prec)?;
        if let Some(out) = &opts.out {
            let path = out.join(&format!("{}.json", prec.id));
            std::fs::write(path, string)?;
        } else {
            println!("{}", string);
        }
    }

    Ok(())
}
