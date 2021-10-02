//! # Low level reader for PK files

use super::file::{PKEntry, PKTrailer};
use super::parser;

use crate::sd0;
use crate::sd0::read::SegmentedDecoder;
use nom::{Finish, IResult, Offset};

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::{self, ErrorKind};
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::marker::{Send, Sync};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Failure when parsing
pub struct ParseError {
    /// The structure that failed to parse
    structure: &'static str,
    /// Address of the error
    addr: u64,
    /// How far the parser got beyond addr
    offset: usize,
    /// The nom error kind
    code: nom::error::ErrorKind,
}

impl ParseError {
    fn map<'r>(
        structure: &'static str,
        addr: u64,
        slice: &'r [u8],
    ) -> impl FnOnce(nom::error::Error<&'r [u8]>) -> Self {
        move |e: nom::error::Error<&'r [u8]>| ParseError {
            structure,
            addr,
            offset: slice.offset(e.input),
            code: e.code,
        }
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse {} at {} (+{}) with code {:?}",
            self.structure, self.addr, self.offset, self.code
        )
    }
}

impl From<ParseError> for io::Error {
    fn from(error: ParseError) -> Self {
        io::Error::new(io::ErrorKind::Other, error)
    }
}

/// A low level pack file reader
pub struct PackFile<'a, T> {
    inner: &'a mut T,
}

/// A low level random access to the entries
pub struct PackEntryAccessor<'b, 'a, T> {
    base_addr: u32,
    count: u32,
    file: &'b mut PackFile<'a, T>,
}

/// A low level read for a file
pub struct PackStreamReader<'b, 'a, T> {
    base_addr: u32,
    offset: u32,
    size: u32,
    file: &'b mut PackFile<'a, T>,
}

trait Readable {
    type Buf: Default + AsMut<[u8]> + AsRef<[u8]>;
    type Output: Sized;
    const NAME: &'static str;

    fn parse(input: &[u8]) -> IResult<&[u8], Self::Output>;
}

struct MagicBytes;

macro_rules! readable_impl {
    ($ty:ty ; $parser:ident([u8;$size:literal]) -> $out:ty) => {
        impl Readable for $ty {
            type Buf = [u8; $size];
            type Output = $out;
            const NAME: &'static str = std::stringify!($ty);

            fn parse(input: &[u8]) -> IResult<&[u8], Self::Output> {
                parser::$parser(input)
            }
        }
    };
    ($ty:ty ; $parser:ident([u8;$size:literal])) => {
        readable_impl!($ty; $parser([u8;$size]) -> $ty);
    }
}

readable_impl!(MagicBytes; parse_pk_magic([u8;4]) -> ());
readable_impl!(PKTrailer; parse_pk_trailer([u8;8]));
readable_impl!(PKEntry; parse_pk_entry([u8;8]));

fn read_value<V: Readable, R: Read + Seek>(reader: &mut R, addr: u64) -> io::Result<V::Output> {
    let mut bytes: V::Buf = Default::default();
    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(bytes.as_mut())?;
    let (_, value) = V::parse(bytes.as_ref()).finish().map_err(ParseError::map(
        V::NAME,
        addr,
        bytes.as_ref(),
    ))?;
    Ok(value)
}

