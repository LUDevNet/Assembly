use assembly_data::fdb::{
    common::ValueType,
    query::pk_filter,
    reader::{builder::DatabaseBuilder, DatabaseBufReader, DatabaseReader},
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

fn run(filename: &Path, tablename: &str, key: &str) -> color_eyre::Result<()> {
    //println!("Loading tables... (this may take a while)");
    let file = File::open(filename)?;
    let mut loader = BufReader::new(file);

    let h = loader.get_header()?;
    let thl = loader.get_table_header_list(h)?;

    let thlv: Vec<_> = thl.into();
    let mut iter = thlv.iter();

    let (_tn, tdh, tth) = loop {
        match iter.next() {
            Some(th) => {
                let tdh = loader.get_table_def_header(th.table_def_header_addr)?;
                let name = loader.get_string(tdh.table_name_addr)?;

                if name == tablename {
                    let tth = loader.get_table_data_header(th.table_data_header_addr)?;
                    break Ok((name, tdh, tth));
                }
            }
            None => break Err(eyre!("Table not found")),
        }
    }?;

    let chl = loader.get_column_header_list(&tdh)?;
    let chlv: Vec<_> = chl.into();
    let mut cnl = Vec::with_capacity(tdh.column_count as usize);
    for ch in chlv.iter() {
        let cn = loader.get_string(ch.column_name_addr)?;
        cnl.push(PCell::new(&cn));
    }

    let value_type = ValueType::try_from(chlv[0].column_data_type).unwrap();
    let filter = pk_filter(key, value_type)?;

    let bc = tth.buckets.count;

    let bhlv: Vec<_> = loader.get_bucket_header_list(&tth)?.into();
    let hash = filter.hash();

    let bh = bhlv[(hash % bc) as usize];
    let rhlha = bh.row_header_list_head_addr;

    let rhi = loader.get_row_header_addr_iterator(rhlha);

    let mut count = 0;
    let mut output = PTable::new();
    output.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    output.set_titles(PRow::new(cnl));

    for orha in rhi.collect::<Vec<_>>() {
        let rha = orha?;
        let rh = loader.get_row_header(rha)?;

        let fdlv: Vec<_> = loader.get_field_data_list(rh)?.into();

        let ff = loader.try_load_field(&fdlv[0])?;
        if filter.filter(&ff) {
            let mut fv = Vec::with_capacity(fdlv.len());
            fv.push(PCell::new(&ff.to_string()));

            for fd in &fdlv[1..] {
                let f = loader.try_load_field(&fd)?;
                fv.push(PCell::new(&f.to_string()));
            }
            count += 1;
            output.add_row(PRow::new(fv));
        }
    }

    output.printstd();
    println!("Printed {} row(s)", count);

    Ok(())
}

#[derive(StructOpt)]
struct Options {
    file: PathBuf,
    table: String,
    key: String,
}

pub fn main() -> color_eyre::Result<()> {
    let opts = Options::from_args();
    run(&opts.file, &opts.table, &opts.key)
}
