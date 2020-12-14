use assembly_data::fdb::{
    common::ValueType,
    reader::{DatabaseBufReader, DatabaseReader},
};
use color_eyre::eyre::eyre;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::{
    convert::TryFrom,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

fn run(filename: &Path, tablename: &str) -> color_eyre::Result<()> {
    //println!("Loading tables... (this may take a while)");
    let file = File::open(filename)?;
    let mut loader = BufReader::new(file);

    let h = loader.get_header()?;
    let thl = loader.get_table_header_list(h)?;

    let thlv: Vec<_> = thl.into();
    let mut iter = thlv.iter();

    let (_tn, tdh, _tth) = loop {
        match iter.next() {
            Some(th) => {
                let tdh = loader.get_table_def_header(th.table_def_header_addr)?;
                let name = loader.get_string(tdh.table_name_addr)?;

                if name == tablename {
                    let tth = loader.get_table_data_header(th.table_data_header_addr)?;
                    break Ok((name, tdh, tth));
                }
            }
            None => break Err(eyre!("Table not found!")),
        }
    }?;

    let mut count = 0;
    let mut output = PTable::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    output.set_titles(PRow::new(vec![PCell::new("Name"), PCell::new("Type")]));

    let chl = loader.get_column_header_list(&tdh)?;
    let chlv: Vec<_> = chl.into();

    for ch in chlv.iter() {
        let cn = loader.get_string(ch.column_name_addr)?;
        let vt = ValueType::try_from(ch.column_data_type).unwrap();

        let cr = PRow::new(vec![PCell::new(&cn), PCell::new(&vt.to_string())]);

        output.add_row(cr);
        count += 1;
    }

    output.printstd();
    println!("Printed {} row(s)", count);

    Ok(())
}

#[derive(StructOpt)]
struct Options {
    file: PathBuf,
    table: String,
}

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();
    run(&opts.file, &opts.table)
}
