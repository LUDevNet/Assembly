//! # Implements primary key hashing
//!

use std::ops::Deref;

use latin1str::{Latin1Str, Latin1String};

use crate::value::{Context, Value};

/// This trait is implemented on all types that represent a field value
pub trait FdbHash {
    /// Get the hash value
    fn hash(&self) -> u32;
}

impl FdbHash for u32 {
    fn hash(&self) -> u32 {
        *self
    }
}

impl FdbHash for i32 {
    fn hash(&self) -> u32 {
        u32::from_ne_bytes(self.to_ne_bytes())
    }
}

impl FdbHash for f32 {
    fn hash(&self) -> u32 {
        u32::from_ne_bytes(self.to_ne_bytes())
    }
}

impl FdbHash for Latin1Str {
    fn hash(&self) -> u32 {
        sfhash::digest(self.as_bytes())
    }
}

impl FdbHash for Latin1String {
    fn hash(&self) -> u32 {
        FdbHash::hash(self.deref())
    }
}

impl FdbHash for str {
    fn hash(&self) -> u32 {
        FdbHash::hash(Latin1String::encode(self).as_ref())
    }
}

impl FdbHash for String {
    fn hash(&self) -> u32 {
        FdbHash::hash(self.as_str())
    }
}

impl FdbHash for i64 {
    fn hash(&self) -> u32 {
        FdbHash::hash(&u64::from_ne_bytes(self.to_ne_bytes()))
    }
}

impl FdbHash for u64 {
    fn hash(&self) -> u32 {
        (self & 0xFFFFFFFF) as u32
    }
}

impl FdbHash for bool {
    fn hash(&self) -> u32 {
        match *self {
            true => 1,
            false => 0,
        }
    }
}

impl<C: Context> FdbHash for Value<C>
where
    C::I64: FdbHash,
    C::String: FdbHash,
    C::XML: FdbHash,
{
    fn hash(&self) -> u32 {
        match self {
            Value::Nothing => 0,
            Value::Integer(i) => FdbHash::hash(i),
            Value::Float(f) => FdbHash::hash(f),
            Value::Text(f) => FdbHash::hash(f),
            Value::Boolean(b) => FdbHash::hash(b),
            Value::BigInt(i) => FdbHash::hash(i),
            // FIXME: This may not be correct
            Value::VarChar(v) => FdbHash::hash(v),
        }
    }
}
