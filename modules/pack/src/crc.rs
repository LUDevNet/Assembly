//! # CRC digest of resource paths
//!
//! The game uses a 32-bit Cyclic-Redundancy-Check (CRC)
//! as a hash-value for a relative path.
//!
//! For details of the calculation, see [`calculate_crc`].

use std::fmt;

use crc::{Crc, CRC_32_MPEG_2};
use serde::{Deserialize, Serialize};

fn normalize_char(b: u8) -> u8 {
    match b {
        b'/' => b'\\',
        b'A'..=b'Z' => b + 0x20,
        _ => b,
    }
}

const ALG: Crc<u32> = Crc::<u32>::new(&CRC_32_MPEG_2);

/// Hash-Value for a relative path
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct CRC(u32);

impl CRC {
    /// Create a new CRC value from a raw 32-bit integer
    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }

    /// Create a new CRC value from a sequence of bytes
    pub fn from_path<P: AsRef<[u8]>>(path: P) -> Self {
        calculate_crc(path.as_ref())
    }

    /// Get the raw CRC value
    pub fn to_raw(&self) -> u32 {
        self.0
    }
}

impl From<u32> for CRC {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for CRC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Calculate the Cyclic-Redundancy-Check for a file path
///
/// The game uses [CRC-32/MPEG-2], transforms all letters to lowercase,
/// replaces slashes with backslashes and appends 4 NULL bytes.
///
/// [CRC-32/MPEG-2]: https://reveng.sourceforge.io/crc-catalogue/17plus.htm#crc.cat.crc-32-mpeg-2
pub fn calculate_crc(path: &[u8]) -> CRC {
    let mut crc = ALG.digest();

    let mut s = 0;
    for (i, b) in path.iter().copied().enumerate() {
        let n = normalize_char(b);
        if n != b {
            if i > s {
                crc.update(&path[s..i]);
            }
            crc.update(&[n]);
            s = i + 1;
        }
    }
    crc.update(&path[s..]);

    // I have no clue why this was added
    crc.update(&[0, 0, 0, 0]);

    CRC(crc.finalize())
}
