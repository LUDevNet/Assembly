//! # [std::io::Write] adapters for `*.sd0` writing

use std::{
    fmt,
    io::{self, ErrorKind, Seek, SeekFrom, Write},
};

use flate2::{write::ZlibEncoder, Compress, FlushCompress};
use thiserror::Error;

pub use flate2::Compression;

use super::CHUNK_LEN;

/// Writes a segmented stream
/* FIXME: pub */
struct SegmentedEncoderRaw<W> {
    inner: W,
    data: Compress,
    /// Invariant: written < CHUNK_LEN
    written: usize,
    consumed: usize,

    buf_in: Vec<u8>,
    buf_out: Vec<u8>,
}

impl<W: Write + Seek> SegmentedEncoderRaw<W> {
    /// Create a new SegmentedEncoder
    #[allow(dead_code)]
    pub fn new(level: Compression, mut inner: W) -> io::Result<Self> {
        inner.write_all(super::MAGIC)?;
        inner.write_all(&0u32.to_le_bytes())?;
        Ok(Self {
            inner,
            written: 0,
            consumed: 0,
            buf_in: Vec::with_capacity(1024),
            buf_out: Vec::with_capacity(1024),
            data: Compress::new(level, true),
        })
    }
}

impl<W: Write + Seek> Write for SegmentedEncoderRaw<W> {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let sum = self.consumed + buf.len();

        // Whether the input spills over to the next chunk
        let _spillover = sum > CHUNK_LEN;

        // How many bytes are available for compression in the current chunk
        let mut z_avail = (CHUNK_LEN - self.consumed).min(buf.len());

        // How many bytes were taken from the input buffer
        let mut d_in = 0;
        loop {
            let c_in = self.data.total_in();
            let buf_in_avail = self.buf_in.capacity() - self.buf_in.len();
            let z_input = if self.buf_in.is_empty() {
                &buf[..z_avail]
            } else {
                // FIXME: is this MIN(len,avail) the best amount to take?
                let take = buf_in_avail.min(z_avail);
                self.buf_in.extend_from_slice(&buf[..take]);
                d_in += take;
                buf = &buf[take..z_avail];
                self.buf_in.as_slice()
            };

            // Check whether this is the last slice to write
            let flush = if self.consumed + d_in == CHUNK_LEN {
                FlushCompress::Finish
            } else {
                FlushCompress::None
            };

            // Do the compression
            let status = self.data.compress_vec(z_input, &mut self.buf_out, flush)?;
            let consumed = (self.data.total_in() - c_in) as usize;

            // Fix the input buffers
            if self.buf_in.is_empty() {
                d_in += consumed;
                buf = &buf[consumed..];
            } else {
                self.buf_in.splice(..consumed, std::iter::empty());
            }

            // Update bytes to compress
            z_avail -= consumed;

            // Write to the stream
            self.inner.write_all(&self.buf_out)?;
            self.written += self.buf_out.len();
            self.buf_out.clear();

            match status {
                flate2::Status::Ok | flate2::Status::BufError => {
                    if z_avail == 0 {
                        break;
                    }
                }
                flate2::Status::StreamEnd => break,
            }
        }
        self.consumed += d_in;

        /*if spillover {
            self.write_all(&buf[..avail])?;

            // Patch the write count
            const DIFF: i64 = CHUNK_LEN as i64;
            self.inner.seek(SeekFrom::Current(- DIFF - 4))?;
            self.inner.write_all(&(sum as u32).to_le_bytes())?;
            self.inner.seek(SeekFrom::Current(DIFF))?;

            self.write_all(&0u32.to_le_bytes())?;
            self.written = 0;

            Ok(avail)
        } else {
            self.write_all(&buf)?;
            self.written = sum;
            Ok(buf.len())
        }*/
        Ok(d_in)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let pos = self.inner.stream_position()?;

        let diff = self.written as i64;
        self.inner.seek(SeekFrom::Current(-diff - 4))?;
        self.inner.write_all(&(self.written as u32).to_le_bytes())?;
        self.inner.seek(SeekFrom::Start(pos))?;

        self.inner.flush()
    }
}

#[derive(Debug, Error)]
/// An Error
pub enum Error {
    /// I/O Error
    Io(#[from] io::Error),
    /// Called finish on invalid
    FinishOnInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FinishOnInvalid => write!(f, "Called finish on invalid"),
            Self::Io(_) => write!(f, "I/O error"),
        }
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(ErrorKind::Other, e)
    }
}

/// A result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum EncoderKind<W: Write> {
    Ok(ZlibEncoder<W>),
    Initial(W),
    Invalid,
}

impl<W: Write + Seek> EncoderKind<W> {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Self::Invalid)
    }

    fn finish(self) -> Result<W> {
        match self {
            Self::Ok(mut z) => {
                let mut total = z.total_out();
                let a_pos = z.get_mut().stream_position()?;
                let mut inner = z.finish()?;
                let b_pos = inner.stream_position()?;
                total += b_pos - a_pos;
                patch_total(&mut inner, total as u32)?;
                Ok(inner)
            }
            Self::Initial(w) => Ok(w),
            Self::Invalid => Err(Error::FinishOnInvalid),
        }
    }
}

/// A `sd0` encoder
pub struct SegmentedEncoder<W: Write + Seek> {
    inner: EncoderKind<W>,
    level: Compression,
}

impl<W: Write + Seek> Drop for SegmentedEncoder<W> {
    fn drop(&mut self) {
        // Must not panic
        let _ = self.finish();
    }
}

impl<W: Write + Seek> SegmentedEncoder<W> {
    /// Create a new encoder
    pub fn new(mut inner: W, level: Compression) -> Result<Self> {
        inner.write_all(super::MAGIC)?;
        Ok(Self {
            level,
            inner: EncoderKind::Initial(inner),
        })
    }
}

fn patch_total<W: Write + Seek>(inner: &mut W, total: u32) -> Result<()> {
    let ti64 = i64::from(total);
    inner.seek(SeekFrom::Current(-4 - ti64))?;
    inner.write_all(&total.to_le_bytes())?;
    inner.seek(SeekFrom::Current(ti64))?;
    Ok(())
}

impl<W: Write + Seek> SegmentedEncoder<W> {
    /// Finish the current block and return the inner writer
    pub fn finish(&mut self) -> Result<W> {
        let inner = self.inner.take().finish()?;
        Ok(inner)
    }
}

impl<W: Write + Seek> Write for SegmentedEncoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let EncoderKind::Ok(z) = &mut self.inner {
            let sum = z.total_out() as usize + buf.len();

            let spillover = sum > CHUNK_LEN;

            // Calculate the number of bytes to write
            let avail = if spillover {
                CHUNK_LEN - z.total_out() as usize
            } else {
                buf.len()
            };

            // Write the bytes
            z.write_all(&buf[..avail])?;

            if sum >= CHUNK_LEN {
                let w = self.finish()?;
                self.inner = EncoderKind::Initial(w);
            }

            if spillover {
                self.write(&buf[avail..]).map(|l| avail + l)
            } else {
                Ok(avail)
            }
        } else if let EncoderKind::Initial(mut w) = self.inner.take() {
            w.write_all(&0u32.to_le_bytes())?;
            self.inner = EncoderKind::Ok(ZlibEncoder::new(w, self.level));
            self.write(buf)
        } else {
            panic!("Called write on an invalid encoder");
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let EncoderKind::Ok(z) = &mut self.inner {
            z.flush()?;
            let total = z.total_out() as u32;
            patch_total(z.get_mut(), total)?;
        }
        Ok(())
    }
}
