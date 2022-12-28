use std::path::PathBuf;

use assembly_xml::localization::{load_locale, Key};
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
                Err(_) => match Key::from_str(part) {
                    Ok(key) => node_ref.str_children.get(&key),
                    Err(_) => None,
                },
            }
            .unwrap();
        }
    }

    let string: String = serde_json::to_string(&node_ref.get_keys())?;
    println!("{}", string);

    Ok(())
}
