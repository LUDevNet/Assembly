//! # Low level reader for PK files

use super::file::{PKHeader, PKEntry};
use super::parser;
use crate::core::reader::{FileResult, FileError};
use crate::sd0::stream::{SegmentedStream, SegmentedError};
use std::io::{Read, Seek, BufRead, SeekFrom};
use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::convert::TryFrom;
use std::error::Error;
use std::marker::{Send,Sync};
use std::fmt::{Display, Formatter, Error as FmtError};

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

#[derive(Debug)]
pub enum StreamError {
    Segmented(SegmentedError),
}

impl<'a,T> PackFile<'a,T>
where T: Seek + BufRead {
    /// Open a file from a stream
    pub fn open<'b: 'a>(inner: &'b mut T) -> Self {
        PackFile{inner}
    }

    /// Check for the magic bytes at the beginning of the file
    pub fn check_magic(&mut self) -> FileResult<()> {
        let mut magic_bytes: [u8;4] = [0;4];
        self.inner.seek(SeekFrom::Start(0)).map_err(FileError::Seek)?;
        self.inner.read_exact(&mut magic_bytes).map_err(FileError::Read)?;
        let (_rest, _magic) = parser::parse_pk_magic(&magic_bytes)?;
        Ok(())
    }

    /// Load the header from the end of the file
    pub fn get_header(&mut self) -> FileResult<PKHeader> {
        let mut header_bytes: [u8;8] = [0;8];
        self.inner.seek(SeekFrom::End(-8)).map_err(FileError::Seek)?;
        self.inner.read_exact(&mut header_bytes).map_err(FileError::Read)?;
        let (_rest, header) = parser::parse_pk_header(&header_bytes)?;
        Ok(header)
    }

    /// Load the header from the end of the file
    pub fn get_entry(&mut self, addr: u32) -> FileResult<PKEntry> {
        let mut entry_bytes: [u8;100] = [0;100];
        self.inner.seek(SeekFrom::Start(u64::from(addr))).map_err(FileError::Seek)?;
        self.inner.read_exact(&mut entry_bytes).map_err(FileError::Read)?;
        let (_rest, entry) = parser::parse_pk_entry(&entry_bytes)?;
        Ok(entry)
    }

    /// Get an random access wrapper for the entries
    pub fn get_entry_accessor<'b>(&'b mut self, addr: u32) -> FileResult<PackEntryAccessor<'b,'a, T>> {
        let mut count_bytes: [u8;4] = [0; 4];
        self.inner.seek(SeekFrom::Start(u64::from(addr))).map_err(FileError::Seek)?;
        self.inner.read_exact(&mut count_bytes).map_err(FileError::Read)?;
        let count = u32::from_le_bytes(count_bytes);
        Ok(PackEntryAccessor::<'b,'a>{base_addr: addr + 4, count, file: self})
    }

    /// Get a list of all entries
    pub fn get_entry_list(&mut self, addr: u32) -> FileResult<Vec<PKEntry>> {
        let mut bytes: Vec<u8> = Vec::new();
        self.inner.seek(SeekFrom::Start(u64::from(addr))).map_err(FileError::Seek)?;
        self.inner.read_to_end(&mut bytes).map_err(FileError::Read)?;
        let (_rest, entry_list) = parser::parse_pk_entry_list(&bytes)?;
        Ok(entry_list)
    }

    /// Get a boxed reader for the file stream
    pub fn get_file_stream<'b>(&'b mut self, entry: PKEntry) -> PackStreamReader<'b,'a,T> {
        let base_addr = entry.file_data_addr;
        let size = if entry.is_compressed[0] == 0 {
            entry.orig_file_size
        } else {
            entry.compr_file_size
            //entry.orig_file_size
        };
        //println!("{:?}", entry);
        PackStreamReader::<'b,'a,T>{file: self, base_addr, offset: 0, size}
    }

    /// Get some object with a read trait representing the data
    pub fn get_file_data<'c, 'b: 'c>(&'b mut self, entry: PKEntry) -> Result<Box<Read + 'c>, StreamError> {
        let is_compr = entry.is_compressed[0] > 0;
        let file_stream = self.get_file_stream(entry);
        Ok(if is_compr {
            let compr_stream = SegmentedStream::try_from(file_stream).map_err(StreamError::Segmented)?;
            Box::new(compr_stream)
        } else {
            Box::new(file_stream)
        })
    }
}

impl<'b, 'a, T> PackEntryAccessor<'b, 'a, T>
where T: Seek + BufRead {

    /// Get a reference to the underlying file
    pub fn get_file_mut(&'b mut self) -> &'b mut PackFile<'a, T> {
        self.file
    }

    /// Get the specified entry if inside of count
    pub fn get_entry(&mut self, index: u32) -> Option<FileResult<PKEntry>> {
        if index <= self.count {
            Some(self.file.get_entry(self.base_addr + index * 100))
        } else {
            None
        }
    }

    /// Get the root entrys if not empty
    pub fn get_root_entry(&mut self) -> Option<FileResult<PKEntry>> {
        self.get_entry(self.count / 2)
    }

}

fn other_io_err<'a, E>(e: E) -> IoError
where E: Into<Box<dyn Error + Send + Sync>>
{
    IoError::new(ErrorKind::Other, e)
}

impl<'b, 'a, T> Read for PackStreamReader<'b, 'a, T>
where T: Seek + BufRead {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
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
        }.and_then(|n| {
            //println!("P-RES: {}", n);
            self.offset += u32::try_from(n).map_err(other_io_err)?; Ok(n)})
    }
}

#[derive(Debug)]
pub enum SeekError {
    Negative(i64),
    OutOfBounds(u64, u64),
}

impl Display for SeekError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            SeekError::Negative(n) => write!(f, "{} < 0", n),
            SeekError::OutOfBounds(o, s) => write!(f, "{} > {}", o, s),
        }
    }
}

impl Error for SeekError {

}

impl<'b, 'a, T> PackStreamReader<'b, 'a, T>
where T: Seek + BufRead {

    fn seek_to_pos(&mut self, n: u64) -> IoResult<u64> {
        if n > self.size.into() {
            self.offset = self.size;
            let e = SeekError::OutOfBounds(n, self.size.into());
            Err(IoError::new(ErrorKind::Other, e))
        } else {
            self.offset = u32::try_from(n).map_err(|e| IoError::new(ErrorKind::Other, e))?;
            Ok(n)
        }
    }

    fn seek_to(&mut self, n: i64) -> IoResult<u64> {
        if n < 0 {
            self.offset = self.size;
            let e = SeekError::Negative(n);
            Err(IoError::new(ErrorKind::Other, e))
        } else {
            self.seek_to_pos(u64::try_from(n).map_err(|e| IoError::new(ErrorKind::Other, e))?)
        }
    }
}

impl<'b, 'a, T> Seek for PackStreamReader<'b, 'a, T>
where T: Seek + BufRead {
    fn seek(&mut self, to: SeekFrom) -> IoResult<u64> {
        match to {
            SeekFrom::Start(n) => self.seek_to_pos(n),
            SeekFrom::Current(n) => self.seek_to(i64::from(self.offset) + n),
            SeekFrom::End(n) => self.seek_to(i64::from(self.size) + n),
        }
    }
}
