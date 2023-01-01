//! # The XML `<localization>` format
//!
//! This is used in:
//! - the `locale/locale.xml` file

use std::{
    collections::{btree_map, BTreeMap},
    fmt,
    fs::File,
    io::{self, BufReader},
    ops::Deref,
    path::Path,
};

use displaydoc::Display;
use quick_xml::{events::Event as XmlEvent, Error as XmlError, Reader as XmlReader};
use thiserror::Error;
use tinystr::TinyStrError;

use super::common::exact::{expect_attribute, expect_end, expect_start, expect_text, Error};
use crate::common::exact::{expect_child_or_end, expect_text_or_end};

mod interner;
pub use interner::Interner;

#[derive(Debug, Display, Error)]
/// Some problem with loading a locale file
pub enum LocaleError {
    /// I/O Error
    Io(#[from] io::Error),
    /// Xml
    Xml(#[from] Error),
    /// TinyStr
    TinyStr(TinyStrError),
}

impl From<TinyStrError> for LocaleError {
    fn from(value: TinyStrError) -> Self {
        Self::TinyStr(value)
    }
}

impl From<XmlError> for LocaleError {
    fn from(e: XmlError) -> Self {
        Self::Xml(Error::Xml(e))
    }
}

/// A key for [LocaleNode] children
pub type Key = interner::StringKey; //inystr::TinyAsciiStr<24>;

#[derive(Debug, Default)]
/// A node in the locale tree
pub struct LocaleNode {
    /// The translation at the current node
    pub value: Option<String>,
    /// The (optional) children with a numeric key
    pub int_children: BTreeMap<u32, LocaleNode>,
    /// The (optional) children with a non-numeric key
    pub str_children: BTreeMap<Key, LocaleNode>,
}

impl LocaleNode {
    /// Return all keys that correspond to this node
    ///
    /// This returns a flat map of locale values
    pub fn get_keys(&self, strs: &Interner) -> BTreeMap<String, String> {
        let mut keys = BTreeMap::new();
        for (key, value) in &self.str_children {
            value.add_keys(&mut keys, strs.lookup(*key).to_string(), strs);
        }
        for (key, value) in &self.int_children {
            value.add_keys(&mut keys, key.to_string(), strs);
        }
        keys
    }

    fn add_keys(&self, keys: &mut BTreeMap<String, String>, prefix: String, strs: &Interner) {
        for (key, value) in &self.str_children {
            let inner = format!("{}_{}", prefix, strs.lookup(*key));
            value.add_keys(keys, inner, strs);
        }
        for (key, value) in &self.int_children {
            let inner = format!("{}_{}", prefix, key);
            value.add_keys(keys, inner, strs);
        }
        if let Some(v) = &self.value {
            keys.insert(prefix, v.clone());
        }
    }
}

#[derive(Debug)]
/// The root of a loaded locale XML
pub struct LocaleRoot {
    /// The inner root node
    root_node: LocaleNode,
    /// The string interner
    strs: Interner,
}

impl LocaleRoot {
    /// Turn root into a reference
    pub fn as_ref(&self) -> LocaleNodeRef<'_, '_> {
        LocaleNodeRef {
            node: &self.root_node,
            strs: &self.strs,
        }
    }

    /// Get the string interner in this tree
    pub fn strs(&self) -> &Interner {
        &self.strs
    }
}

/// Iterator over String subkeys
pub struct StrKey<'s> {
    key: Key,
    strs: &'s Interner,
}

impl fmt::Display for StrKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<'s> StrKey<'s> {
    fn new(key: Key, strs: &'s Interner) -> Self {
        Self { key, strs }
    }

    /// Get the interner [Key] for this value
    pub fn key(&self) -> Key {
        self.key
    }
}

impl<'s> Deref for StrKey<'s> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.strs.lookup(self.key)
    }
}

/// Iterator over String subkeys
#[derive(Clone)]
pub struct StrNodeMap<'a, 's> {
    iter: btree_map::Iter<'a, Key, LocaleNode>,
    strs: &'s Interner,
}

impl<'a, 's> Iterator for StrNodeMap<'a, 's> {
    type Item = (StrKey<'s>, LocaleNodeRef<'a, 's>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, node)| {
            let k = StrKey::new(*key, self.strs);
            let r = LocaleNodeRef::new(node, self.strs);
            (k, r)
        })
    }
}

/// Iterator over int subkeys
#[derive(Clone)]
pub struct IntNodeMap<'a, 's> {
    iter: btree_map::Iter<'a, u32, LocaleNode>,
    strs: &'s Interner,
}

impl<'a, 's> Iterator for IntNodeMap<'a, 's> {
    type Item = (u32, LocaleNodeRef<'a, 's>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, node)| {
            let r = LocaleNodeRef {
                node,
                strs: self.strs,
            };
            (*key, r)
        })
    }
}

/// Reference to a [LocaleNode] with an [Interner]
#[derive(Clone)]
pub struct LocaleNodeRef<'a, 's> {
    node: &'a LocaleNode,
    strs: &'s Interner,
}

impl<'a, 's> LocaleNodeRef<'a, 's> {
    /// Get the string interner in this tree
    pub fn strs(&self) -> &'s Interner {
        self.strs
    }

    /// Get the actual node
    pub fn node(&self) -> &'a LocaleNode {
        self.node
    }

    /// Get the value of this [LocaleNode]
    pub fn value(&self) -> Option<&str> {
        self.node.value.as_deref()
    }

    /// Get the string children of this [LocaleNode]
    pub fn str_child_iter(&self) -> StrNodeMap {
        StrNodeMap {
            iter: self.node.str_children.iter(),
            strs: self.strs,
        }
    }

    /// Get an integer child
    pub fn get_int(&self, key: u32) -> Option<Self> {
        if let Some(node) = self.node.int_children.get(&key) {
            return Some(Self::new(node, self.strs));
        }
        None
    }

    /// Get an integer child
    pub fn get_str(&self, key: Key) -> Option<LocaleNodeRef<'a, 's>> {
        if let Some(node) = self.node.str_children.get(&key) {
            return Some(LocaleNodeRef::new(node, self.strs));
        }
        None
    }

    /// Get the integer children of this [LocaleNode]
    pub fn int_child_iter(&self) -> IntNodeMap {
        IntNodeMap {
            iter: self.node.int_children.iter(),
            strs: self.strs,
        }
    }

    fn new(node: &'a LocaleNode, strs: &'s Interner) -> Self {
        Self { node, strs }
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
pub fn load_locale(path: &Path) -> Result<LocaleRoot, LocaleError> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut root_node = LocaleNode {
        value: None,
        int_children: BTreeMap::new(),
        str_children: BTreeMap::new(),
    };
    let mut strs = Interner::with_capacity(0x4000);

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

        let mut node = &mut root_node;
        for comp in id.split('_') {
            if let Ok(num) = comp.parse::<u32>() {
                node = node.int_children.entry(num).or_default();
            } else {
                let key = strs.intern(comp); // Key::from_str(comp)?;
                node = node.str_children.entry(key).or_default();
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

    Ok(LocaleRoot { root_node, strs })
}
