//! # Data definitions for sysdiagrams

#[derive(Debug)]
pub struct TableInfo {
    //pub head_1: u16,
    pub head_2: u16,
    pub head_3: u32,
    // pub head_4: Option<u32>,
    // type_length: u16
    // 0x00 0x80
    // head_4: u16,
    pub id: u32,
    pub value_1: u32,
    pub value_2a: u8,
    pub value_2b: u8,
    pub value_3: u16,
    // name_length: u16,
    // 0x00 0x80
    // value_4: u16,
    pub info_type: String,

    pub mid_1: u32,
    pub mid_2: u32,
    pub name: String,

    //pub end_1: u16,
}

#[derive(Debug)]
pub struct SysDiagram {
}
