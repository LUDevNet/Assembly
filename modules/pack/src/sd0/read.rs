//! # [std::io::Read] adapters for `*.sd0` reading
use flate2::{read::ZlibDecoder, Decompress, FlushDecompress};
use thiserror::Error;

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Take};

/// # Error type for segmented streams
#[derive(Debug, Error)]
pub enum Error {
    /// The magic bytes are wrong
    MagicMismatch([u8; 5]),
    /// An IO Error occured
    IO(#[from] io::Error),
    /// called io::Read::read again after an error
    DecoderInvalid,
}

/// Result with segmented error
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::MagicMismatch(arr) => write!(f, "Magic is wrong: {:?}", arr),
            Error::IO(_) => write!(f, "I/O error"),
            Error::DecoderInvalid => write!(f, "Called io::Read again after an error"),
        }
    }
}

fn check_magic<T: Read>(inner: &mut T) -> Result<()> {
    let mut magic: [u8; 5] = [0; 5];
    inner.read_exact(&mut magic)?;
    if &magic == super::MAGIC {
        Ok(())
    } else {
        Err(Error::MagicMismatch(magic))
    }
}

fn read_size<T: Read>(inner: &mut T) -> io::Result<Option<u32>> {
    let mut chunk_size_bytes = [0u8; 4];
    let len = inner.read(&mut chunk_size_bytes)?;
    if len < 4 {
        if len == 0 {
            return Ok(None);
        }
        inner.read_exact(&mut chunk_size_bytes[len..])?;
    }
    let len = u32::from_le_bytes(chunk_size_bytes);
    Ok(Some(len))
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(ErrorKind::Other, e)
    }
}

enum DecoderKind<T> {
    Ok(ZlibDecoder<Take<T>>),
    Initial(T),
    Invalid,
}

impl<T> DecoderKind<T> {
    fn take(&mut self) -> Self {
        std::mem::replace(self, DecoderKind::Invalid)
    }

    fn try_get_mut(&mut self) -> Result<&mut T> {
        match self {
            Self::Ok(z) => Ok(z.get_mut().get_mut()),
            Self::Invalid => Err(Error::DecoderInvalid),
            Self::Initial(t) => Ok(t),
        }
    }

    fn try_into_inner(self) -> Result<T> {
        match self {
            Self::Ok(z) => Ok(z.into_inner().into_inner()),
            Self::Invalid => Err(Error::DecoderInvalid),
            Self::Initial(t) => Ok(t),
        }
    }
}

/// # A `sd0` streamed file
///
/// ```
/// use assembly_pack::sd0::read::SegmentedDecoder;
/// use std::io::{Cursor, Read};
///
/// const BYTES: [u8; 30] = [
///     0x73, 0x64, 0x30, 0x01, 0xff, 0x15, 0x00, 0x00,
///     0x00, 0x78, 0xda, 0xf3, 0x48, 0xcd, 0xc9, 0xc9,
///     0x57, 0x08, 0xcf, 0x2f, 0xca, 0x49, 0x51, 0xe4,
///     0x02, 0x00, 0x20, 0x91, 0x04, 0x48,
/// ];
///
/// let mut decoder = SegmentedDecoder::new(Cursor::new(&BYTES)).unwrap();
/// let mut output = String::new();
/// decoder.read_to_string(&mut output).unwrap();
/// assert_eq!(output, "Hello World!\n");
/// ```
pub struct SegmentedDecoder<T> {
    inner: DecoderKind<T>,
}

impl<T> SegmentedDecoder<T> {
    /// Get the inner reader
    ///
    /// This panics when the value is invalid, which only happens when
    /// a read failed.
    pub fn into_inner(self) -> T {
        self.inner.try_into_inner().expect("decoder invalid")
    }

    /// Get a mutable reference to the inner stream
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.try_get_mut().expect("decoder invalid")
    }
}

impl<T: Read> SegmentedDecoder<T> {
    /// Create a new reader
    pub fn new(mut inner: T) -> Result<Self> {
        check_magic(&mut inner)?;
        Ok(Self {
            inner: DecoderKind::Initial(inner),
        })
    }
}

impl<T: Read> Read for SegmentedDecoder<T> {
    /// Read from the (decompressed) stream
    ///
    /// This may leave the
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let DecoderKind::Ok(z) = &mut self.inner {
            let len = z.read(buf)?;
            if len == 0 {
                let inner = self.inner.take().try_into_inner()?;
                self.inner = DecoderKind::Initial(inner);

                self.read(buf) // important recursive call!
            } else {
                Ok(len)
            }
        } else if let DecoderKind::Initial(mut inner) = self.inner.take() {
            if let Some(limit) = read_size(&mut inner)? {
                let take = inner.take(limit.into());
                self.inner = DecoderKind::Ok(ZlibDecoder::new(take));
                self.read(buf)
            } else {
                Ok(0) // EOF
            }
        } else {
            Err(Error::DecoderInvalid.into())
        }
    }
}

/// # A `sd0` streamed file
struct SegmentedDecoderRaw<T> {
    obj: BufReader<Take<T>>,
    data: Decompress,
}

impl<T: Read> SegmentedDecoderRaw<T> {
    fn read_zlib(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut d_out = 0;
        let mut c_out = self.data.total_out();
        let mut c_in = self.data.total_in();
        loop {
            let input = self.obj.fill_buf()?;
            let mut d_in = 0;
            let flush = if input.is_empty() {
                FlushDecompress::Finish
            } else {
                FlushDecompress::None
            };
            let status = self
                .data
                .decompress(input, &mut buf[d_out as usize..], flush)?;
            update(&mut c_out, &mut d_out, self.data.total_out());
            update(&mut c_in, &mut d_in, self.data.total_in());

            self.obj.consume(d_in as usize);

            match status {
                flate2::Status::Ok => {
                    if d_out as usize == buf.len() {
                        break;
                    }
                }
                flate2::Status::BufError => break,
                flate2::Status::StreamEnd => break,
            }
        }
        Ok(d_out as usize)
    }

    /// Create a new stream from a `Read` object, with buffer capacity
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize, inner: T) -> Result<Self> {
        Self::with_buf_reader(inner, |inner| BufReader::with_capacity(capacity, inner))
    }

    /// Create a new stream from a `Read` object
    #[allow(dead_code)]
    pub fn new(inner: T) -> Result<Self> {
        Self::with_buf_reader(inner, BufReader::new)
    }

    fn with_buf_reader<F>(mut inner: T, make_buf_reader: F) -> Result<Self>
    where
        F: FnOnce(Take<T>) -> BufReader<Take<T>>,
    {
        check_magic(&mut inner)?;
        if let Some(size) = read_size(&mut inner)? {
            Ok(Self {
                obj: make_buf_reader(inner.take(size.into())),
                data: Decompress::new(true),
            })
        } else {
            todo!()
        }
    }
}

#[inline]
fn update(cnt: &mut u64, diff: &mut u64, new: u64) {
    *diff += new - *cnt;
    *cnt = new;
}

impl<T> Read for SegmentedDecoderRaw<T>
where
    T: Read,
{
    #[allow(clippy::many_single_char_names)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.read_zlib(buf)?;
        if size < buf.len() {
            // Skip to the next chunk if present, otherwise just keep the last valid state
            if let Some(limit) = read_size(self.obj.get_mut().get_mut())? {
                self.obj.get_mut().set_limit(limit.into());
                self.data.reset(true);
                return self.read(&mut buf[size..]);
            }
        }
        Ok(size)
    }
}
