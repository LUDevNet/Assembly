//! # The segmented (`*.sd0`) compression format
//!
//! This format is used to deflate (zlib) the data served from the server to the client,
//! and to use less space in the pack archives.
//!
//! ## Serialization
//!
//! ```text
//! [L:5] 's' 'd' '0' 0x01 0xff
//! repeated:
//!     [u32] length V (of the following chunk)
//!     [L:V] zlib stream (deflate with zlib header)
//! ```

/// The magic bytes for the sd0 format
pub const MAGIC: &[u8; 5] = b"sd0\x01\xff";

/// Invariant: less than -i64::MIN - 4
const SEGMENT_SIZE: u32 = 0x40000;
const CHUNK_LEN: usize = SEGMENT_SIZE as usize;

use std::io::Cursor;

pub use flate2::Compression;

pub mod fs;
pub mod index;
pub mod read;
pub mod write;

/// Encode a byte slice into a vector
pub fn encode<B: AsRef<[u8]>>(
    data: B,
    output: &mut Vec<u8>,
    level: Compression,
) -> write::Result<()> {
    let input = data.as_ref();
    let mut reader = Cursor::new(input);

    let writer = Cursor::new(output);

    let mut writer = write::SegmentedEncoder::new(writer, level)?;
    std::io::copy(&mut reader, &mut writer)?;
    Ok(())
}

/// Decode a byte slice into a vector
pub fn decode<B: AsRef<[u8]>>(data: B, output: &mut Vec<u8>) -> read::Result<()> {
    let mut writer = Cursor::new(output);

    let compressed = Cursor::new(data);
    let mut reader = read::SegmentedDecoder::new(compressed)?;

    std::io::copy(&mut reader, &mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sd0::encode;

    use super::{decode, Compression};
    use std::io;

    fn roundtrip(data: &[u8]) -> io::Result<Vec<u8>> {
        let mut compressed = Vec::with_capacity(data.len() / 2);
        super::encode(data, &mut compressed, Compression::best())?;
        let mut decompressed = Vec::with_capacity(data.len());
        super::decode(&compressed, &mut decompressed)?;
        Ok(decompressed)
    }

    #[test]
    fn test_roundtrip() {
        let short = lipsum::lipsum(100);
        let test = roundtrip(short.as_bytes()).unwrap();
        assert_eq!(&test, short.as_bytes());
    }

    #[test]
    fn test_decode_empty() {
        let empty = super::MAGIC;
        let mut output = Vec::new();
        decode(empty, &mut output).unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_encode_empty() {
        let empty = &[];
        let mut output = Vec::new();
        encode(empty, &mut output, Compression::best()).unwrap();
        assert_eq!(output, super::MAGIC);
    }
}
