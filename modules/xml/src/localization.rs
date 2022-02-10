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

use crate::common::exact::{expect_child_or_end, expect_text_or_end};

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

const TAG_LOCALIZATION: &str = "localization";
const TAG_LOCALES: &str = "locales";
const TAG_LOCALE: &str = "locale";
const TAG_PHRASES: &str = "phrases";
const TAG_PHRASE: &str = "phrase";
const TAG_TRANSLATION: &str = "translation";

const ATTR_COUNT: &str = "count";
const ATTR_LOCALE: &str = "locale";
const ATTR_ID: &str = "id";

const LOCALE_EN_US: &str = "en_US";

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

    let _ = expect_start(TAG_LOCALIZATION, &mut reader, &mut buf)?;
    buf.clear();

    let e_locales = expect_start(TAG_LOCALES, &mut reader, &mut buf)?;
    let locale_count: usize = expect_attribute(ATTR_COUNT, &reader, &e_locales)?;
    let mut real_locale_count = 0;
    buf.clear();

    while expect_child_or_end(TAG_LOCALE, TAG_LOCALES, &mut reader, &mut buf)?.is_some() {
        buf.clear();

        let locale = expect_text(&mut reader, &mut buf)?;
        log::debug!("Found locale '{}'", locale);

        expect_end(TAG_LOCALE, &mut reader, &mut buf)?;
        buf.clear();

        real_locale_count += 1;
    }
    buf.clear();

    if real_locale_count != locale_count {
        log::warn!(
            "locale.xml specifies a locale count of {}, but has {}",
            locale_count,
            real_locale_count
        );
    }

    let e_locales = expect_start(TAG_PHRASES, &mut reader, &mut buf)?;
    let phrase_count: usize = expect_attribute(ATTR_COUNT, &reader, &e_locales)?;
    let mut real_phrase_count = 0;
    buf.clear();

    while let Some(e_phrase) = expect_child_or_end(TAG_PHRASE, TAG_PHRASES, &mut reader, &mut buf)?
    {
        let id: String = expect_attribute(ATTR_ID, &reader, &e_phrase)?;
        buf.clear();

        let mut translation = None;

        while let Some(e_translation) =
            expect_child_or_end(TAG_TRANSLATION, TAG_PHRASE, &mut reader, &mut buf)?
        {
            let locale: String = expect_attribute(ATTR_LOCALE, &reader, &e_translation)?;
            buf.clear();

            let trans = expect_text_or_end(TAG_TRANSLATION, &mut reader, &mut buf)?;
            if locale == LOCALE_EN_US {
                translation = Some(trans);
            }
            buf.clear();
        }
        buf.clear();

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

        real_phrase_count += 1;
    }
    buf.clear();

    if phrase_count != real_phrase_count {
        log::warn!(
            "locale.xml specifies a count of {} phrases, but has {}",
            phrase_count,
            real_phrase_count
        );
    }

    Ok(root)
}
