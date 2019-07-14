use assembly_core::nom::{
    number::complete::{le_u32},
    IResult,
    named, named_args, do_parse, count, map, take,
};
use super::file::*;
use std::convert::TryInto;

// Read a table definition header from a *.fdb file
named!(pub parse_table_def_header<FDBTableDefHeader>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        c: le_u32 >>
        (FDBTableDefHeader{ column_count: a, table_name_addr: b, column_header_list_addr: c })
    )
);

named!(pub parse_table_data_header<FDBTableDataHeader>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        (FDBTableDataHeader{ bucket_count: a, bucket_header_list_addr: b })
    )
);

named_args!(pub parse_bucket_header_list(i: usize)<FDBBucketHeaderList>,
    map!(
        count!(
            map!(le_u32, |a| FDBBucketHeader{ row_header_list_head_addr: a }),
        i),
    FDBBucketHeaderList::from)
);

named!(parse_column_header<FDBColumnHeader>,
    do_parse!(
        column_data_type: le_u32 >>
        column_name_addr: le_u32 >>
        (FDBColumnHeader { column_data_type, column_name_addr })
    )
);

named_args!(pub parse_column_header_list(i: usize)<FDBColumnHeaderList>,
    map!(count!(parse_column_header, i), FDBColumnHeaderList::from)
);

named!(pub parse_row_header_list_entry<FDBRowHeaderListEntry>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        (FDBRowHeaderListEntry { row_header_addr: a, row_header_list_next_addr: b })
    )
);

named!(pub parse_row_header<FDBRowHeader>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        (FDBRowHeader { field_count: a, field_data_list_addr: b })
    )
);

pub fn parse_field_data(i: &[u8]) -> IResult<&[u8], FDBFieldData> {
    let (i, data_type) = le_u32(i)?;
    let (i, byte_slice) = take!(i, 4)?;
    // This cannot fail
    let value: [u8; 4] = byte_slice.try_into().unwrap();
    Ok((i, FDBFieldData { data_type, value }))
}

named_args!(pub parse_field_data_list(i: usize)<FDBFieldDataList>,
    map!(count!(parse_field_data, i), FDBFieldDataList::from)
);

named!(parse_table_header<FDBTableHeader>,
    do_parse!(
        table_def_header_addr: le_u32 >>
        table_data_header_addr: le_u32 >>
        (FDBTableHeader{ table_def_header_addr, table_data_header_addr })
    )
);

named_args!(pub parse_table_headers(i: usize)<FDBTableHeaderList>,
    map!(count!(parse_table_header, i), FDBTableHeaderList::from)
);

named!(pub parse_header<FDBHeader>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        (FDBHeader{table_count: a, table_header_list_addr: b})
    )
);
