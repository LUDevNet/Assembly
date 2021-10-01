//! # `md5` hashsum of files

use std::{fmt, io::Write, str::FromStr};

use serde::{ser::SerializeTuple, Deserialize, Serialize};

pub mod padded;

/// # MD5 hashsum of a file
///
/// The game uses md5 hashes of the content to identify a specific version of a file.
///
/// *Note*: Currently, this doesn't actually provide an implementation of that hash. Please use a tool
/// like `md5sum` for that.
#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MD5Sum(pub [u8; 16]);

impl MD5Sum {
    pub fn from_hex_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 32 {
            return Err(Error::InvalidLength(bytes.len()));
        }

        let mut arr = [0u8; 16];
        let mut k = 4;

        for (bi, b) in bytes.iter().copied().enumerate() {
            let v = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                _ => return Err(Error::InvalidByte { index: bi as u8, b }),
            };
            arr[bi >> 1] += v << k;
            k = 4 - k;
        }

        Ok(Self(arr))
    }
}

impl fmt::Debug for MD5Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..16 {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}

impl fmt::Display for MD5Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidLength(usize),
    InvalidByte { index: u8, b: u8 },
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(v) => write!(f, "Invalid byte count for md5sum: {}", v),
            Self::InvalidByte { index, b } => {
                write!(f, "Invalid byte {} at {} for md5sum", b, index)
            }
        }
    }
}

impl FromStr for MD5Sum {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_hex_bytes(input.as_bytes())
    }
}

struct MD5Visitor;

impl<'de> serde::de::Visitor<'de> for MD5Visitor {
    type Value = MD5Sum;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "md5sum hex string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        MD5Sum::from_str(v).map_err(|e| E::custom(e.to_string()))
    }
}

impl Serialize for MD5Sum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = [0u8; 32];
        write!(bytes.as_mut(), "{:?}", self).unwrap();
        if serializer.is_human_readable() {
            let str = unsafe { std::str::from_utf8_unchecked(&bytes[..]) };
            serializer.serialize_str(str)
        } else {
            let mut m = serializer.serialize_tuple(32)?;
            for b in bytes {
                m.serialize_element(&b)?;
            }
            m.end()
        }
    }
}

impl<'de> Deserialize<'de> for MD5Sum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(MD5Visitor)
        } else {
            let bytes = <[u8; 32]>::deserialize(deserializer)?;
            match MD5Sum::from_hex_bytes(&bytes) {
                Ok(sum) => Ok(sum),
                Err(e) => Err(<D::Error as serde::de::Error>::custom(e.to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MD5Sum;

    const ARR: [u8; 16] = [
        0x33, 0x7e, 0x24, 0xd7, 0x26, 0xfd, 0x72, 0x8f, //
        0x92, 0x95, 0x7a, 0x2c, 0x90, 0x8d, 0xde, 0xe6,
    ];
    const STR: &str = "337e24d726fd728f92957a2c908ddee6";

    #[test]
    fn test() {
        let s = MD5Sum::from_hex_bytes(STR.as_bytes()).unwrap();
        assert_eq!(s.0, ARR);
        let f = format!("{:?}", s);
        assert_eq!(f, STR);
    }

    #[test]
    fn test_bincode() {
        let s: MD5Sum = bincode::deserialize(STR.as_bytes()).unwrap();
        assert_eq!(s.0, ARR);
        let bytes = bincode::serialize(&s).unwrap();
        assert_eq!(&bytes, STR.as_bytes());
    }

    #[test]
    fn test_json() {
        let input = format!("\"{}\"", STR);
        let s: MD5Sum = serde_json::from_str(&input).unwrap();
        assert_eq!(s.0, ARR);
        let output = serde_json::to_string(&s).unwrap();
        assert_eq!(output, input);
    }
}
