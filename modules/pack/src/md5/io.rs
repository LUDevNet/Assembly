//! # MD5 hash on `Read`/`Write`

use std::io::{Read, Write};

use md5::Context;

use super::MD5Sum;

/// # Wrapper for [`std::io`] traits.
///
/// This struct calculates an MD5 hash as data passes through
///
/// **Note:** The hash will be valid for neither if read and write are interleaved
///
/// Additionally, this struct does not pass-through [`std::io::Seek`], as this may
/// change the hash of data that was already written.
pub struct IOSum<I> {
    inner: I,
    context: Context,
    bytes: usize,
}

impl<I> IOSum<I> {
    /// Create a mutable reference to the inner reader
    pub fn get_mut(&mut self) -> &mut I {
        &mut self.inner
    }

    /// Create a mutable reference to the inner reader
    pub fn into_inner(self) -> (I, MD5Sum) {
        (self.inner, MD5Sum(self.context.compute().0))
    }

    /// Create a new Instance
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            context: Context::new(),
            bytes: 0,
        }
    }

    /// Get the MD5 digest
    pub fn digest(&self) -> MD5Sum {
        MD5Sum(self.context.clone().compute().0)
    }

    /// Get the byte count
    pub fn byte_count(&self) -> usize {
        self.bytes
    }
}

impl<R: Read> Read for IOSum<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.inner.read(buf)?;
        self.context.consume(&buf[..len]);
        self.bytes += len;
        Ok(len)
    }
}

impl<I: Write> Write for IOSum<I> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.inner.write(buf)?;
        self.context.consume(&buf[..len]);
        self.bytes += len;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

// FIXME: Can't seek through a hash
/*
impl<I: Seek> Seek for IOSum<I> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}
*/
