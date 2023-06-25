//! # Low level reader for PK files

use super::file::{PKEntry, PKEntryData, PKTrailer};
use super::parser;

use crate::common::{CRCTree, CRCTreeCollector, CRCTreeVisitor};
use crate::sd0::{self, read::SegmentedDecoder};
use nom::{Finish, IResult, Offset};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::{self, ErrorKind};
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::marker::{Send, Sync};
use std::ops::ControlFlow;

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
pub struct PackFile<T> {
    inner: T,
}

/// A low level read for a file
pub struct PackStreamReader<'b, T> {
    base_addr: u32,
    offset: u32,
    size: u32,
    file: &'b mut PackFile<T>,
}

trait Readable {
    type Buf: AsMut<[u8]> + AsRef<[u8]>;
    type Output: Sized;
    const NAME: &'static str;

    fn make() -> Self::Buf;
    fn parse(input: &[u8]) -> IResult<&[u8], Self::Output>;
}

struct MagicBytes;

macro_rules! readable_impl {
    ($ty:ty ; $parser:ident([u8;$size:literal]) -> $out:ty) => {
        impl Readable for $ty {
            type Buf = [u8; $size];
            type Output = $out;
            const NAME: &'static str = std::stringify!($ty);

            fn make() -> Self::Buf {
                [0; $size]
            }

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
readable_impl!(PKEntry; parse_pk_entry([u8;100]));

fn read_value<V: Readable, R: Read + Seek>(reader: &mut R, addr: u64) -> io::Result<V::Output> {
    let mut bytes: V::Buf = V::make();
    reader.read_exact(bytes.as_mut())?;
    let (_, value) = V::parse(bytes.as_ref()).finish().map_err(ParseError::map(
        V::NAME,
        addr,
        bytes.as_ref(),
    ))?;
    Ok(value)
}

impl<T> PackFile<T>
where
    T: Seek + BufRead,
{
    /// Open a file from a stream
    pub fn open(inner: T) -> Self {
        PackFile { inner }
    }

    /// Return the inner reader
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Get a reference to the inner reader
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner reader
    pub fn get_mut(&mut self) -> &T {
        &mut self.inner
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
    pub fn get_entry_accessor(mut self, addr: u32) -> io::Result<PackEntryAccessor<T>> {
        let mut count_bytes: [u8; 4] = [0; 4];
        self.inner.seek(SeekFrom::Start(u64::from(addr)))?;
        self.inner.read_exact(&mut count_bytes)?;
        let count = u32::from_le_bytes(count_bytes);
        Ok(PackEntryAccessor {
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
    pub fn get_file_stream<'b>(&'b mut self, entry: PKEntry) -> PackStreamReader<'b, T> {
        let base_addr = entry.file_data_addr;
        let size = match entry.is_compressed & 0xff {
            0 => entry.meta.raw,
            _ => entry.meta.compressed,
        }
        .size;
        //println!("{:?}", entry);
        PackStreamReader::<'b, T> {
            file: self,
            base_addr,
            offset: 0,
            size,
        }
    }

    /// Get some object with a read trait representing the data
    pub fn get_file_data(
        &mut self,
        entry: PKEntry,
    ) -> std::result::Result<PackDataStream<T>, sd0::read::Error> {
        let is_compr = (entry.is_compressed & 0xff) > 0;
        let file_stream = self.get_file_stream(entry);
        Ok(if is_compr {
            let compr_stream = SegmentedDecoder::new(file_stream)?;
            PackDataStream::Compressed(compr_stream)
        } else {
            PackDataStream::Plain(file_stream)
        })
    }
}

/// A stream that is either compressed or plain
pub enum PackDataStream<'b, T> {
    /// The stream is *not* sd0 compressed
    Plain(PackStreamReader<'b, T>),
    /// The stream *is* sd0 compressed
    Compressed(SegmentedDecoder<PackStreamReader<'b, T>>),
}

impl<'b, T: Seek + BufRead> std::io::Read for PackDataStream<'b, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Plain(inner) => inner.read(buf),
            Self::Compressed(inner) => inner.read(buf),
        }
    }
}

/// A low level random access to the entries
pub struct PackEntryAccessor<T> {
    base_addr: u32,
    count: u32,
    file: PackFile<T>,
}

impl<T> PackEntryAccessor<T> {
    /// Return the contained [PackFile]
    pub fn into_inner(self) -> PackFile<T> {
        self.file
    }

    /// Get a mutable reference to the underlying file
    pub fn get_mut(&mut self) -> &mut PackFile<T> {
        &mut self.file
    }

    /// Get a reference to the underlying file
    pub fn get_ref(&self) -> &PackFile<T> {
        &self.file
    }
}

impl<T> PackEntryAccessor<T>
where
    T: Seek + BufRead,
{
    /// Get the specified entry if inside of count
    pub fn get_entry(&mut self, index: i32) -> io::Result<Option<PKEntry>> {
        if index >= 0 {
            Ok(Some(
                self.file.get_entry(self.base_addr + (index as u32) * 100)?,
            ))
        } else {
            Ok(None)
        }
    }

    /// Get all the entries
    pub fn read_all(&mut self) -> io::Result<CRCTree<PKEntryData>> {
        let mut collector = CRCTreeCollector::new();
        self.visit(&mut collector)?;
        Ok(collector.into_inner())
    }

    /// Implements a visitor pattern
    ///
    /// This [CRCTreeVisitor::visit] function is called once for every node in the tree
    /// in tree order.
    pub fn visit<V>(&mut self, visitor: &mut V) -> io::Result<ControlFlow<V::Break>>
    where
        V: CRCTreeVisitor<PKEntryData>,
    {
        let parent = self.get_root_entry()?;
        self.visit_recursive(visitor, parent)
    }

    /// Implements a visitor pattern
    fn visit_recursive<V>(
        &mut self,
        visitor: &mut V,
        parent: Option<PKEntry>,
    ) -> io::Result<ControlFlow<V::Break>>
    where
        V: CRCTreeVisitor<PKEntryData>,
    {
        let data = if let Some(entry) = parent {
            entry
        } else {
            return Ok(ControlFlow::Continue(()));
        };
        let left = self.get_entry(data.left)?;
        if let ControlFlow::Break(e) = self.visit_recursive(visitor, left)? {
            return Ok(ControlFlow::Break(e));
        }
        if let ControlFlow::Break(e) = visitor.visit(data.crc, data.data) {
            return Ok(ControlFlow::Break(e));
        }
        let right = self.get_entry(data.right)?;
        if let ControlFlow::Break(e) = self.visit_recursive(visitor, right)? {
            return Ok(ControlFlow::Break(e));
        }
        Ok(ControlFlow::Continue(()))
    }

    /// Get the root entrys if not empty
    pub fn get_root_entry(&mut self) -> io::Result<Option<PKEntry>> {
        self.get_entry((self.count / 2) as i32)
    }

    fn find_entry_recursive(
        &mut self,
        parent: Option<PKEntry>,
        crc: u32,
    ) -> io::Result<Option<PKEntry>> {
        let data = match parent {
            Some(x) => x,
            None => return Ok(None),
        };
        match data.crc.cmp(&crc) {
            Ordering::Less => {
                let right = self.get_entry(data.right)?;
                self.find_entry_recursive(right, crc)
            }
            Ordering::Greater => {
                let left = self.get_entry(data.left)?;
                self.find_entry_recursive(left, crc)
            }
            Ordering::Equal => Ok(Some(data)),
        }
    }

    /// Find an entry given a CRC
    pub fn find_entry(&mut self, crc: u32) -> io::Result<Option<PKEntry>> {
        let root = self.get_root_entry()?;
        self.find_entry_recursive(root, crc)
    }

    /// Return the number of entries
    pub fn get_count(&self) -> u32 {
        self.count
    }
}

fn other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn Error + Send + Sync>>,
{
    io::Error::new(ErrorKind::Other, e)
}

impl<'b, T> Read for PackStreamReader<'b, T>
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
