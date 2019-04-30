use std::convert::From;
use nom::{IResult, le_u32, le_u8, do_parse, count, count_fixed};
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
    FDBFieldData,
    FDBFieldDataList,
    FDBRowHeader,
    FDBRowHeaderListEntry,
};

pub fn parse_table_def_header(data: &[u8]) -> IResult<&[u8], FDBTableDefHeader> {
    do_parse!(data,
        a: le_u32 >>
        b: le_u32 >>
        c: le_u32 >>
        (FDBTableDefHeader{ column_count: a, table_name_addr: b, column_header_list_addr: c })
    )
}

pub fn parse_table_data_header(data: &[u8]) -> IResult<&[u8], FDBTableDataHeader> {
    do_parse!(data,
        a: le_u32 >>
        b: le_u32 >>
        (FDBTableDataHeader{ bucket_count: a, bucket_header_list_addr: b })
    )
}

pub fn parse_bucket_header_list(data: &[u8], i: usize) -> IResult<&[u8], FDBBucketHeaderList> {
    count!(data,
        do_parse!(
            a: le_u32 >>
            (FDBBucketHeader{ row_header_list_head_addr: a })
        ), i)
        .map(|(r, vec)| (r, FDBBucketHeaderList::from(vec)))
}

pub fn parse_column_header_list(data: &[u8], i: usize) -> IResult<&[u8], FDBColumnHeaderList> {
    count!(data,
        do_parse!(
            a: le_u32 >>
            b: le_u32 >>
            (FDBColumnHeader { column_data_type: a, column_name_addr: b })
        ), i)
        .map(|(r, vec)| (r, FDBColumnHeaderList::from(vec)))
}

pub fn parse_row_header_list_entry(data: &[u8]) -> IResult<&[u8], FDBRowHeaderListEntry> {
    do_parse!(data,
        a: le_u32 >>
        b: le_u32 >>
        (FDBRowHeaderListEntry { row_header_addr: a, row_header_list_next_addr: b })
    )
}

pub fn parse_row_header(data: &[u8]) -> IResult<&[u8], FDBRowHeader> {
    do_parse!(data,
        a: le_u32 >>
        b: le_u32 >>
        (FDBRowHeader { field_count: a, field_data_list_addr: b })
    )
}

pub fn parse_field_data_list(data: &[u8], i: usize) -> IResult<&[u8], FDBFieldDataList> {
    count!(data,
        do_parse!(
            a: le_u32 >>
            b: count_fixed!(u8, le_u8, 4) >>
            (FDBFieldData { data_type: a, value: b })
        ), i)
        .map(|(r, vec)| (r, FDBFieldDataList::from(vec)))
}

pub fn parse_table_headers(data: &[u8], i: usize) -> IResult<&[u8], FDBTableHeaderList> {
    count!(data,
        do_parse!(
            a: le_u32 >>
            b: le_u32 >>
            (FDBTableHeader{table_def_header_addr: a, table_data_header_addr: b})
        ), i)
        .map(|(r, vec)| (r, FDBTableHeaderList::from(vec)))
}

pub fn parse_header(data: &[u8]) -> IResult<&[u8], FDBHeader> {
    do_parse!(data,
        a: le_u32 >>
        b: le_u32 >>
        (FDBHeader{table_count: a, table_header_list_addr: b})
    )
}
