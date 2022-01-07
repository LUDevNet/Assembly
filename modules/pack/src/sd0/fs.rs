//! # Compression for use in a script
//!
//! This is a complete re-implementation of the sd0 encoding, separate from the rest of this crate.

use std::{
    ffi::OsStr,
    fs::File,
    io::{self, BufWriter, Cursor, Read, Write},
    path::Path,
};

pub use flate2::Compression;
use flate2::FlushCompress;

use crate::{
    md5::{io::IOSum, MD5Sum},
    txt::{FileLine, FileMeta},
};

use super::{
    index::{HeaderLine, SegmentLine},
    CHUNK_LEN, SEGMENT_SIZE,
};

/// SD0 Converter
pub struct Converter {
    /// Whether to generate 'si0' files
    pub generate_segment_index: bool,
}

const fn compress_bound(source_len: usize) -> usize {
    source_len + (source_len >> 12) + (source_len >> 14) + (source_len >> 25) + 13
}

const CHUNK_BOUND: usize = compress_bound(CHUNK_LEN);

impl Converter {
    /// Convert a file to sd0
    pub fn convert_file(&self, input: &Path, output: &Path) -> io::Result<FileLine> {
        let mut input_file = IOSum::new(File::open(input)?);
        let mut output_file = IOSum::new(File::create(output)?);

        let mut index_file = if self.generate_segment_index {
            Some(Vec::<u8>::new())
        } else {
            None
        };

        let mut raw = Vec::<u8>::with_capacity(CHUNK_LEN);
        let mut compressed = Vec::<u8>::with_capacity(CHUNK_BOUND);

        let mut start: u32 = 0;
        let mut compressed_start: u32 = 0;

        let level = Compression::best();
        let mut cmp = flate2::Compress::new(level, true);

        output_file.write_all(b"sd0\x01\xff")?;

        loop {
            let mut limited = input_file.take(CHUNK_LEN as u64);
            let size = limited.read_to_end(&mut raw)? as u32;

            if size == 0 {
                input_file = limited.into_inner();
                break;
            }

            cmp.compress_vec(&raw, &mut compressed, FlushCompress::Finish)?;
            cmp.reset();

            let compressed_size = compressed.len() as u32;

            if let Some(index_data) = &mut index_file {
                let line = SegmentLine {
                    start,
                    size,
                    adler: adler32::adler32(Cursor::new(&raw))?,
                    raw_hash: MD5Sum(md5::compute(&raw).0),
                    compressed_start,
                    compressed_size,
                    compressed_hash: MD5Sum(md5::compute(&compressed).0),
                };
                write!(index_data, "{}\r", line)?;
            }

            input_file = limited.into_inner();
            compressed_start += 4;
            output_file.write_all(&compressed_size.to_le_bytes())?;
            compressed_start += compressed_size;
            output_file.write_all(&compressed)?;

            start += size;

            // Clear the buffers
            raw.clear();
            compressed.clear();

            if (size as usize) < CHUNK_LEN {
                break;
            }
        }

        input_file.flush()?;
        output_file.flush()?;

        let raw_meta = FileMeta {
            size: input_file.byte_count() as u32,
            hash: MD5Sum(input_file.digest().0),
        };
        let compressed_meta = FileMeta {
            size: output_file.byte_count() as u32,
            hash: MD5Sum(output_file.digest().0),
        };

        if let Some(index_data) = index_file {
            let new_ext = match output.extension() {
                Some(ext) => {
                    let mut e = ext.to_owned();
                    e.push(OsStr::new(".sd0"));
                    e
                }
                None => OsStr::new("sd0").to_owned(),
            };
            let path = output.with_extension(new_ext);
            let mut writer = BufWriter::new(File::create(path)?);

            let header = HeaderLine {
                magic: "si0\\x01\\xff",
                raw_size: raw_meta.size,
                raw_hash: raw_meta.hash,
                segment_size: SEGMENT_SIZE,
            };
            write!(writer, "{}\r", header)?;
            writer.write_all(&index_data)?;
        }

        Ok(FileLine::new(raw_meta, compressed_meta))
    }
}
