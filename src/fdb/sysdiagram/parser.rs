use super::core::*;
use crate::core::parser::{parse_string_u16};
use nom::{le_u32, le_u16, le_u8};

named!(pub parse_table_info<TableInfo>,
    do_parse!(
        head_2: le_u16 >>
        head_3: le_u32 >>
        type_len: le_u16 >>
        _type_len: tag!([0x00, 0x80]) >>
        id: le_u32 >>
        value_1: le_u32 >>
        value_2a: le_u8 >>
        value_2b: le_u8 >>
        value_3: le_u16 >>
        name_len: le_u16 >>
        _name_len: tag!([0x00, 0x80]) >>
        info_type: call!(parse_string_u16, type_len) >>
        tag!([0x00]) >>
        mid_1: le_u32 >>
        mid_2: le_u32 >>
        name: call!(parse_string_u16, name_len) >>
        _name_pad: take!(3 - (name_len + 1) % 4) >>
        _end_1: tag!([0x00]) >>
        (TableInfo {
            head_2, head_3,
            id,
            value_1, value_2a, value_2b, value_3,
            info_type,
            mid_1, mid_2,
            name
            //end_1,
        })
    )
);
