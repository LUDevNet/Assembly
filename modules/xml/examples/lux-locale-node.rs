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

    let locale_root = load_locale(&opt.path)?;
    let mut node_ref = locale_root.as_ref();
    let strs = locale_root.strs();

    if !opt.prefix.is_empty() {
        for part in opt.prefix.split('_') {
            node_ref = match part.parse() {
                Ok(i) => node_ref.get_int(i),
                Err(_) => match strs.get(part) {
                    Some(key) => node_ref.get_str(key),
                    None => None,
                },
            }
            .unwrap();
        }
    }

    let string: String = serde_json::to_string(&node_ref.node().get_keys(strs))?;
    println!("{}", string);

    Ok(())
}
