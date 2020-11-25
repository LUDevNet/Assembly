extern crate getopts;
use anyhow::anyhow;
use assembly_data::fdb::{
    builder::DatabaseBuilder,
    core::ValueType,
    query::pk_filter,
    reader::{DatabaseBufReader, DatabaseReader},
};
use getopts::Options;
use prettytable::{Cell as PCell, Row as PRow, Table as PTable};
use std::{env, fs::File, io::BufReader};

fn run(filename: &str, tablename: &str, key: &str) -> Result<(), anyhow::Error> {
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
            None => break Err(anyhow!("Table not found")),
        }
    }?;

    let chl = loader.get_column_header_list(&tdh)?;
    let chlv: Vec<_> = chl.into();
    let mut cnl = Vec::with_capacity(tdh.column_count as usize);
    for ch in chlv.iter() {
        let cn = loader.get_string(ch.column_name_addr)?;
        cnl.push(PCell::new(&cn));
    }

    let value_type = ValueType::from(chlv[0].column_data_type);
    let filter = pk_filter(key, value_type)?;

    let bc = tth.bucket_count;

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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE TABLE KEY [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    let (file, table, key) = if matches.free.len() >= 3 {
        (&matches.free[0], &matches.free[1], &matches.free[2])
    } else {
        print_usage(&program, opts);
        return Ok(());
    };
    run(file, table, key)
}
