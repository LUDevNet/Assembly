//! # This module contains Read adapters for sd0 reading and writing
use assembly_core::borrow::Oom;
use flate2::bufread::ZlibDecoder;
use thiserror::Error;

use std::convert::{From, TryFrom};
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufReader, ErrorKind, Read};
use std::num::TryFromIntError;

/// # Error type for segmented streams
#[derive(Debug, Error)]
pub enum Error {
    /// The magic bytes are wrong
    MagicMismatch([u8; 5]),
    /// An IO Error occured
    Read(#[source] io::Error),
    /// Integer parse failed
    TryFromInt(#[from] TryFromIntError),
}

/// Result with segmented error
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::MagicMismatch(arr) => write!(f, "Magic is wrong: {:?}", arr),
            Error::Read(_) => write!(f, "I/O error"),
            Error::TryFromInt(_) => write!(f, "Failed to parse integer"),
        }
    }
}

/// # `Read`-Stream wrapper for sd0
///
/// This structure wraps an inner stream, which it treats as an sd0 stream.
/// Initially, the stream does not contain anything, when you call `next_chunk`
/// on an empty stream, the struct can be read again.
pub struct ChunkDecoder<'a, T> {
    inner: Oom<'a, T>,
    chunk_remain: usize,
}

/// `impl BufRead` for a single chunk
pub type ChunkStreamReader<'a, T> = BufReader<ChunkDecoder<'a, T>>;
/// ZLib Decoder for a single chunk
pub type DecoderType<'a, T> = ZlibDecoder<ChunkStreamReader<'a, T>>;
type DecoderOptionType<'a, T> = Option<DecoderType<'a, T>>;
type DecoderOptionResult<'a, T> = Result<DecoderOptionType<'a, T>>;

impl<'a, T> ChunkDecoder<'a, T>
where
    T: Read,
{
    /// Create a new empty chunk stream reader from some reference or object
    pub fn new<'b, I: Into<Oom<'b, T>>>(some: I) -> ChunkDecoder<'b, T> {
        ChunkDecoder {
            inner: some.into(),
            chunk_remain: 0,
        }
    }

    /// Create a new empty chunk stream reader from some reference or object,
    /// initialized to the first chunk
    pub fn init<'b, I: Into<Oom<'b, T>>>(some: I) -> DecoderOptionResult<'b, T> {
        ChunkDecoder::new(some).next_chunk()
    }

    /// Load the next chunk
    pub fn next_chunk(mut self) -> DecoderOptionResult<'a, T> {
        let mut chunk_size_bytes: [u8; 4] = [0; 4];
        match self.inner.as_mut().read_exact(&mut chunk_size_bytes) {
            Ok(()) => {
                let size = u32::from_le_bytes(chunk_size_bytes);
                //println!("CHNK: {}", size);
                self.chunk_remain = usize::try_from(size)?;
                let buf_read = BufReader::new(self);
                Ok(Some(ZlibDecoder::new(buf_read)))
            }
            Err(_) => Ok(None),
        }
    }
}

#[derive(Debug)]
struct FailedToRead {
    inner: io::Error,
    kind: FailedToReadKind,
}

impl fmt::Display for FailedToRead {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read SDO chunk ({:?})", self.kind)
    }
}

impl std::error::Error for FailedToRead {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

#[derive(Debug)]
enum FailedToReadKind {
    Root,
    FileRead,
}

fn map_io_result(inner: io::Error) -> FailedToRead {
    FailedToRead {
        inner,
        kind: FailedToReadKind::Root,
    }
}

fn map_file_read_result(inner: io::Error) -> FailedToRead {
    FailedToRead {
        inner,
        kind: FailedToReadKind::FileRead,
    }
}

impl From<FailedToRead> for io::Error {
    fn from(error: FailedToRead) -> Self {
        io::Error::new(ErrorKind::Other, error)
    }
}

impl<'a, T> Read for ChunkDecoder<'a, T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        //println!("BUF: {}", buf_len);
        //println!("REM: {}", self.chunk_remain);
        let len = if self.chunk_remain == 0 {
            Ok(0)
        } else if buf_len > self.chunk_remain {
            let max = self.chunk_remain;
            //println!("MAX: {}", max);
            let len = self.inner.as_mut().read(&mut buf[..max])?;
            self.chunk_remain -= len;
            Ok(len)
        } else {
            let len = self.inner.as_mut().read(buf)?;
            self.chunk_remain -= len;
            Ok(len)
        }
        .map_err(map_io_result)?;
        Ok(len)
    }
}

/// # A `sd0` streamed file
pub struct SegmentedDecoder<'a, T> {
    /// The currently active decoder
    decoder: Option<DecoderType<'a, T>>,
}

impl<'a, T> SegmentedDecoder<'a, T>
where
    T: Read,
{
    fn check_magic(inner: &mut T) -> Result<()> {
        let mut magic: [u8; 5] = [0; 5];
        inner.read_exact(&mut magic).map_err(Error::Read)?;
        if magic == [b's', b'd', b'0', 0x01, 0xff] {
            Ok(())
        } else {
            Err(Error::MagicMismatch(magic))
        }
    }

    /// Create a new stream from a reference to a `Read` object
    pub fn new(inner: &'a mut T) -> Result<Self> {
        SegmentedDecoder::check_magic(inner)?;
        let cs = ChunkDecoder::init(inner)?;
        Ok(SegmentedDecoder { decoder: cs })
    }

    /// Create a new stream from a `Read` object
    pub fn try_from(mut inner: T) -> Result<Self> {
        SegmentedDecoder::check_magic(&mut inner)?;
        let cs = ChunkDecoder::init(inner)?;
        Ok(SegmentedDecoder { decoder: cs })
    }

    /// Internal function to swap out the decoder
    fn next_or_done(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match std::mem::replace(&mut self.decoder, None) {
            Some(decoder) => match decoder.into_inner().into_inner().next_chunk() {
                Ok(Some(mut decoder)) => {
                    let next_read_len = decoder.read(buf).map_err(map_io_result)?;
                    // Store the new decoder
                    self.decoder = Some(decoder);
                    // Returned `next_read_len` bytes from the next chunk
                    Ok(next_read_len)
                }
                // There is no upcoming chunk
                Ok(None) => Ok(0),
                // Some error occured
                Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
            },
            // This should never ever happen, as we have recently read from the decode
            None => panic!("The sd0 zlib decoder is gone!"),
        }
    }
}

impl<'a, T> Read for SegmentedDecoder<'a, T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.decoder {
            Some(decoder) => {
                let read_len = decoder.read(buf).map_err(map_file_read_result)?;
                // Successfully read read_len of buf.len() bytes
                if read_len == 0 {
                    // The decode stream is complete, decoder.total_out() bytes were read
                    self.next_or_done(buf)
                } else {
                    // We have read some bytes, though there may be more
                    Ok(read_len)
                }
            }
            None => Ok(0),
        }
    }
}
