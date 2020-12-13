use assembly_data::fdb::ro::Handle;
use mapr::Mmap;
use std::{fs::File, path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opt = Options::from_args();
    let start = Instant::now();

    let file = File::open(&opt.file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let handle = Handle::new(&mmap);

    let header = handle.header()?;
    println!("#Tables: {}", header.table_count());
    for table_header in header.table_header_list()? {
        let def_header = table_header.table_def_header()?;
        let _table_name = def_header.table_name()?;
        //println!("{}", table_name.to_str());
        for column_header in def_header.column_header_list()? {
            let _column_name = column_header.column_name()?;
            let _column_data_type = column_header.column_data_type();
            //println!("- {}: {:?}", column_name.to_str(), column_data_type);
        }
        //println!("{:?}", def_header.raw());
        let data_header = table_header.table_data_header()?;
        let mut cnt = 0;
        for bucket_header in data_header.bucket_header_list()? {
            //println!("{:?}", bucket_header);
            for _row in bucket_header.bucket_iter() {
                cnt += 1;
            }
        }
        println!("{}", cnt);
        //println!("{:?}", data_header.raw());
    }

    let duration = start.elapsed();
    println!(
        "Finished in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
