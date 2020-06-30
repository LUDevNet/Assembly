use super::slice::Latin1Str;
use crate::fdb::file::{
    FDBHeader, FDBRowHeader, FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader,
};
use std::{
    convert::TryInto,
    fmt,
    ops::{Deref, Range},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufferError {
    //#[error("index out of bounds {0}")]
    //OutOfBounds(#[from] io::Error),
    #[error("index out of bounds {}..{}", .0.start, .0.end)]
    OutOfBounds(Range<usize>),
}

#[derive(Copy, Clone)]
pub struct Buffer<'a>(&'a [u8]);

pub type Res<T> = Result<T, BufferError>;

impl<'a> Deref for Buffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Buffer<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self(buf)
    }

    pub fn get_len_at(self, start: usize, len: usize) -> Res<&'a [u8]> {
        let end = start + len;
        self.0
            .get(Range { start, end })
            .ok_or_else(|| BufferError::OutOfBounds(Range { start, end }))
    }

    pub fn string(self, addr: u32) -> Res<&'a Latin1Str> {
        let start = addr as usize;
        let buf = self.0.get(start..).ok_or_else(|| {
            let end = self.0.len();
            BufferError::OutOfBounds(Range { start, end })
        })?;
        Ok(Latin1Str::new(buf))
    }

    pub fn header(self, addr: u32) -> Res<FDBHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBHeader {
            table_count: u32::from_le_bytes(a.try_into().unwrap()),
            table_header_list_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }

    pub fn table_def_header(&self, addr: u32) -> Res<FDBTableDefHeader> {
        let buf = self.get_len_at(addr as usize, 12)?;
        let (a, buf) = buf.split_at(4);
        let (b, c) = buf.split_at(4);
        Ok(FDBTableDefHeader {
            column_count: u32::from_le_bytes(a.try_into().unwrap()),
            table_name_addr: u32::from_le_bytes(b.try_into().unwrap()),
            column_header_list_addr: u32::from_le_bytes(c.try_into().unwrap()),
        })
    }

    pub fn table_data_header(self, addr: u32) -> Res<FDBTableDataHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBTableDataHeader {
            bucket_count: u32::from_le_bytes(a.try_into().unwrap()),
            bucket_header_list_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }

    pub fn row_header_list_entry(self, addr: u32) -> Res<FDBRowHeaderListEntry> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBRowHeaderListEntry {
            row_header_addr: u32::from_le_bytes(a.try_into().unwrap()),
            row_header_list_next_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }

    pub fn row_header(self, addr: u32) -> Res<FDBRowHeader> {
        let buf = self.get_len_at(addr as usize, 8)?;
        let (a, b) = buf.split_at(4);
        Ok(FDBRowHeader {
            field_count: u32::from_le_bytes(a.try_into().unwrap()),
            field_data_list_addr: u32::from_le_bytes(b.try_into().unwrap()),
        })
    }
}

impl<'a> fmt::Debug for Buffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("base", &self.0.as_ptr())
            .field("len", &self.0.len())
            .finish()
    }
}
