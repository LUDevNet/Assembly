use super::core::*;

use assembly_core::parser::parse_u32_string;
use assembly_core::nom::{
    named, do_parse, tag, length_count, map_res, fold_many_m_n,
    number::complete::le_u32,
};

use std::collections::BTreeMap;
use std::convert::TryFrom;

struct FileRefData {
    filename_crc: u32,
    #[allow(dead_code)]
    left: u32,
    #[allow(dead_code)]
    right: u32,
    pack_file: u32,
    category: u32,
}

fn extend_map(mut map: BTreeMap<u32, FileRef>, data: FileRefData) -> BTreeMap<u32, FileRef> {
    map.insert(data.filename_crc, FileRef{category: data.category, pack_file: data.pack_file});
    map
}

named!(parse_file_ref<FileRefData>,
    do_parse!(
        filename_crc: le_u32 >>
        left: le_u32 >>
        right: le_u32 >>
        pack_file: le_u32 >>
        category: le_u32 >>
        (FileRefData{filename_crc, left, right, pack_file, category})
    )
);

named!(parse_pack_file_ref<PackFileRef>,
    do_parse!(
        path: parse_u32_string >>
        (PackFileRef{path})
    )
);

named!(pub parse_pki_file<PackIndexFile>,
    do_parse!(
        _version: tag!(u32::to_le_bytes(3)) >>
        archives: length_count!(le_u32, parse_pack_file_ref) >>
        file_count: map_res!(le_u32, usize::try_from) >>
        files: fold_many_m_n!(file_count, file_count, parse_file_ref, BTreeMap::new(), extend_map) >>
        (PackIndexFile{archives,files})
    )
);
