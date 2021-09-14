//! # The XML `<localization>` format
//!
//! This is used in:
//! - the `locale/locale.xml` file

use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use displaydoc::Display;
use quick_xml::{events::Event as XmlEvent, Error as XmlError, Reader as XmlReader};
use thiserror::Error;

use super::common::exact::{expect_attribute, expect_end, expect_start, expect_text, Error};

#[derive(Debug, Display, Error)]
/// Some problem with loading a locale file
pub enum LocaleError {
    /// I/O Error
    Io(#[from] io::Error),
    /// Xml
    Xml(#[from] Error),
}

impl From<XmlError> for LocaleError {
    fn from(e: XmlError) -> Self {
        Self::Xml(Error::Xml(e))
    }
}
#[derive(Debug, Default)]
/// A node in the locale tree
pub struct LocaleNode {
    /// The translation at the current node
    pub value: Option<String>,
    /// The (optional) children with a numeric key
    pub int_children: BTreeMap<u32, LocaleNode>,
    /// The (optional) children with a non-numeric key
    pub str_children: BTreeMap<String, LocaleNode>,
}

impl LocaleNode {
    /// Return all keys that correspond to this node
    ///
    /// This returns a flat map of locale values
    pub fn get_keys(&self) -> BTreeMap<String, String> {
        let mut keys = BTreeMap::new();
        for (key, value) in &self.str_children {
            value.add_keys(&mut keys, key.clone());
        }
        for (key, value) in &self.int_children {
            value.add_keys(&mut keys, key.to_string());
        }
        keys
    }

    fn add_keys(&self, keys: &mut BTreeMap<String, String>, prefix: String) {
        for (key, value) in &self.str_children {
            let inner = format!("{}_{}", prefix, key);
            value.add_keys(keys, inner);
        }
        for (key, value) in &self.int_children {
            let inner = format!("{}_{}", prefix, key);
            value.add_keys(keys, inner);
        }
        if let Some(v) = &self.value {
            keys.insert(prefix, v.clone());
        }
    }
}

/// Load a locale file
pub fn load_locale(path: &Path) -> Result<LocaleNode, LocaleError> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut root = LocaleNode {
        value: None,
        int_children: BTreeMap::new(),
        str_children: BTreeMap::new(),
    };

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

    let e_locales = expect_start("phrases", &mut reader, &mut buf)?;
    //println!("<phrases>");
    let phrase_count = expect_attribute("count", &reader, &e_locales)?;
    buf.clear();

    for _ in 0..phrase_count {
        let e_phrase = expect_start("phrase", &mut reader, &mut buf)?;
        let id: String = expect_attribute("id", &reader, &e_phrase)?;

        //let key = id.strip_prefix(&opt.prefix).map(|x| x.to_owned());
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
                        return Err(LocaleError::Xml(Error::ExpectedEndTag(
                            String::from("phrase"),
                            name_str.into_owned(),
                        )));
                    }
                }
                XmlEvent::Start(e) => {
                    if e.name() == b"translation" {
                        e
                    } else {
                        let name_str = reader.decode(e.name());
                        return Err(LocaleError::Xml(Error::ExpectedEndTag(
                            String::from("translation"),
                            name_str.into_owned(),
                        )));
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

        let mut node = &mut root;
        for comp in id.split('_') {
            if let Ok(num) = comp.parse::<u32>() {
                node = node.int_children.entry(num).or_default();
            } else {
                node = node.str_children.entry(comp.to_owned()).or_default();
            }
        }
        if let Some(translation) = translation {
            node.value = Some(translation);
        }
    }

    let _ = expect_end("phrases", &mut reader, &mut buf)?;
    //println!("</phrases>");
    buf.clear();

    Ok(root)
}