impl<'a, T> PackFile<'a, T>
where
    T: Seek + BufRead,
{
    /// Open a file from a stream
    pub fn open<'b: 'a>(inner: &'b mut T) -> Self {
        PackFile { inner }
    }

    /// Check for the magic bytes at the beginning of the file
    pub fn check_magic(&mut self) -> io::Result<()> {
        self.inner.seek(SeekFrom::Start(0))?;
        read_value::<MagicBytes, _>(&mut self.inner, 0)
    }

    /// Load the header from the end of the file
    pub fn get_header(&mut self) -> io::Result<PKTrailer> {
        let addr = self.inner.seek(SeekFrom::End(-8))?;
        read_value::<PKTrailer, _>(&mut self.inner, addr)
    }

    /// Load the header from the end of the file
    pub fn get_entry(&mut self, addr: u32) -> io::Result<PKEntry> {
        let addr = u64::from(addr);
        self.inner.seek(SeekFrom::Start(addr))?;
        read_value::<PKEntry, _>(&mut self.inner, addr)
    }

    /// Get an random access wrapper for the entries
    pub fn get_entry_accessor<'b>(
        &'b mut self,
        addr: u32,
    ) -> io::Result<PackEntryAccessor<'b, 'a, T>> {
        let mut count_bytes: [u8; 4] = [0; 4];
        self.inner.seek(SeekFrom::Start(u64::from(addr)))?;
        self.inner.read_exact(&mut count_bytes)?;
        let count = u32::from_le_bytes(count_bytes);
        Ok(PackEntryAccessor::<'b, 'a> {
            base_addr: addr + 4,
            count,
            file: self,
        })
    }

    /// Get a list of all entries
    pub fn get_entry_list(&mut self, addr: u32) -> io::Result<Vec<PKEntry>> {
        let mut bytes: Vec<u8> = Vec::new();
        let addr = self.inner.seek(SeekFrom::Start(u64::from(addr)))?;
        self.inner.read_to_end(&mut bytes)?;
        let (_rest, entry_list) = parser::parse_pk_entry_list(&bytes)
            .finish()
            .map_err(ParseError::map("Vec<PKEntry>", addr, &bytes))?;
        Ok(entry_list)
    }

    /// Get a boxed reader for the file stream
    pub fn get_file_stream<'b>(&'b mut self, entry: PKEntry) -> PackStreamReader<'b, 'a, T> {
        let base_addr = entry.file_data_addr;
        let size = if (entry.is_compressed & 0xff) == 0 {
            entry.orig_file_size
        } else {
            entry.compr_file_size
            //entry.orig_file_size
        };
        //println!("{:?}", entry);
        PackStreamReader::<'b, 'a, T> {
            file: self,
            base_addr,
            offset: 0,
            size,
        }
    }

    /// Get some object with a read trait representing the data
    pub fn get_file_data<'c, 'b: 'c>(
        &'b mut self,
        entry: PKEntry,
    ) -> std::result::Result<Box<dyn Read + 'c>, sd0::read::Error> {
        let is_compr = (entry.is_compressed & 0xff) > 0;
        let file_stream = self.get_file_stream(entry);
        Ok(if is_compr {
            let compr_stream = SegmentedDecoder::new(file_stream)?;
            Box::new(compr_stream)
        } else {
            Box::new(file_stream)
        })
    }
}

impl<'b, 'a, T> PackEntryAccessor<'b, 'a, T>
where
    T: Seek + BufRead,
{
    /// Get a reference to the underlying file
    pub fn get_file_mut(&'b mut self) -> &'b mut PackFile<'a, T> {
        self.file
    }

    /// Get the specified entry if inside of count
    pub fn get_entry(&mut self, index: i32) -> Option<io::Result<PKEntry>> {
        if index >= 0 {
            Some(self.file.get_entry(self.base_addr + (index as u32) * 100))
        } else {
            None
        }
    }

    /// Get the root entrys if not empty
    pub fn get_root_entry(&mut self) -> Option<io::Result<PKEntry>> {
        self.get_entry((self.count / 2) as i32)
    }
}

fn other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn Error + Send + Sync>>,
{
    io::Error::new(ErrorKind::Other, e)
}

impl<'b, 'a, T> Read for PackStreamReader<'b, 'a, T>
where
    T: Seek + BufRead,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = u64::from(self.base_addr + self.offset);
        self.file.inner.seek(SeekFrom::Start(pos))?;
        let buf_len = buf.len();
        //println!("P-BUF: {}", buf_len);
        let offset = usize::try_from(self.offset).map_err(other_io_err)?;
        //println!("P-OFF: {}", offset);
        let size = usize::try_from(self.size).map_err(other_io_err)?;
        //println!("P-SIZ: {}", size);
        if offset + buf_len > size {
            let max = size - offset;
            //println!("P-MAX: {}", max);
            self.file.inner.read(&mut buf[..max])
        } else {
            self.file.inner.read(buf)
        }
        .and_then(|n| {
            //println!("P-RES: {}", n);
            self.offset += u32::try_from(n).map_err(other_io_err)?;
            Ok(n)
        })
    }
}
