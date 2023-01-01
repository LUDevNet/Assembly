use std::{collections::HashMap, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringKey(u32);

#[derive(Debug)]
/// A simple string interner
/// 
/// Source: <https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html>
pub struct Interner {
    map: HashMap<&'static str, u32>,
    vec: Vec<&'static str>,
    buf: String,
    full: Vec<String>,
}

impl Interner {
    /// Create a new instance
    pub fn with_capacity(cap: usize) -> Interner {
        let cap = cap.next_power_of_two();

        Interner {
            map: HashMap::default(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            full: Vec::new(),
        }
    }

    /// Check for the presence of a single key
    pub fn get(&self, name: &str) -> Option<StringKey> {
        self.map.get(name).copied().map(StringKey)
    }

    /// Intern a string, returning a key
    pub fn intern(&mut self, name: &str) -> StringKey {
        if let Some(&id) = self.map.get(name) {
            return StringKey(id);
        }

        let name = unsafe { self.alloc(name) };
        let id = self.map.len() as u32;
        self.map.insert(name, id);
        self.vec.push(name);
        debug_assert!(self.lookup(StringKey(id)) == name);
        debug_assert!(self.intern(name).0 == id);
        StringKey(id)
    }

    /// Lookup a [StringKey] to return a reference to [str]
    pub fn lookup(&self, id: StringKey) -> &str {
        self.vec[id.0 as usize]
    }

    unsafe fn alloc(&mut self, name: &str) -> &'static str {
        let cap = self.buf.capacity();

        if cap < self.buf.len() + name.len() {
            let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }

        let interned = {
            let start = self.buf.len();
            self.buf.push_str(name);
            &self.buf[start..]
        };

        &*(interned as *const str)
    }
}
