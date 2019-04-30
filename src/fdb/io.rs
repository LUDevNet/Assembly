use std::{fs, io};
use std::io::{Read, Seek, SeekFrom, BufRead};
use std::convert::TryFrom;
use super::core::{
    Schema,
    Table,
    Column,
    Bucket,
    Row,
    Field,
    ValueType,
};
use super::file::{
    FDBHeader,
    FDBTableHeader,
    FDBTableHeaderList,
    FDBTableDefHeader,
    FDBColumnHeader,
    FDBColumnHeaderList,
    FDBTableDataHeader,
    FDBBucketHeader,
    FDBBucketHeaderList,
    FDBRowHeader,
    FDBRowHeaderList,
    FDBRowHeaderListEntry,
    FDBFieldData,
    FDBFieldDataList,
};
use super::parser;
use nom;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1252;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Seek(io::Error),
    Read(io::Error),
    StringEncoding(String),
    Count(std::num::TryFromIntError),
    UnknownType(u32),
    Incomplete,
    ParseError,
    ParseFailure,
    NotImplemented,
}

// Trait to load data from an FDB file
trait TryFromFDB<A>
where A: Read + Seek, Self: Sized {
    type Error;
    type Header;

    fn try_from_fdb(buf: &mut A, header: Self::Header) -> Result<Self, Self::Error>;
}

type LoadResult<A> = Result<A, LoadError>;

impl From<nom::Err<&[u8]>> for LoadError {
    fn from(e: nom::Err<&[u8]>) -> LoadError {
        match e {
            // Need to translate the error here, as this lives longer than the input
            nom::Err::Incomplete(_) => LoadError::Incomplete,
            nom::Err::Error(_) => LoadError::ParseError,
            nom::Err::Failure(_) => LoadError::ParseFailure,
        }
    }
}


impl TryFrom<&str> for Schema {
    type Error = LoadError;

    fn try_from(filename: &str) -> LoadResult<Schema> {
        fs::File::open(filename)
            .map_err(LoadError::Io)
            .and_then(Schema::try_from)
    }
}

impl TryFrom<fs::File> for Schema {
    type Error = LoadError;

    fn try_from(file: fs::File) -> LoadResult<Schema> {
        Schema::try_from_fdb(&mut io::BufReader::new(file), ())
    }
}

impl<T> TryFromFDB<T> for FDBHeader
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = ();

    fn try_from_fdb(buf: &mut T, _header: ()) -> LoadResult<FDBHeader> {
        let mut header_bytes: [u8; 8] = [0; 8];
        buf.read_exact(&mut header_bytes)
            .map_err(LoadError::Read)
            .and_then(|_| parser::parse_header(&header_bytes)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for FDBTableHeaderList
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBHeader;

    fn try_from_fdb(buf: &mut T, header: FDBHeader) -> LoadResult<FDBTableHeaderList> {
        let mut table_headers_bytes: Vec<u8> = Vec::new();
        let count = header.table_count;
        buf.take((count * 8).into()).read_to_end(&mut table_headers_bytes)
            .map_err(LoadError::Read)
            .and_then(|_| usize::try_from(count)
                .map_err(LoadError::Count))
            .and_then(|c| parser::parse_table_headers(&table_headers_bytes, c)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<'a,T> TryFromFDB<T> for FDBTableDefHeader
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = u32;

    fn try_from_fdb(buf: &mut T, addr: u32) -> LoadResult<FDBTableDefHeader> {
        buf.seek(SeekFrom::Start(addr.into()))
            .map_err(LoadError::Io)
            .and_then(|_| {
                let mut table_def_header_bytes: [u8; 12] = [0; 12];
                buf.read_exact(&mut table_def_header_bytes)
                    .map_err(LoadError::Read)
                    .and_then(|_| parser::parse_table_def_header(&table_def_header_bytes)
                        .map_err(LoadError::from)
                        .map(|r| r.1)
                    )
            })
    }
}

impl<T> TryFromFDB<T> for String
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = u32;

    fn try_from_fdb(buf: &mut T, addr: u32) -> LoadResult<String> {
        let mut string: Vec<u8> = Vec::new();
        buf.seek(SeekFrom::Start(addr.into()))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.read_until(0x00, &mut string)
                .map_err(LoadError::Read))
            .and_then(|_| {
                if string.ends_with(&[0x00]) {
                    string.pop();
                }
                WINDOWS_1252.decode(&string, DecoderTrap::Strict)
                    .map_err(|e| LoadError::StringEncoding(String::from(e)))
            })
    }
}

impl<T> TryFromFDB<T> for i64
where T: Read + Seek {
    type Error = LoadError;
    type Header = u32;

    fn try_from_fdb(buf: &mut T, addr: u32) -> LoadResult<Self> {
        let mut bytes: [u8; 8] = [0; 8];
        buf.seek(SeekFrom::Start(addr.into()))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.read_exact(&mut bytes)
                .map_err(LoadError::Read))
            .map(|_| i64::from_le_bytes(bytes))
    }
}

