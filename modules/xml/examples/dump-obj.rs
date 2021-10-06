use std::{io::Cursor, path::PathBuf};

use argh::FromArgs;
use assembly_xml::obj::Object;

#[derive(FromArgs)]
/// debug an LU character XML (savegame)
struct Args {
    /// a filename
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Args = argh::from_env();

    println!("{}", args.file.display());
    let xml = std::fs::read_to_string(&args.file)?;
    let reader = Cursor::new(xml);
    let mut de = quick_xml::de::Deserializer::from_reader(reader);

    let data: Object = serde_path_to_error::deserialize(&mut de)?;
    println!("{:#?}", data);

    Ok(())
}
