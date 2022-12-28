use std::path::PathBuf;

use assembly_xml::localization::{load_locale, LocaleNode};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    path: PathBuf,
}

fn max_str_key_len(node: &LocaleNode) -> usize {
    let mut max = 0;
    for (k, v) in node.str_children.iter() {
        max = max.max(k.len()).max(max_str_key_len(v))
    }
    for v in node.int_children.values() {
        max = max.max(max_str_key_len(v))
    }
    max
}

fn main() -> color_eyre::Result<()> {
    let opt = Options::from_args();

    let locale_node = load_locale(&opt.path)?;
    let max = max_str_key_len(&locale_node);
    println!("max key length: {}", max);

    Ok(())
}
