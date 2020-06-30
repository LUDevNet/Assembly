//! # Parsers for the data
use super::file::*;
use assembly_core::nom::{
    do_parse, named,
    number::complete::{le_u16, le_u32},
    tag,
};

named!(pub parse_chunk_version<ChunkVersion>,
    do_parse!(
        header: le_u16 >>
        data: le_u16 >>
        (ChunkVersion{header, data})
    )
);

named!(pub parse_chunk_header<ChunkHeader>,
    do_parse!(
        tag!("CHNK") >>
        id: le_u32 >>
        version: parse_chunk_version >>
        size: le_u32 >>
        offset: le_u32 >>
        (ChunkHeader{id, version, size, offset})
    )
);

named!(pub parse_file_meta_chunk_data<FileMetaChunkData>,
    do_parse!(
        version: le_u32 >>
        revision: le_u32 >>
        chunk_2000_offset: le_u32 >>
        chunk_2001_offset: le_u32 >>
        chunk_2002_offset: le_u32 >>
        (FileMetaChunkData{
            version, revision,
            chunk_2000_offset, chunk_2001_offset, chunk_2002_offset
        })
    )
);
