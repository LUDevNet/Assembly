//! Low-Level reader for FDB files
//!
//! This module provides a struct which can be used to access
//! a FDB file in any order the user desires.

use std::io::{BufRead, Read, Seek, SeekFrom};

use super::file::*;
use super::parser;

use assembly_core::reader::{FileError, FileResult, ParseError};
use encoding_rs::WINDOWS_1252;

pub struct FDBRowHeaderAddrIterator<'a, T: ?Sized> {
    next_addr: u32,
    file: &'a mut T,
}

pub trait DatabaseBufReader
where
    Self: Seek + BufRead,
{
    /// Read a string from the file
    fn get_string(&mut self, addr: u32) -> FileResult<String>;
}

impl<T> DatabaseBufReader for T
where
    T: Seek + BufRead,
{
    fn get_string(&mut self, addr: u32) -> FileResult<String> {
        let mut string: Vec<u8> = Vec::new();
        self.seek(SeekFrom::Start(addr.into()))?;
        self.read_until(0x00, &mut string)?;
        if string.ends_with(&[0x00]) {
            string.pop();
        }
        let (decoded, _, _) = WINDOWS_1252.decode(&string);
        Ok(decoded.into_owned())
    }
}
impl<T> DatabaseReader for T where T: Seek + Read + ?Sized {}

pub trait DatabaseReader
where
    Self: Seek + Read,
{
    /// Read the schema header
    fn get_header(&mut self) -> FileResult<FDBHeader> {
        let mut bytes = [0; FDBHeader::BYTE_COUNT];
        self.seek(SeekFrom::Start(0))?;
        self.read_exact(&mut bytes)?;
        let (_rest, header) = parser::parse_header(&bytes[..]).map_err(ParseError::from)?;
        Ok(header)
    }

    /// Read the table header
    fn get_table_header_list(&mut self, header: FDBHeader) -> FileResult<FDBTableHeaderList> {
        let mut table_headers_bytes: Vec<u8> = Vec::new();
        let byte_count: u64 = header.table_headers_byte_count() as u64;
        let list_addr: u64 = header.table_header_list_addr.into();
        let count = header.table_count as usize;
        self.seek(SeekFrom::Start(list_addr))
            .map_err(FileError::Seek)?;
        self.take(byte_count)
            .read_to_end(&mut table_headers_bytes)
            .map_err(FileError::Read)?;
        let (_rest, table_header_list) =
            parser::parse_table_headers(&table_headers_bytes, count).map_err(FileError::from)?;
        Ok(table_header_list)
    }

    /// Read the table def header
    fn get_table_def_header(&mut self, addr: u32) -> FileResult<FDBTableDefHeader> {
        let mut table_def_header_bytes = [0; FDBTableDefHeader::BYTE_COUNT];
        self.seek(SeekFrom::Start(addr.into()))
            .map_err(FileError::Seek)?;
        self.read_exact(&mut table_def_header_bytes)
            .map_err(FileError::Read)?;
        let (_rest, def_header) =
            parser::parse_table_def_header(&table_def_header_bytes).map_err(FileError::from)?;
        Ok(def_header)
    }

    /// Get a 64bit integer
    fn get_i64(&mut self, addr: u32) -> FileResult<i64> {
        let mut bytes: [u8; 8] = [0; 8];
        self.seek(SeekFrom::Start(addr.into()))
            .map_err(FileError::Seek)?;
        self.read_exact(&mut bytes).map_err(FileError::Read)?;
        Ok(i64::from_le_bytes(bytes))
    }

    /// Get the column header list
    fn get_column_header_list<'b>(
        &'b mut self,
        header: &FDBTableDefHeader,
    ) -> FileResult<FDBColumnHeaderList> {
        let off: u64 = header.column_header_list_addr.into();
        let byte_count = header.column_header_list_byte_count();

        let mut column_header_list_bytes: Vec<u8> = vec![0; byte_count];
        let count = header.column_count as usize;

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(column_header_list_bytes.as_mut_slice())
            .map_err(FileError::Read)?;

        let (_rest, column_header_list) =
            parser::parse_column_header_list(&column_header_list_bytes, count)
                .map_err(FileError::from)?;

        Ok(column_header_list)
    }

    /// Get the table data header
    fn get_table_data_header(&mut self, addr: u32) -> FileResult<FDBTableDataHeader> {
        let off: u64 = addr.into();
        let mut table_data_header_bytes = [0; FDBTableDataHeader::BYTE_COUNT];

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(&mut table_data_header_bytes)
            .map_err(FileError::Read)?;

        let (_rest, table_data_header) =
            parser::parse_table_data_header(&table_data_header_bytes).map_err(FileError::from)?;

        Ok(table_data_header)
    }

    /// Get the table bucket header list
    fn get_bucket_header_list(
        &mut self,
        header: &FDBTableDataHeader,
    ) -> FileResult<FDBBucketHeaderList> {
        let off: u64 = header.bucket_header_list_addr.into();
        let count = header.bucket_count as usize;
        let byte_count = header.bucket_header_list_byte_count();

        let mut bucket_header_list_bytes: Vec<u8> = vec![0; byte_count];

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(bucket_header_list_bytes.as_mut_slice())
            .map_err(FileError::Read)?;

        let (_rest, bucket_header_list) =
            parser::parse_bucket_header_list(&bucket_header_list_bytes, count)
                .map_err(FileError::from)?;
        Ok(bucket_header_list)
    }

    /// Get a row header list entry
    fn get_row_header_list_entry(&mut self, addr: u32) -> FileResult<FDBRowHeaderListEntry> {
        let off = u64::from(addr);
        let mut row_header_list_entry_bytes = [0; FDBRowHeaderListEntry::BYTE_COUNT];

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(&mut row_header_list_entry_bytes)
            .map_err(FileError::Read)?;

        let (_rest, row_header_list_entry) =
            parser::parse_row_header_list_entry(&row_header_list_entry_bytes)
                .map_err(FileError::from)?;
        Ok(row_header_list_entry)
    }

    /// Get a row header
    fn get_row_header(&mut self, addr: u32) -> FileResult<FDBRowHeader> {
        let off = u64::from(addr);
        let mut row_header_bytes: [u8; 8] = [0; FDBRowHeader::BYTE_COUNT];

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(&mut row_header_bytes)
            .map_err(FileError::Read)?;

        let (_rest, row_header) =
            parser::parse_row_header(&row_header_bytes).map_err(FileError::from)?;
        Ok(row_header)
    }

    fn get_field_data_list(&mut self, header: FDBRowHeader) -> FileResult<FDBFieldDataList> {
        let off: u64 = header.field_data_list_addr.into();
        let byte_count = header.field_data_list_byte_count();
        let count = header.field_count as usize;

        let mut field_data_list_bytes: Vec<u8> = vec![0; byte_count];

        self.seek(SeekFrom::Start(off)).map_err(FileError::Seek)?;
        self.read_exact(field_data_list_bytes.as_mut_slice())
            .map_err(FileError::Read)?;

        let (_rest, field_data_list) = parser::parse_field_data_list(&field_data_list_bytes, count)
            .map_err(FileError::from)?;
        Ok(field_data_list)
    }

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
