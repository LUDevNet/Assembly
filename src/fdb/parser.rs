use nom::{le_u32, le_u8};
use super::file::*;

/// Read a table definition header from a *.fdb file
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

named_args!(pub parse_column_header_list(i: usize)<FDBColumnHeaderList>,
    map!(
        count!(
            do_parse!(
                a: le_u32 >>
                b: le_u32 >>
                (FDBColumnHeader { column_data_type: a, column_name_addr: b })
            ),
        i), FDBColumnHeaderList::from
    )
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

named_args!(pub parse_field_data_list(i: usize)<FDBFieldDataList>,
    map!(
        count!(
            do_parse!(
                a: le_u32 >>
                b: count_fixed!(u8, le_u8, 4) >>
                (FDBFieldData { data_type: a, value: b })
            ),
        i), FDBFieldDataList::from
    )
);

named_args!(pub parse_table_headers(i: usize)<FDBTableHeaderList>,
    map!(
        count!(
            do_parse!(
                a: le_u32 >>
                b: le_u32 >>
                (FDBTableHeader{table_def_header_addr: a, table_data_header_addr: b})
            ),
        i), FDBTableHeaderList::from
    )
);

named!(pub parse_header<FDBHeader>,
    do_parse!(
        a: le_u32 >>
        b: le_u32 >>
        (FDBHeader{table_count: a, table_header_list_addr: b})
    )
);
