use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

use assembly_data::xml::common::exact::{expect_attribute, expect_end, expect_start, expect_text};
use color_eyre::eyre::eyre;
use quick_xml::{Reader as XmlReader, events::Event as XmlEvent};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    path: PathBuf,
    prefix: String,
}

fn main() -> color_eyre::Result<()> {
    let opt = Options::from_args();

    let file = File::open(opt.path)?;
    let file = BufReader::new(file);

    let mut reader = XmlReader::from_reader(file);
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    if let Ok(XmlEvent::Decl(_)) = reader.read_event(&mut buf) {}
    buf.clear();

    let _ = expect_start("localization", &mut reader, &mut buf)?;
    //println!("<localization>");
    buf.clear();

    let e_locales = expect_start("locales", &mut reader, &mut buf)?;
    //println!("<locales>");
    let locale_count = expect_attribute("count", &reader, &e_locales)?;
    buf.clear();

    for _ in 0..locale_count {
        let _ = expect_start("locale", &mut reader, &mut buf)?;
        //print!("<locale>");
        buf.clear();

        let _locale = expect_text(&mut reader, &mut buf)?;
        //print!("{}", locale);
        buf.clear();

        let _ = expect_end("locale", &mut reader, &mut buf)?;
        //println!("</locale>");
        buf.clear();
    }

    let _ = expect_end("locales", &mut reader, &mut buf)?;
    buf.clear();
    //println!("</locales>");

    let mut dict = BTreeMap::new();

    let e_locales = expect_start("phrases", &mut reader, &mut buf)?;
    //println!("<phrases>");
    let phrase_count = expect_attribute("count", &reader, &e_locales)?;
    buf.clear();

    for _ in 0..phrase_count {
        let e_phrase = expect_start("phrase", &mut reader, &mut buf)?;
        let id: String = expect_attribute("id", &reader, &e_phrase)?;

        let key = id.strip_prefix(&opt.prefix).map(|x| x.to_owned());
        buf.clear();

        let mut translation = None;

        loop {
            let event = reader.read_event(&mut buf)?;
            let e_translation = match event {
                XmlEvent::End(e) => {
                    if e.name() == b"phrase" {
                        break;
                    } else {
                        let name_str = reader.decode(e.name());
                        return Err(eyre!("Unexpected end tag </{}>", name_str));
                    }
                }
                XmlEvent::Start(e) => {
                    if e.name() == b"translation" {
                        e
                    } else {
                        let name_str = reader.decode(e.name());
                        return Err(eyre!("Unexpected tag <{}>", name_str));
                    }
                }
                _ => panic!(),
            };
            let locale: String = expect_attribute("locale", &reader, &e_translation)?;
            buf.clear();

            let trans = expect_text(&mut reader, &mut buf)?;
            if &locale == "en_US" {
                translation = Some(trans);
            }
            buf.clear();

            let _ = expect_end("translation", &mut reader, &mut buf)?;
            buf.clear();
        }

        if let (Some(key), Some(translation)) = (key, translation) {
            dict.insert(key, translation);
        }
    }

    let _ = expect_end("phrases", &mut reader, &mut buf)?;
    //println!("</phrases>");
    buf.clear();

    let string: String = serde_json::to_string(&dict)?;
    println!("{}", string);

    Ok(())
}
