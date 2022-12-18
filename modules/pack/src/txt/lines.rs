use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use nom::{
    bytes::complete::{take, take_while},
    character::complete::char,
    combinator::{map, map_res, rest},
    sequence::{preceded, tuple},
    IResult,
};
use nom_supreme::final_parser::{final_parser, Location};

use crate::md5::MD5Sum;

/*
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}
*/

pub(crate) fn md5(input: &str) -> IResult<&str, MD5Sum> {
    map_res(take(32usize), MD5Sum::from_str)(input)
}

/// The line in the `[version]` section
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionLine {
    /// The version of this manifest
    pub version: u32,
    /// The hash of `version` as a string
    pub hash: MD5Sum,
    /// The name of this manifest
    ///
    /// This name in this field in `trunk.txt` is appended to the version on the loading screen.
    pub name: String,
}

impl fmt::Display for VersionLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.version, self.hash, self.name)
    }
}

impl VersionLine {
    /// Create a new version line
    pub fn new(version: u32, name: String) -> VersionLine {
        let hash = MD5Sum(md5::compute(version.to_string()).0);
        VersionLine {
            version,
            hash,
            name,
        }
    }

    /// Hash the version number
    pub fn hash_version(&self) -> MD5Sum {
        MD5Sum(md5::compute(self.version.to_string()).0)
    }

    /// Check whether the version and it's hash match
    pub fn verify(&self) -> bool {
        self.hash == self.hash_version()
    }
}

fn decimal(input: &str) -> IResult<&str, u32> {
    map_res(take_while(|c: char| c.is_ascii_digit()), str::parse)(input)
}

fn _version_line(input: &str) -> IResult<&str, VersionLine> {
    map(
        tuple((
            decimal,
            preceded(char(','), md5),
            preceded(char(','), map(rest, String::from)),
        )),
        |(version, hash, name)| VersionLine {
            version,
            hash,
            name,
        },
    )(input)
}

pub(crate) fn version_line(input: &str) -> Result<VersionLine, nom::error::Error<Location>> {
    final_parser(_version_line)(input)
}

/// Metadata for a single file
pub struct FileMeta {
    /// Size of the file
    pub size: u32,
    /// md5sum of the file
    pub hash: MD5Sum,
}

/// One line in the `[files]` section
///
/// This doesn't include the path, which is a key
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileLine {
    /// Size of the file
    pub filesize: u32,
    /// md5sum of the file
    pub hash: MD5Sum,
    /// Size of the compressed file
    pub compressed_filesize: u32,
    /// md5sum of the compressed file
    pub compressed_hash: MD5Sum,
    /// Hash of the comma separated line
    pub line_hash: MD5Sum,
}

impl fmt::Display for FileLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.filesize,
            self.hash,
            self.compressed_filesize,
            self.compressed_hash,
            self.line_hash
        )
    }
}

impl FileLine {
    /// Get the (relative) patcher URL for this file
    pub fn to_path(&self) -> String {
        let hash = format!("{:?}", self.hash);
        let mut chars = hash.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        format!("{}/{}/{}.sd0", c1, c2, hash)
    }

    /// Create a new file line
    pub fn new(raw: FileMeta, compressed: FileMeta) -> Self {
        let text = format!(
            "{},{},{},{}",
            raw.size, raw.hash, compressed.size, compressed.hash
        );
        Self {
            filesize: raw.size,
            hash: raw.hash,
            compressed_filesize: compressed.size,
            compressed_hash: compressed.hash,
            line_hash: MD5Sum(md5::compute(text).0),
        }
    }
}

fn _file_line(input: &str) -> IResult<&str, (&str, FileLine)> {
    map(
        tuple((
            take_while(|c: char| c != ','),
            preceded(char(','), decimal),
            preceded(char(','), md5),
            preceded(char(','), decimal),
            preceded(char(','), md5),
            preceded(char(','), md5),
        )),
        |(filename, filesize, hash, compressed_filesize, compressed_hash, line_hash)| {
            (
                filename,
                FileLine {
                    filesize,
                    hash,
                    compressed_filesize,
                    compressed_hash,
                    line_hash,
                },
            )
        },
    )(input)
}

pub(crate) fn file_line(input: &str) -> Result<(&str, FileLine), nom::error::Error<Location>> {
    final_parser(_file_line)(input)
}

#[cfg(test)]
mod tests {
    use super::{MD5Sum, VersionLine};

    const BYTES: [u8; 16] = [
        0xe1, 0x77, 0x1d, 0x0f, 0x4c, 0x93, 0xe3, 0x27, 0xc6, 0x62, 0x1a, 0x0e, 0xf2, 0xe1, 0xbd,
        0xce,
    ];

    #[test]
    fn parse_md5() {
        assert_eq!(
            super::md5("e1771d0f4c93e327c6621a0ef2e1bdce"),
            Ok(("", MD5Sum(BYTES)))
        );
    }

    #[test]
    fn parse_version_line() {
        let hash = MD5Sum([
            0x97, 0x78, 0xd5, 0xd2, 0x19, 0xc5, 0x08, 0x0b, 0x9a, 0x6a, 0x17, 0xbe, 0xf0, 0x29,
            0x33, 0x1c,
        ]);
        assert_eq!(
            super::version_line("82,9778d5d219c5080b9a6a17bef029331c,0"),
            Ok(VersionLine {
                version: 82,
                hash,
                name: "0".into(),
            })
        );
    }
}