impl<T> TryFromFDB<T> for FDBColumnHeaderList
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBTableDefHeader;

    fn try_from_fdb(buf: &mut T, header: FDBTableDefHeader) -> LoadResult<FDBColumnHeaderList> {
        let off: u64 = header.column_header_list_addr.into();
        let mut column_header_list_bytes: Vec<u8> = Vec::new();
        let count: u32 = header.column_count;
        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Io)
            .and_then(|_| {
                buf.take((count * 8).into()).read_to_end(&mut column_header_list_bytes)
                    .map_err(LoadError::Io)
                    .and_then(|_| usize::try_from(count)
                        .map_err(LoadError::Count))
                })
            .and_then(|count| {
                parser::parse_column_header_list(&mut column_header_list_bytes, count)
                    .map_err(LoadError::from)
                    .map(|r| r.1)
                })
    }
}

impl<T> TryFromFDB<T> for FDBTableDataHeader
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBTableHeader;

    fn try_from_fdb(buf: &mut T, header: FDBTableHeader) -> LoadResult<FDBTableDataHeader> {
        let off: u64 = header.table_data_header_addr.into();
        let mut table_data_header_bytes: [u8; 8] = [0; 8];
        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.read_exact(&mut table_data_header_bytes)
                .map_err(LoadError::Read))
            .and_then(|_| parser::parse_table_data_header(&table_data_header_bytes)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for FDBBucketHeaderList
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBTableDataHeader;

    fn try_from_fdb(buf: &mut T, header: FDBTableDataHeader) -> LoadResult<FDBBucketHeaderList> {
        let off: u64 = header.bucket_header_list_addr.into();
        let count: u32 = header.bucket_count;
        let mut bucket_header_list_bytes: Vec<u8> = Vec::new();

        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.take((count * 4).into()).read_to_end(&mut bucket_header_list_bytes)
                .map_err(LoadError::Read))
            .and_then(|_| usize::try_from(count)
                .map_err(LoadError::Count))
            .and_then(|c| parser::parse_bucket_header_list(&bucket_header_list_bytes, c)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for FDBRowHeaderListEntry
where T: Read + Seek {
    type Error = LoadError;
    type Header = u32;

    fn try_from_fdb(buf: &mut T, header: u32) -> LoadResult<FDBRowHeaderListEntry> {
        let off: u64 = header.into();
        let mut row_header_list_entry_bytes: [u8; 8] = [0; 8];
        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.take(8)
                .read_exact(&mut row_header_list_entry_bytes)
                .map_err(LoadError::Read))
            .and_then(|_| parser::parse_row_header_list_entry(&row_header_list_entry_bytes)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for FDBRowHeaderList
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBBucketHeader;

    fn try_from_fdb(mut buf: &mut T, header: FDBBucketHeader) -> LoadResult<FDBRowHeaderList> {
        let mut next: LoadResult<u32> = Ok(header.row_header_list_head_addr);
        let mut row_header_list: Vec<FDBRowHeader> = Vec::new();
        loop {
            next = match next {
                Ok(std::u32::MAX) => break,
                Ok(addr) => {
                    FDBRowHeaderListEntry::try_from_fdb(&mut buf, addr)
                        .and_then(|entry| {
                            let next_addr = entry.row_header_list_next_addr;
                            FDBRowHeader::try_from_fdb(&mut buf, entry)
                                .map(|row_header| {
                                    row_header_list.push(row_header);
                                    next_addr
                                })
                        })
                }
                Err(e) => Err(e)
            }
        }
        next.map(|_| FDBRowHeaderList::from(row_header_list))
    }
}

impl<T> TryFromFDB<T> for FDBRowHeader
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBRowHeaderListEntry;

    fn try_from_fdb(buf: &mut T, header: FDBRowHeaderListEntry) -> LoadResult<FDBRowHeader> {
        let off: u64 = header.row_header_addr.into();
        let mut row_header_bytes: [u8; 8] = [0; 8];
        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.take(8)
                .read_exact(&mut row_header_bytes)
                .map_err(LoadError::Read))
            .and_then(|_| parser::parse_row_header(&row_header_bytes)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for FDBFieldDataList
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBRowHeader;

    fn try_from_fdb(buf: &mut T, header: Self::Header) -> LoadResult<Self> {
        let off: u64 = header.field_data_list_addr.into();
        let count: u32 = header.field_count;
        let mut field_data_list_bytes: Vec<u8> = Vec::new();

        buf.seek(SeekFrom::Start(off))
            .map_err(LoadError::Seek)
            .and_then(|_| buf.take((count * 8).into()).read_to_end(&mut field_data_list_bytes)
                .map_err(LoadError::Read))
            .and_then(|_| usize::try_from(count)
                .map_err(LoadError::Count))
            .and_then(|c| parser::parse_field_data_list(&field_data_list_bytes, c)
                .map_err(LoadError::from))
            .map(|r| r.1)
    }
}

impl<T> TryFromFDB<T> for Field
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBFieldData;

    fn try_from_fdb(mut buf: &mut T, header: Self::Header) -> LoadResult<Self> {
        let bytes = header.value;
        match ValueType::from(header.data_type) {
            ValueType::Nothing => Ok(Field::Nothing),
            ValueType::Integer => Ok(bytes)
                .map(i32::from_le_bytes)
                .map(Field::Integer),
            ValueType::Float => Ok(bytes)
                .map(u32::from_le_bytes)
                .map(f32::from_bits)
                .map(Field::Float),
            ValueType::Text => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| String::try_from_fdb(&mut buf, addr))
                .map(Field::Text),
            ValueType::Boolean => Ok(bytes)
                .map(|v| v != [0; 4])
                .map(Field::Boolean),
            ValueType::BigInt => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| i64::try_from_fdb(&mut buf, addr))
                .map(Field::BigInt),
            ValueType::VarChar => Ok(bytes)
                .map(u32::from_le_bytes)
                .and_then(|addr| String::try_from_fdb(&mut buf, addr))
                .map(Field::VarChar),
            ValueType::Unknown(k) =>
                Err(LoadError::UnknownType(k))
        }
    }
}

