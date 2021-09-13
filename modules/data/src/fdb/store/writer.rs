use std::{hint::unreachable_unchecked, io};

use crate::fdb::{
    common::Latin1Str,
    file::{
        ArrayHeader, FDBBucketHeader, FDBColumnHeader, FDBFieldData, FDBRowHeader,
        FDBRowHeaderListEntry, FDBTableDataHeader, FDBTableDefHeader, FDBTableHeader,
    },
};

#[allow(clippy::upper_case_acronyms)]
pub trait WriteLE {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()>;
}

impl WriteLE for Latin1Str {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(self.as_bytes())?;
        match self.len() % 4 {
            0 => out.write_all(&[0, 0, 0, 0])?,
            1 => out.write_all(&[0, 0, 0])?,
            2 => out.write_all(&[0, 0])?,
            3 => out.write_all(&[0])?,
            _ => unsafe { unreachable_unchecked() },
        }
        Ok(())
    }
}

impl WriteLE for ArrayHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.count.to_le_bytes())?;
        out.write_all(&self.base_offset.to_le_bytes())?;
        Ok(())
    }
}

impl WriteLE for FDBTableHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.table_def_header_addr.to_le_bytes())?;
        out.write_all(&self.table_data_header_addr.to_le_bytes())?;
        Ok(())
    }
}

impl WriteLE for FDBTableDefHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.column_count.to_le_bytes())?;
        out.write_all(&self.table_name_addr.to_le_bytes())?;
        out.write_all(&self.column_header_list_addr.to_le_bytes())?;
        Ok(())
    }
}

impl WriteLE for FDBColumnHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.column_data_type.to_le_bytes())?;
        out.write_all(&self.column_name_addr.to_le_bytes())?;
        Ok(())
    }
}

impl WriteLE for FDBTableDataHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        self.buckets.write_le(out)
    }
}

impl WriteLE for FDBBucketHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.row_header_list_head_addr.to_le_bytes())
    }
}

impl WriteLE for FDBRowHeaderListEntry {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&self.row_header_addr.to_le_bytes())?;
        out.write_all(&self.row_header_list_next_addr.to_le_bytes())?;
        Ok(())
    }
}

impl WriteLE for FDBRowHeader {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        self.fields.write_le(out)
    }
}

impl WriteLE for FDBFieldData {
    fn write_le<IO: io::Write>(&self, out: &mut IO) -> io::Result<()> {
        out.write_all(&u32::to_le_bytes(self.data_type))?;
        out.write_all(&self.value)?;
        Ok(())
    }
}
