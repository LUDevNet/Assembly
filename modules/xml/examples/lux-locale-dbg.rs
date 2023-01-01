use std::{collections::BTreeMap, path::PathBuf};

use assembly_xml::localization::{load_locale, Interner, Key, LocaleNodeRef};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    path: PathBuf,
}

#[derive(Default, Debug)]
struct Hist {
    by_key: BTreeMap<Key, usize>,
}

impl Hist {
    fn process(&mut self, node: LocaleNodeRef) {
        for (k, v) in node.str_child_iter() {
            self.process(v);
            *self.by_key.entry(k.key()).or_default() += 1;
        }
        for (_k, v) in node.int_child_iter() {
            self.process(v);
        }
    }

    fn lengths(&self, strs: &Interner) -> BTreeMap<usize, usize> {
        let mut hist = BTreeMap::new();
        for key in self.by_key.keys() {
            let string = strs.lookup(*key);
            *hist.entry(string.len()).or_default() += 1;
        }
        hist
    }
}

fn main() -> color_eyre::Result<()> {
    let opt = Options::from_args();

    let locale_root = load_locale(&opt.path)?;
    println!("Done Loading");
    let mut hist = Hist::default();
    hist.process(locale_root.as_ref());
    let lengths = hist.lengths(locale_root.strs());

    //println! {"{:#?}", lengths};
    println!("{:#?}", &lengths);
    println!("#strings: {}", lengths.values().copied().sum::<usize>());
    println!(
        "#strings: {}",
        lengths.iter().map(|(l, r)| l * r).sum::<usize>()
    );

    Ok(())
}
