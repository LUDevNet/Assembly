//! Low-Level reader for FDB files
//!
//! This module provides a struct which can be used to access
//! a FDB file in any order the user desires.

pub mod builder;

use std::io::{self, BufRead, Read, Seek, SeekFrom};

use super::{
    file::{
        //lists::{FDBBucketHeaderList, FDBColumnHeaderList, FDBFieldDataList, FDBTableHeaderList},
        FDBHeader,
        FDBRowHeader,
        FDBRowHeaderListEntry,
        FDBTableDataHeader,
        FDBTableDefHeader,
    },
    parser::{ParseFDB, ParseLE},
};

use assembly_core::reader::{FileError, FileResult};
use assembly_fdb_core::file::{FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBTableHeader};
use latin1str::Latin1String;
use nom::{Finish, IResult};

/// Implementation of [`DatabaseReader::get_row_header_addr_iterator`]
#[allow(clippy::upper_case_acronyms)]
pub struct FDBRowHeaderAddrIterator<'a, T: ?Sized> {
    next_addr: u32,
    file: &'a mut T,
}

/// Extension trait to `Seek + BufRead` for reading strings
pub trait DatabaseBufReader
where
    Self: Seek + BufRead,
{
    /// Read a string from the file
    fn get_string(&mut self, addr: u32) -> io::Result<String>;
}

impl<T> DatabaseBufReader for T
where
    T: Seek + BufRead,
{
    fn get_string(&mut self, addr: u32) -> io::Result<String> {
        self.seek(SeekFrom::Start(addr.into()))?;
        let string = Latin1String::read_cstring(self)?;
        Ok(string.decode().into_owned())
    }
}
impl<T> DatabaseReader for T where T: Seek + Read + ?Sized {}

/// Parse a struct at the give offset into the buffer
fn parse_at<R: Seek + Read + ?Sized, T>(
    reader: &mut R,
    addr: impl Into<u64>,
    buf: &mut [u8],
    parser: impl Fn(&[u8]) -> IResult<&[u8], T>,
) -> FileResult<T> {
    let addr = addr.into();
    reader.seek(SeekFrom::Start(addr))?;
    reader.read_exact(buf)?;
    let (_rest, header) = parser(buf).finish().map_err(|e| FileError::Parse {
        addr,
        offset: buf.len() - e.input.len(),
        code: e.code,
    })?;
    Ok(header)
}

fn bytes<IO: ParseLE>() -> IO::Buf {
    IO::Buf::default()
}

fn parse_list_at<R: Seek + Read + ?Sized, T: ParseFDB>(
    reader: &mut R,
    addr: impl Into<u64>,
    count: u32,
) -> FileResult<Vec<T>> {
    let addr = addr.into();
    reader.seek(SeekFrom::Start(addr))?;
    let buf_len = <T::IO as ParseLE>::BYTE_COUNT;
    let mut buf = bytes::<T::IO>();
    let mut offset = 0;
    let mut list = Vec::with_capacity(count as usize);
    for _ in 0..count {
        reader.read_exact(buf.as_mut())?;
        let (_rest, t) = T::parse(buf.as_mut())
            .finish()
            .map_err(|e| FileError::Parse {
                addr,
                offset: offset + buf_len - e.input.len(),
                code: e.code,
            })?;
        list.push(t);
        offset += buf_len;
    }
    Ok(list)
}
/// Extension to `Seek + Read` to read an FDB file
pub trait DatabaseReader
where
    Self: Seek + Read,
{
    /// Read the schema header
    fn get_header(&mut self) -> FileResult<FDBHeader> {
        let mut bytes = [0; 8];
        parse_at(self, 0u64, &mut bytes, FDBHeader::parse)
    }

    /// Read the table header
    fn get_table_header_list(&mut self, header: FDBHeader) -> FileResult<Vec<FDBTableHeader>> {
        let addr = header.tables.base_offset;
        let count = header.tables.count;
        parse_list_at(self, addr, count)
    }

    /// Read the table def header
    fn get_table_def_header(&mut self, addr: u32) -> FileResult<FDBTableDefHeader> {
        let mut bytes = [0; std::mem::size_of::<FDBTableDefHeader>()];
        parse_at(self, addr, &mut bytes, FDBTableDefHeader::parse)
    }

    /// Get a 64bit integer
    fn get_i64(&mut self, addr: u32) -> io::Result<i64> {
        let mut bytes: [u8; 8] = [0; 8];
        self.seek(SeekFrom::Start(addr.into()))?;
        self.read_exact(&mut bytes)?;
        Ok(i64::from_le_bytes(bytes))
    }

    /// Get the column header list
    fn get_column_header_list(
        &mut self,
        header: &FDBTableDefHeader,
    ) -> FileResult<Vec<FDBColumnHeader>> {
        parse_list_at(self, header.column_header_list_addr, header.column_count)
    }

    /// Get the table data header
    fn get_table_data_header(&mut self, addr: u32) -> FileResult<FDBTableDataHeader> {
        let mut bytes = bytes::<<FDBTableDataHeader as ParseFDB>::IO>();
        parse_at(self, addr, &mut bytes, FDBTableDataHeader::parse)
    }

    /// Get the table bucket header list
    fn get_bucket_header_list(
        &mut self,
        header: &FDBTableDataHeader,
    ) -> FileResult<Vec<FDBBucketHeader>> {
        let addr = header.buckets.base_offset;
        let count = header.buckets.count;
        parse_list_at(self, addr, count)
    }

    /// Get a row header list entry
    fn get_row_header_list_entry(&mut self, addr: u32) -> FileResult<FDBRowHeaderListEntry> {
        let mut bytes = [0; std::mem::size_of::<FDBRowHeaderListEntry>()];
        parse_at(self, addr, &mut bytes, FDBRowHeaderListEntry::parse)
    }

    /// Get a row header
    fn get_row_header(&mut self, addr: u32) -> FileResult<FDBRowHeader> {
        let mut bytes: [u8; 8] = [0; std::mem::size_of::<FDBRowHeader>()];
        parse_at(self, addr, &mut bytes, FDBRowHeader::parse)
    }

    /// Returns a vector of `FDBFieldData`
    fn get_field_data_list(&mut self, header: FDBRowHeader) -> FileResult<Vec<FDBFieldData>> {
        parse_list_at(self, header.fields.base_offset, header.fields.count)
    }

    /// Returns an iterator over `FDBRowHeader` offsets
    fn get_row_header_addr_iterator<'a>(
        &'a mut self,
        addr: u32,
    ) -> FDBRowHeaderAddrIterator<'a, Self> {
        FDBRowHeaderAddrIterator::<'a> {
            file: self,
            next_addr: addr,
        }
    }
}

impl<'a, T> Iterator for FDBRowHeaderAddrIterator<'a, T>
where
    T: Read + Seek,
{
    type Item = FileResult<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_addr {
            std::u32::MAX => None,
            addr => match self.file.get_row_header_list_entry(addr) {
                Ok(entry) => {
                    self.next_addr = entry.row_header_list_next_addr;
                    Some(Ok(entry.row_header_addr))
                }
                Err(e) => {
                    self.next_addr = std::u32::MAX;
                    Some(Err(e))
                }
            },
        }
    }
}
