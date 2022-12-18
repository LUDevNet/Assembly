#![cfg(feature = "md5sum")]

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::common::FileMeta;

use super::io::IOSum;

/// Get the md5sum of a file
pub fn md5sum(path: &Path) -> io::Result<FileMeta> {
    let file = File::open(path)?;
    let mut md5sum = IOSum::new(file);

    let mut buf: Box<[u8]> = Box::from([0u8; 1204 * 16]);

    let mut c = 1;
    while c > 0 {
        c = md5sum.read(buf.as_mut())?;
    }

    let size = md5sum.byte_count() as u32;
    let (_, hash) = md5sum.into_inner();
    Ok(FileMeta { size, hash })
}
