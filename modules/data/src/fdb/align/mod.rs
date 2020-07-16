use assembly_core::buffer::Unaligned;
use derive_new::new;
use memchr::memchr;

mod c;
use super::de::slice::Latin1Str;
pub use c::{
    FDBBucketHeaderC, FDBColumnHeaderC, FDBFieldDataC, FDBFieldValueC, FDBHeaderC, FDBRowHeaderC,
    FDBRowHeaderListEntryC, FDBTableDataHeaderC, FDBTableDefHeaderC, FDBTableHeaderC,
};
use std::borrow::Cow;

pub fn get_latin1_str(buf: &[u8], offset: u32) -> &Latin1Str {
    let (_, haystack) = buf.split_at(offset as usize);
    if let Some(end) = memchr(0, haystack) {
        let (content, _) = haystack.split_at(end);
        unsafe { Latin1Str::from_bytes_unchecked(content) }
    } else {
        panic!(
            "Offset {} is supposed to be a string but does not have a null-terminator",
            offset
        );
    }
}

#[derive(Copy, Clone, new)]
pub struct Database<'a> {
    buf: &'a [u8],
}

impl<'a> Database<'a> {
    pub fn tables(self) -> Tables<'a> {
        let header = FDBHeaderC::cast(self.buf, 0);
        let len = header.table_count.extract();
        let base = header.table_header_list_addr.extract();
        let slice = FDBTableHeaderC::cast_slice(self.buf, base, len);
        Tables {
            buf: self.buf,
            slice,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tables<'a> {
    buf: &'a [u8],
    slice: &'a [FDBTableHeaderC],
}

fn map_table_header<'a>(buf: &'a [u8]) -> impl Fn(&'a FDBTableHeaderC) -> Table<'a> {
    move |header: &'a FDBTableHeaderC| {
        let def_header_addr = header.table_def_header_addr.extract();
        let data_header_addr = header.table_data_header_addr.extract();

        let def_header = FDBTableDefHeaderC::cast(buf, def_header_addr);
        let data_header = FDBTableDataHeaderC::cast(buf, data_header_addr).extract();

        let name_addr = def_header.table_name_addr.extract();
        let name = get_latin1_str(buf, name_addr);

        let column_count = def_header.column_count.extract();
        let column_header_list_addr = def_header.column_header_list_addr.extract();

        let columns =
            FDBColumnHeaderC::cast_slice(buf, column_header_list_addr, column_count);

        let buckets = FDBBucketHeaderC::cast_slice(
            buf,
            data_header.bucket_header_list_addr,
            data_header.bucket_count,
        );

        Table {
            buf,
            name,
            columns,
            buckets,
        }
    }
}

impl<'a> Tables<'a> {
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    pub fn get(self, index: usize) -> Option<Table<'a>> {
        self.slice.get(index).map(map_table_header(self.buf))
    }

    pub fn iter(&self) -> impl Iterator<Item = Table<'a>> {
        self.slice.iter().map(map_table_header(self.buf))
    }
}

#[derive(Copy, Clone)]
pub struct Table<'a> {
    buf: &'a [u8],
    name: &'a Latin1Str,
    columns: &'a [FDBColumnHeaderC],
    buckets: &'a [FDBBucketHeaderC],
}

impl<'a> Table<'a> {
    /// Get the undecoded name of the table
    pub fn name_raw(&self) -> &Latin1Str {
        self.name
    }

    /// Get the name of the table
    pub fn name(&self) -> Cow<str> {
        self.name.decode()
    }
}
