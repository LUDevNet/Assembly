//! # This module contains Read/Write adapters for sd0 reading and writing
//!
//!
use std::io::{Read, BufReader, Error as IoError, Result as IoResult, ErrorKind};
use flate2::bufread::ZlibDecoder;
use std::convert::{TryFrom, From};
use std::num::TryFromIntError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;
use crate::core::borrow::Oom;

#[derive(Debug)]
pub enum SegmentedError {
    MagicMismatch([u8;5]),
    Read(IoError),
    TryFromInt(TryFromIntError),
    #[cfg(debug_assertions)]
    NotImplemented,
    ZlibMissing,
}

type SegmentedResult<T> = Result<T, SegmentedError>;

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

pub struct SegmentedChunkStream<'a, T> {
    inner: Oom<'a, T>,
    chunk_remain: usize,
}

pub type ChunkStreamReader<'a, T> = BufReader<SegmentedChunkStream<'a, T>>;
pub type DecoderType<'a,T> = ZlibDecoder<ChunkStreamReader<'a, T>>;


impl<'a, T> SegmentedChunkStream<'a, T>
where T: Read {
    fn new<'b, I: Into<Oom<'b, T>>>(some: I) -> SegmentedChunkStream<'b, T> {
        SegmentedChunkStream{inner: some.into(), chunk_remain: 0}
    }

    fn init<'b, I: Into<Oom<'b, T>>>(some: I) -> SegmentedResult<DecoderOptionType<'b, T>> {
        SegmentedChunkStream::new(some).next_chunk()
    }

    fn next_chunk(mut self) -> SegmentedResult<DecoderOptionType<'a, T>> {
        let mut chunk_size_bytes: [u8; 4] = [0; 4];
        match self.inner.as_mut().read_exact(&mut chunk_size_bytes) {
            Ok(()) => {
                let size = u32::from_le_bytes(chunk_size_bytes);
                //println!("CHNK: {}", size);
                self.chunk_remain = usize::try_from(size)?;
                let buf_read = BufReader::new(self);
                Ok(Some(ZlibDecoder::new(buf_read)))
            },
            Err(_) => {
                Ok(None)
            }
        }
    }
}

impl<'a, T> Read for SegmentedChunkStream<'a, T>
where T: Read {
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
            },
            Err(e) => {
                //println!("ERR: {:?}", e);
                Err(e)
            },
        }
    }
}

pub type DecoderOptionType<'a,T> = Option<DecoderType<'a,T>>;

pub struct SegmentedStream<'a, T> {
    decoder: DecoderOptionType<'a,T>,
}

/*pub struct OwningSegmentedStream<'a, T> {
    inner: T,
    stream: SegmentedStream<'a, T>,
}*/

impl<'a, T> SegmentedStream<'a, T>
where T: Read {
    pub fn check_magic<'b>(inner: &'b mut T) -> Result<(), SegmentedError> {
        let mut magic: [u8;5] = [0;5];
        inner.read_exact(&mut magic).map_err(SegmentedError::Read)?;
        if magic == ['s' as u8, 'd' as u8, '0' as u8, 0x01, 0xff] {
            Ok(())
        } else {
            Err(SegmentedError::MagicMismatch(magic))
        }
    }

    pub fn new<'b>(inner: &'b mut T) -> Result<SegmentedStream<'b, T>, SegmentedError> {
        SegmentedStream::check_magic(inner)?;
        let cs = SegmentedChunkStream::init(inner)?;
        Ok(SegmentedStream{decoder: cs})
    }

    pub fn try_from<'b>(mut inner: T) -> Result<SegmentedStream<'b, T>, SegmentedError> {
        SegmentedStream::check_magic(&mut inner)?;
        let cs = SegmentedChunkStream::init(inner)?;
        Ok(SegmentedStream{decoder: cs})
    }
}

/*impl<'a, T> OwningSegmentedStream<'a, T>
where T: Read {
    pub fn new(mut inner: T) -> Result<OwningSegmentedStream<'a, T>, SegmentedError> {
        let mut stream = SegmentedStream::new(&mut inner)?;
        Ok(OwningSegmentedStream{stream, inner})
    }
}*/

impl<'a, T> Read for SegmentedStream<'a, T>
where T: Read {
    fn read(&mut self, buf: &mut[u8]) -> IoResult<usize> {
        //println!("Read");
        match std::mem::replace(&mut self.decoder, None) {
            Some(mut decoder) => {
                let buf_len = buf.len();
                let read_len = decoder.read(buf)?;
                //println!("DEC: {} of {}", read_len, buf_len);
                if read_len == 0 {
                    //println!("TOTAL: {}", decoder.total_out());
                    match decoder.into_inner().into_inner().next_chunk() {
                        Ok(Some(mut decoder)) => {
                            let next_read_len = decoder.read(buf)?;
                            std::mem::replace(&mut self.decoder, Some(decoder));
                            //println!("OK_CHNK: {}", next_read_len);
                            Ok(next_read_len)
                        },
                        Ok(None) => {
                            //println!("OK-DONE: {}", 0);
                            Ok(0)
                        },
                        Err(e) => Err(IoError::new(ErrorKind::Other, e)),
                    }
                } else {
                    std::mem::replace(&mut self.decoder, Some(decoder));
                    //println!("OK-FIN: {}", read_len);
                    Ok(read_len)
                }
            },
            None => Ok(0),
        }
    }
}
