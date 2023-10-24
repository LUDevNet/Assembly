use assembly_fdb::{
    mem::{Database, Table},
    value::Value,
};
use base64::{engine::general_purpose::STANDARD, read::DecoderReader};
use color_eyre::eyre::{eyre, Context};
use mapr::Mmap;
use std::{fs::File, io::Cursor};
use std::{io::BufWriter, path::PathBuf};

#[derive(argh::FromArgs)]
/// parse a sysdiagram from a FDB file
struct Options {
    /// path to the FDB file
    #[argh(positional)]
    file: PathBuf,

    #[argh(positional, default = "PathBuf::from(\"out.sysdiagram\")")]
    output: PathBuf,
}

fn load_database(opts: &Options) -> color_eyre::Result<()> {
    // Load the database file
    let file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let table = db
        .tables()?
        .by_name("sysdiagrams")
        .ok_or_else(|| eyre!("Failed to find table sysdiagrams"))?;
    let table: Table = table.context("Failed to load table 'sysdiagrams'")?;

    if let Some(row) = table.row_iter().next() {
        match row.field_at(4) {
            Some(Value::Text(text)) => {
                let mut wrapped_reader = Cursor::new(text.as_bytes());
                let mut decoder = DecoderReader::new(&mut wrapped_reader, &STANDARD);
                let file = File::create(&opts.output)?;
                let mut writer = BufWriter::new(file);
                print!("Writing {} ...", opts.output.display());
                std::io::copy(&mut decoder, &mut writer)?;
                println!("Done!");
            }
            data => println!("Wrong data: {:?}", data),
        }
    } else {
        eprintln!("No sysdiagram found in table");
    }
    Ok(())
}

pub fn main() -> color_eyre::Result<()> {
    let opts: Options = argh::from_env();
    load_database(&opts).with_context(|| "Loading database failed!")
}
