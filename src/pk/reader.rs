//! # Low level reader for PK files

use super::file::{PKHeader, PKEntry};
use super::parser;
use crate::core::reader::{FileResult, FileError};
use std::io::{Seek, BufRead, SeekFrom};

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
        println!("{}", count);
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
}

impl<'b, 'a, T> PackEntryAccessor<'b, 'a, T>
where T: Seek + BufRead {

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
