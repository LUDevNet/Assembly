use super::{
    buffer::{Buffer, BufferError},
    slice::{
        FDBBucketHeaderSlice, FDBColumnHeaderSlice, FDBFieldDataSlice, FDBTableHeaderSlice,
        Latin1Str,
    },
};
use crate::fdb::{
    core::ValueType,
    file::{
        FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBHeader, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
};
use std::borrow::Cow;

#[derive(Copy, Clone, Debug)]
pub struct Handle<'a, T> {
    buffer: Buffer<'a>,
    raw: T,
}
pub type Res<'a, T> = Result<Handle<'a, T>, BufferError>;

impl<'a, T> Handle<'a, T> {
    pub fn raw(&self) -> &T {
        &self.raw
    }

    fn wrap<R>(&self, raw: R) -> Handle<'a, R> {
        Handle {
            buffer: self.buffer,
            raw,
        }
    }
}

impl<'a> Handle<'a, ()> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: Buffer::new(buffer),
            raw: (),
        }
    }

    pub fn header(&self) -> Res<'a, FDBHeader> {
        let header = self.buffer.header(0)?;
        Ok(self.wrap(header))
    }
}

impl<'a> Handle<'a, FDBHeader> {
    pub fn table_count(&self) -> u32 {
        self.raw.table_count
    }

    pub fn table_header_list(&self) -> Res<'a, FDBTableHeaderSlice<'a>> {
        let len = self.table_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.table_header_list_addr as usize, len)?;
        Ok(self.wrap(FDBTableHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;
    type IntoIter = Handle<'a, FDBTableHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    type Item = Handle<'a, FDBTableHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBTableHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBTableHeader> {
    pub fn table_def_header(&self) -> Res<'a, FDBTableDefHeader> {
        let raw = self
            .buffer
            .table_def_header(self.raw.table_def_header_addr)?;
        Ok(self.wrap(raw))
    }

    pub fn table_data_header(&self) -> Res<'a, FDBTableDataHeader> {
        let raw = self
            .buffer
            .table_data_header(self.raw.table_data_header_addr)?;
        Ok(self.wrap(raw))
    }
}

impl<'a> Handle<'a, FDBTableDefHeader> {
    pub fn column_count(&self) -> u32 {
        self.raw.column_count
    }

    pub fn table_name(&self) -> Res<'a, &'a Latin1Str> {
        let raw = self.buffer.string(self.raw.table_name_addr)?;
        Ok(self.wrap(raw))
    }

    pub fn column_header_list(&self) -> Res<'a, FDBColumnHeaderSlice<'a>> {
        let len = self.column_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.column_header_list_addr as usize, len)?;
        Ok(self.wrap(FDBColumnHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;
    type IntoIter = Handle<'a, FDBColumnHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    type Item = Handle<'a, FDBColumnHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBColumnHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBColumnHeader> {
    pub fn column_name(&self) -> Res<'a, &'a Latin1Str> {
        let raw = self.buffer.string(self.raw.column_name_addr)?;
        Ok(self.wrap(raw))
    }

    pub fn column_data_type(&self) -> ValueType {
        ValueType::from(self.raw.column_data_type)
    }
}

impl<'a> Handle<'a, FDBTableDataHeader> {
    pub fn bucket_count(&self) -> u32 {
        self.raw.bucket_count
    }

    pub fn bucket_header_list(&self) -> Res<'a, FDBBucketHeaderSlice<'a>> {
        let len = self.bucket_count() as usize * 4;
        let buf = self
            .buffer
            .get_len_at(self.raw.bucket_header_list_addr as usize, len)?;
        Ok(self.wrap(FDBBucketHeaderSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;
    type IntoIter = Handle<'a, FDBBucketHeaderSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    type Item = Handle<'a, FDBBucketHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBBucketHeaderSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, FDBBucketHeader> {
    pub fn first(&self) -> Option<Res<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_head_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(
                self.buffer
                    .row_header_list_entry(addr)
                    .map(|e| self.wrap(e)),
            )
        }
    }

    pub fn bucket_iter(&self) -> Handle<'a, FDBRowHeaderRef> {
        self.wrap(FDBRowHeaderRef(self.raw.row_header_list_head_addr))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FDBRowHeaderRef(u32);

impl<'a> Iterator for Handle<'a, FDBRowHeaderRef> {
    type Item = Res<'a, FDBRowHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.raw.0;
        if addr == 0xFFFFFFFF {
            None
        } else {
            match self.buffer.row_header_list_entry(addr) {
                Ok(e) => {
                    self.raw.0 = e.row_header_list_next_addr;
                    match self.buffer.row_header(e.row_header_addr) {
                        Ok(rh) => Some(Ok(self.wrap(rh))),
                        Err(e) => {
                            self.raw.0 = 0xFFFFFFFF;
                            Some(Err(e))
                        }
                    }
                }
                Err(e) => {
                    self.raw.0 = 0xFFFFFFFF;
                    Some(Err(e))
                }
            }
        }
    }
}

impl<'a> Handle<'a, FDBRowHeaderListEntry> {
    pub fn next(&self) -> Option<Res<'a, FDBRowHeaderListEntry>> {
        let addr = self.raw.row_header_list_next_addr;
        if addr == 0xFFFFFFFF {
            None
        } else {
            Some(
                self.buffer
                    .row_header_list_entry(addr)
                    .map(|e| self.wrap(e)),
            )
        }
    }

    pub fn row_header(&self) -> Res<'a, FDBRowHeader> {
        let e = self.buffer.row_header(self.raw.row_header_addr)?;
        Ok(self.wrap(e))
    }
}

impl<'a> Handle<'a, FDBRowHeader> {
    pub fn field_count(&self) -> u32 {
        self.raw.field_count
    }

    pub fn field_data_list(&self) -> Res<'a, FDBFieldDataSlice> {
        let len = self.field_count() as usize * 8;
        let buf = self
            .buffer
            .get_len_at(self.raw.field_data_list_addr as usize, len)?;
        Ok(self.wrap(FDBFieldDataSlice(buf)))
    }
}

impl<'a> IntoIterator for &Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;
    type IntoIter = Handle<'a, FDBFieldDataSlice<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}

impl<'a> Iterator for Handle<'a, FDBFieldDataSlice<'a>> {
    type Item = Handle<'a, FDBFieldData>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> DoubleEndedIterator for Handle<'a, FDBFieldDataSlice<'a>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back().map(|raw| Handle {
            buffer: self.buffer,
            raw,
        })
    }
}

impl<'a> Handle<'a, &'a Latin1Str> {
    pub fn to_str(&self) -> Cow<'a, str> {
        self.raw.decode()
    }
}
