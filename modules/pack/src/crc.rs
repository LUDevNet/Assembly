//! # CRC digest of resource paths

use crc::{Crc, CRC_32_MPEG_2};

fn normalize_char(b: u8) -> u8 {
    match b {
        b'/' => b'\\',
        b'A'..=b'Z' => b + 0x20,
        _ => b,
    }
}

const ALG: Crc<u32> = Crc::<u32>::new(&CRC_32_MPEG_2);

/// Calculate the Cyclic-Redundancy-Check for a file path
///
/// The game uses [CRC-32/MPEG-2], transforms all letters to lowercase,
/// replaces slashes with backslashes and appends 4 NULL bytes.
///
/// [CRC-32/MPEG-2]: https://reveng.sourceforge.io/crc-catalogue/17plus.htm#crc.cat.crc-32-mpeg-2
pub fn calculate_crc(path: &[u8]) -> u32 {
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

    crc.finalize()
}