impl<T> TryFromFDB<T> for Row
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBRowHeader;

    fn try_from_fdb(mut buf: &mut T, header: Self::Header) -> LoadResult<Self> {
        FDBFieldDataList::try_from_fdb(&mut buf, header)
            .map(|field_list| {
                let mut fields: Vec<Field> = Vec::new();
                for field in field_list {
                    match Field::try_from_fdb(&mut buf, field) {
                        Ok(value) => fields.push(value),
                        Err(e) => println!("{:?}", e),
                    }
                }
                Row::from(fields)
            })
    }
}

impl<T> TryFromFDB<T> for Bucket
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBBucketHeader;

    fn try_from_fdb(buf: &mut T, header: Self::Header) -> LoadResult<Self> {
        let mut rows: Vec<Row> = Vec::new();
        FDBRowHeaderList::try_from_fdb(buf, header)
            .and_then(|row_headers| {
                for row_header in row_headers {
                    match Row::try_from_fdb(buf, row_header) {
                        Ok(row) => {
                            println!("{:?}", row);
                            rows.push(row);
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
                Ok(Bucket(rows))
            })
    }
}

impl<T> TryFromFDB<T> for Column
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBColumnHeader;

    fn try_from_fdb(buf: &mut T, header: FDBColumnHeader) -> LoadResult<Column> {
        let col_type = ValueType::from(header.column_data_type);
        String::try_from_fdb(buf, header.column_name_addr)
            .map(|col_name| Column::from((col_name.as_ref(), col_type)))
    }
}

impl<T> TryFromFDB<T> for Table
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = FDBTableHeader;

    fn try_from_fdb(mut buf: &mut T, header: FDBTableHeader) -> LoadResult<Table> {
        FDBTableDefHeader::try_from_fdb(buf, header.table_def_header_addr)
        .and_then(|def_header| {
            String::try_from_fdb(buf, def_header.table_name_addr)
                .map(|table_name| (def_header, table_name))
        })
        .and_then(|(def_header, table_name)| {
            println!("# Table {}", table_name);
            FDBColumnHeaderList::try_from_fdb(buf, def_header)
                .map(|column_def| {
                    let mut table_columns: Vec<Column> = Vec::new();
                    let cs: Vec<FDBColumnHeader> = column_def.into();
                    for column in cs {
                        match Column::try_from_fdb(&mut buf, column) {
                            Ok(col) => {
                                println!("{:?}", col);
                                table_columns.push(col);
                            },
                            Err(e) => {
                                println!("{:?}", e);
                            },
                        }
                    }
                    (table_name, table_columns)
                })
        })
        .and_then(|(table_name, table_columns)| {
            let mut table_buckets: Vec<Bucket> = Vec::new();
            FDBTableDataHeader::try_from_fdb(buf, header)
                .and_then(|table_data_header| {
                    FDBBucketHeaderList::try_from_fdb(buf, table_data_header)
                })
                .map(|bucket_header_list| {
                    for bucket_header in bucket_header_list {
                        match Bucket::try_from_fdb(buf, bucket_header) {
                            Ok(bucket) => table_buckets.push(bucket),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    (table_name, table_columns, table_buckets)
                })
        })
        .and_then(|(table_name, table_columns, table_buckets)| {
            Ok(Table::new(table_name, table_buckets, table_columns))
        })
    }
}

impl<T> TryFromFDB<T> for Schema
where T: BufRead + Seek {
    type Error = LoadError;
    type Header = ();

    fn try_from_fdb(mut buf: &mut T, _header: ()) -> LoadResult<Schema> {
        FDBHeader::try_from_fdb(buf, ())
            .and_then(|header| {
                println!("{:?}", header);
                buf.seek(SeekFrom::Start(header.table_header_list_addr.into()))
                .map_err(LoadError::Io)
                .map(|_| header)
            })
            .and_then(|header| {
                FDBTableHeaderList::try_from_fdb(&mut buf, header)
                .map(|headers| {
                    let hs: Vec<FDBTableHeader> = headers.into();
                    let mut tables: Vec<Table> = Vec::new();
                    for h in hs {
                        println!("{:?}", h);
                        match Table::try_from_fdb(&mut buf, h) {
                            Ok(table) => tables.push(table),
                            Err(e) => {
                                println!("{:?}", e)
                            },
                        }
                    }
                    Schema::from(tables)
                })
            })
            .map_err(|e| {
                println!("{:?}", e);
                e
            })
    }
}
