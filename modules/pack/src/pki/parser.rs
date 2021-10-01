use crate::common::{parser::parse_crc_node, CRCTreeNode};

use super::core::*;

use assembly_core::nom::{
    bytes::complete::tag,
    combinator::map_res,
    multi::{fold_many_m_n, length_count},
    number::complete::le_u32,
    IResult,
};
use assembly_core::parser::parse_u32_string;

use std::collections::BTreeMap;
use std::convert::TryFrom;

type FileRefData = CRCTreeNode<FileRef>;

fn extend_map(mut map: BTreeMap<u32, FileRef>, data: FileRefData) -> BTreeMap<u32, FileRef> {
    map.insert(data.crc, data.data);
    map
}

fn parse_file_ref(input: &[u8]) -> IResult<&[u8], FileRef> {
    let (input, pack_file) = le_u32(input)?;
    let (input, category) = le_u32(input)?;
    Ok((
        input,
        FileRef {
            pack_file,
            category,
        },
    ))
}

fn parse_file_ref_node(input: &[u8]) -> IResult<&[u8], FileRefData> {
    parse_crc_node(parse_file_ref)(input)
}

fn parse_pack_file_ref(input: &[u8]) -> IResult<&[u8], PackFileRef> {
    let (input, path) = parse_u32_string(input)?;
    Ok((input, PackFileRef { path }))
}

const LE_THREE: [u8; 4] = u32::to_le_bytes(3);

pub fn parse_pki_file(input: &[u8]) -> IResult<&[u8], PackIndexFile> {
    let (input, _version) = tag(LE_THREE)(input)?;
    let (input, archives) = length_count(le_u32, parse_pack_file_ref)(input)?;
    let (input, file_count) = map_res(le_u32, usize::try_from)(input)?;
    let (input, files) = fold_many_m_n(
        file_count,
        file_count,
        parse_file_ref_node,
        BTreeMap::new,
        extend_map,
    )(input)?;
    Ok((input, PackIndexFile { archives, files }))
}
