use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use nom::{
    bytes::complete::{take_while, take_while_m_n},
    character::complete::char,
    combinator::{map, map_res, rest},
    multi::fill,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use nom_supreme::final_parser::{final_parser, Location};

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MD5(pub [u8; 16]);

impl fmt::Debug for MD5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..16 {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}

impl FromStr for MD5 {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        md5(input).finish().map(|(_rest, b)| b).map_err(|_e| ())
    }
}

pub(crate) fn md5(input: &str) -> IResult<&str, MD5> {
    let mut buf = [0u8; 16];
    let (input, ()) = fill(hex_primary, &mut buf[..])(input)?;
    Ok((input, MD5(buf)))
}

/// The line in the `[version]` section
#[derive(Debug, PartialEq)]
pub struct VersionLine {
    /// The version of this manifest
    pub version: u32,
    /// The hash of ???
    pub hash: MD5,
    /// The name of this manifest
    pub name: String,
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

/// One line in the `[files]` section
///
/// This doesn't include the path, which is a key
#[derive(Debug, PartialEq)]
pub struct FileLine {
    /// Size of the file
    pub filesize: u32,
    /// md5sum of the file
    pub hash: MD5,
    /// Size of the compressed file
    pub compressed_filesize: u32,
    /// md5sum of the compressed file
    pub compressed_hash: MD5,
    /// Hash of the comma separated line
    pub line_hash: MD5,
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
    use super::{VersionLine, MD5};

    const BYTES: [u8; 16] = [
        0xe1, 0x77, 0x1d, 0x0f, 0x4c, 0x93, 0xe3, 0x27, 0xc6, 0x62, 0x1a, 0x0e, 0xf2, 0xe1, 0xbd,
        0xce,
    ];

    #[test]
    fn parse_md5() {
        assert_eq!(
            super::md5("e1771d0f4c93e327c6621a0ef2e1bdce"),
            Ok(("", MD5(BYTES)))
        );
    }

    #[test]
    fn parse_version_line() {
        let hash = MD5([
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
