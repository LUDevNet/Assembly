//! # The segmented (`*.sd0`) compression format
//!
//! This format is used to deflate (zlib) the data
//! served from the server to the client, and to
//! use less space in the pack archives.

/// The magic bytes for the sd0 format
pub const MAGIC: &[u8; 5] = b"sd0\x01\xff";

pub use flate2::Compression;

pub mod read;
pub mod write;

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor};

    use super::read::SegmentedDecoder;
    use super::write::SegmentedEncoder;
    use super::Compression;

    fn encode<B: AsRef<[u8]>>(data: B) -> super::write::Result<Vec<u8>> {
        let input = data.as_ref();
        let mut reader = Cursor::new(input);

        let bytes = Vec::<u8>::new();
        let writer = Cursor::new(bytes);

        let mut writer = SegmentedEncoder::new(writer, Compression::best())?;
        std::io::copy(&mut reader, &mut writer)?;

        let compressed = writer.finish()?;
        Ok(compressed.into_inner())
    }

    fn decode<B: AsRef<[u8]>>(data: B, capacity: usize) -> super::read::Result<Vec<u8>> {
        let bytes2 = Vec::with_capacity(capacity);
        let mut writer = Cursor::new(bytes2);

        let compressed = Cursor::new(data);
        let mut reader = SegmentedDecoder::new(compressed)?;

        std::io::copy(&mut reader, &mut writer)?;

        Ok(writer.into_inner())
    }

    fn roundtrip(data: &[u8]) -> io::Result<Vec<u8>> {
        let compressed = encode(data)?;
        let decompressed = decode(&compressed, data.len())?;
        Ok(decompressed)
    }

    #[test]
    fn test_roundtrip() {
        let short = lipsum::lipsum(100);
        let test = roundtrip(short.as_bytes()).unwrap();
        assert_eq!(&test, short.as_bytes());
    }
}
