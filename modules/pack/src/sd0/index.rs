//! # Segmented Index Files (si0)

use std::fmt;

use crate::md5::MD5Sum;

/// The first line is a header of the following form:
///
/// ```py
/// '%s%s:%08x:%s:%08x\r'
/// ```
///
/// with the following data:
pub struct HeaderLine {
    /// 1) the file extension `si0` and magic bytes 0x01, 0xff as `\x01\xff`
    pub magic: &'static str,
    /// 2) the total size of the input
    pub raw_size: u32,
    /// 3) the MD5 hash of the input
    pub raw_hash: MD5Sum,
    /// 4) the segment size
    pub segment_size: u32,
}

impl fmt::Display for HeaderLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{:08x}:{}:{:08x}",
            self.magic, self.raw_size, self.raw_hash, self.segment_size
        )
    }
}

/// The rest of the file is one line for every compressed block, in the following form:
///
/// ```py
/// %08x:%08x:%s:%s:%08x:%08x:%s
/// ```
///
/// with the following data:
pub struct SegmentLine {
    /// 1) start of the block in the raw file
    pub start: u32,
    /// 2) size of the block
    pub size: u32,
    /// 3) Adler32 of the raw bytes modulo 0xFFFFFFFF, as hex, with the last letter removed
    pub adler: u32,
    /// 4) MD5 hash of the raw bytes
    pub raw_hash: MD5Sum,
    /// 5) number of bytes already written to compressed file (without magic bytes)
    pub compressed_start: u32,
    /// 6) number of compressed bytes
    pub compressed_size: u32,
    /// 7) MD5 hash of compressed bytes
    pub compressed_hash: MD5Sum,
}

impl fmt::Display for SegmentLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}:{:08x}:{:x}:{}:{:08x}:{:08x}:{}",
            self.start,
            self.size,
            (self.adler % 0xFFFFFFFF) >> 4,
            self.raw_hash,
            self.compressed_start,
            self.compressed_size,
            self.compressed_hash
        )
    }
}
