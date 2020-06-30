//! # This module contains Read/Write adapters for sd0 reading and writing
//!
//!
use assembly_core::borrow::Oom;
use libflate::deflate::Decoder as ZlibDecoder;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{BufReader, Error as IoError, ErrorKind, Read, Result as IoResult};
use std::num::TryFromIntError;

/// # Error type for segmented streams
#[derive(Debug)]
pub enum SegmentedError {
    MagicMismatch([u8; 5]),
    Read(IoError),
    TryFromInt(TryFromIntError),
    #[cfg(debug_assertions)]
    NotImplemented,
    ZlibMissing,
}

/// Result with segmented error
pub type SegmentedResult<T> = Result<T, SegmentedError>;

impl Error for SegmentedError {}

impl Display for SegmentedError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SegmentedError::MagicMismatch(arr) => write!(f, "Magic is wrong: {:?}", arr),
            SegmentedError::Read(io) => io.fmt(f),
            SegmentedError::TryFromInt(tfi) => tfi.fmt(f),
            #[cfg(debug_assertions)]
            SegmentedError::NotImplemented => write!(f, "Not implemented!"),
            SegmentedError::ZlibMissing => write!(f, "ZLIB Decoder is None!"),
        }
    }
}

impl From<TryFromIntError> for SegmentedError {
    fn from(e: TryFromIntError) -> SegmentedError {
        SegmentedError::TryFromInt(e)
    }
}

/// # `Read`-Stream wrapper for sd0
///
/// This structure wraps an inner stream, which it treats as an sd0 stream.
/// Initially, the stream does not contain anything, when you call `next_chunk`
/// on an empty stream, the struct can be read again.
pub struct SegmentedChunkStream<'a, T> {
    inner: Oom<'a, T>,
    chunk_remain: usize,
}

pub type ChunkStreamReader<'a, T> = BufReader<SegmentedChunkStream<'a, T>>;
pub type DecoderType<'a, T> = ZlibDecoder<ChunkStreamReader<'a, T>>;
type DecoderOptionType<'a, T> = Option<DecoderType<'a, T>>;
type DecoderOptionResult<'a, T> = SegmentedResult<DecoderOptionType<'a, T>>;

impl<'a, T> SegmentedChunkStream<'a, T>
where
    T: Read,
{
    /// Create a new empty chunk stream reader from some reference or object
    pub fn new<'b, I: Into<Oom<'b, T>>>(some: I) -> SegmentedChunkStream<'b, T> {
        SegmentedChunkStream {
            inner: some.into(),
            chunk_remain: 0,
        }
    }

    /// Create a new empty chunk stream reader from some reference or object,
    /// initialized to the first chunk
    pub fn init<'b, I: Into<Oom<'b, T>>>(some: I) -> DecoderOptionResult<'b, T> {
        SegmentedChunkStream::new(some).next_chunk()
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

impl<'a, T> Read for SegmentedChunkStream<'a, T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let buf_len = buf.len();
        //println!("BUF: {}", buf_len);
        //println!("REM: {}", self.chunk_remain);
        match if self.chunk_remain == 0 {
            Ok(0)
        } else if buf_len > self.chunk_remain {
            let max = self.chunk_remain;
            //println!("MAX: {}", max);
            self.chunk_remain = 0;
            self.inner.as_mut().read(&mut buf[..max])
        } else {
            self.chunk_remain -= buf_len;
            self.inner.as_mut().read(buf)
        } {
            Ok(n) => {
                //println!("RES: {}", n);
                Ok(n)
            }
            Err(e) => {
                //println!("ERR: {:?}", e);
                Err(e)
            }
        }
    }
}

/// # A `sd0` streamed file
pub struct SegmentedStream<'a, T> {
    /// The currently active decoder
    decoder: Option<DecoderType<'a, T>>,
}

impl<'a, T> SegmentedStream<'a, T>
where
    T: Read,
{
    fn check_magic<'b>(inner: &'b mut T) -> SegmentedResult<()> {
        let mut magic: [u8; 5] = [0; 5];
        inner.read_exact(&mut magic).map_err(SegmentedError::Read)?;
        if magic == ['s' as u8, 'd' as u8, '0' as u8, 0x01, 0xff] {
            Ok(())
        } else {
            Err(SegmentedError::MagicMismatch(magic))
        }
    }

    /// Create a new stream from a reference to a `Read` object
    pub fn new(inner: &'a mut T) -> SegmentedResult<Self> {
        SegmentedStream::check_magic(inner)?;
        let cs = SegmentedChunkStream::init(inner)?;
        Ok(SegmentedStream { decoder: cs })
    }

    /// Create a new stream from a `Read` object
    pub fn try_from(mut inner: T) -> SegmentedResult<Self> {
        SegmentedStream::check_magic(&mut inner)?;
        let cs = SegmentedChunkStream::init(inner)?;
        Ok(SegmentedStream { decoder: cs })
    }

    /// Internal function to swap out the decoder
    fn next_or_done(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        match std::mem::replace(&mut self.decoder, None) {
            Some(decoder) => match decoder.into_inner().into_inner().next_chunk() {
                Ok(Some(mut decoder)) => {
                    let next_read_len = decoder.read(buf)?;
                    // Store the new decoder
                    std::mem::replace(&mut self.decoder, Some(decoder));
                    // Returned `next_read_len` bytes from the next chunk
                    Ok(next_read_len)
                }
                // There is no upcoming chunk
                Ok(None) => Ok(0),
                // Some error occured
                Err(e) => Err(IoError::new(ErrorKind::Other, e)),
            },
            // This should never ever happen, as we have recently read from the decode
            None => panic!("The sd0 zlib decoder is gone!"),
        }
    }
}

impl<'a, T> Read for SegmentedStream<'a, T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        match &mut self.decoder {
            Some(decoder) => {
                let read_len = decoder.read(buf)?;
                // Successfully ead read_len of buf.len() bytes
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
