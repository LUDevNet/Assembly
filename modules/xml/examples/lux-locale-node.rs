use std::path::PathBuf;

use assembly_xml::localization::load_locale;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    path: PathBuf,
    #[structopt(default_value = "")]
    prefix: String,
}

fn main() -> color_eyre::Result<()> {
    let opt = Options::from_args();

    let locale_node = load_locale(&opt.path)?;
    let mut node_ref = &locale_node;

    if !opt.prefix.is_empty() {
        for part in opt.prefix.split('_') {
            node_ref = match part.parse() {
                Ok(i) => node_ref.int_children.get(&i),
                Err(_) => node_ref.str_children.get(part),
            }
            .unwrap();
        }
    }

    let string: String = serde_json::to_string(&node_ref.get_keys())?;
    println!("{}", string);

    Ok(())
}
