//! # Parser methods for the general types
use super::types::{ObjectID, ObjectTemplate, Quaternion, Vector3f, WorldID};
//use encoding::{all::UTF_16LE, DecoderTrap, Encoding};
use nom::{
    bytes::complete::take,
    combinator::{map, map_opt, map_res},
    error::FromExternalError,
    multi::length_data,
    sequence::tuple,
};
use nom::{
    error::ParseError,
    number::complete::{le_f32, le_u32, le_u8},
    IResult,
};
use num_traits::FromPrimitive;
use std::{char::decode_utf16, string::FromUtf8Error};

/// Helper method to dump some values
#[allow(dead_code)]
pub fn dump<T>(val: T) -> T
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
    val
}

type Res<'a, T, E> = IResult<&'a [u8], T, E>;

/// Parse a Vector3f
pub fn parse_vec3f<'a, E>(input: &'a [u8]) -> Res<'a, Vector3f, E>
where
    E: ParseError<&'a [u8]>,
{
    map(tuple((le_f32, le_f32, le_f32)), |(x, y, z)| {
        Vector3f::new(x, y, z)
    })(input)
}

/// Parse a Quaternion
pub fn parse_quat<'a, E>(input: &'a [u8]) -> Res<'a, Quaternion, E>
where
    E: ParseError<&'a [u8]>,
{
    map(tuple((le_f32, le_f32, le_f32, le_f32)), |(x, y, z, w)| {
        Quaternion::new(x, y, z, w)
    })(input)
}

/// Parse a Quaternion
pub fn parse_quat_wxyz<'a, E>(input: &'a [u8]) -> Res<'a, Quaternion, E>
where
    E: ParseError<&'a [u8]>,
{
    map(tuple((le_f32, le_f32, le_f32, le_f32)), |(w, x, y, z)| {
        Quaternion::new(x, y, z, w)
    })(input)
}

/// Parse a WorldID
pub fn parse_world_id<'a, E>(input: &'a [u8]) -> Res<'a, WorldID, E>
where
    E: ParseError<&'a [u8]>,
{
    map_opt(le_u32, WorldID::from_u32)(input)
}

/// Parse an ObjectTemplate
pub fn parse_object_template<'a, E>(input: &'a [u8]) -> Res<'a, ObjectTemplate, E>
where
    E: ParseError<&'a [u8]>,
{
    map_opt(le_u32, ObjectTemplate::from_u32)(input)
}

/// Parse an ObjectID
pub fn parse_object_id<'a, E>(input: &'a [u8]) -> Res<'a, ObjectID, E>
where
    E: ParseError<&'a [u8]>,
{
    map(tuple((le_u32, le_u32)), |(a, b)| ObjectID::new(b, a))(input)
}

fn map_wstring(val: &[u8]) -> Result<String, ()> {
    let iter = val.chunks_exact(2);
    if let [] = iter.remainder() {
        let iter = iter.map(|s| (s[0] as u16) + ((s[1] as u16) << 8));
        decode_utf16(iter).map(|r| r.or(Err(()))).collect()
    } else {
        Err(())
    }
}

/// Parse a u8 wstring
pub fn parse_u8_wstring<'a, E>(input: &'a [u8]) -> Res<'a, String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], ()>,
{
    le_u8(input).and_then(|(input, count)| {
        let len = usize::from(count) * 2;
        map_res(take(len), map_wstring)(input)
    })
}

/// Parse a u32 wstring
pub fn parse_u32_wstring<'a, E>(input: &'a [u8]) -> Res<'a, String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], ()>,
{
    le_u32(input).and_then(|(input, count)| {
        let len = count * 2;
        map_res(take(len), map_wstring)(input)
    })
}

/// Parse a string with u16 length specifier
pub fn parse_string_u16<'a, E>(input: &'a [u8], i: u16) -> Res<'a, String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], FromUtf8Error>,
{
    map_res(map(take(i), Vec::from), String::from_utf8)(input)
}

/// Parse a string after an u8 length specifier
pub fn parse_u8_string<'a, E>(input: &'a [u8]) -> Res<'a, String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], FromUtf8Error>,
{
    map_res(map(length_data(le_u8), Vec::from), String::from_utf8)(input)
}

/// Parse a string after an u32 length specifier
pub fn parse_u32_string<'a, E>(input: &'a [u8]) -> Res<'a, String, E>
where
    E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], FromUtf8Error>,
{
    map_res(map(length_data(le_u32), Vec::from), String::from_utf8)(input)
}

/// Parse an u32 boolean
pub fn parse_u32_bool<'a, E>(input: &'a [u8]) -> Res<'a, bool, E>
where
    E: ParseError<&'a [u8]>,
{
    map(le_u32, |v| v > 0)(input)
}

/// Parse an u8 boolean
pub fn parse_u8_bool<'a, E>(input: &'a [u8]) -> Res<'a, bool, E>
where
    E: ParseError<&'a [u8]>,
{
    map(le_u8, |v| v > 0)(input)
}

#[cfg(test)]
mod test {
    use super::parse_u8_wstring;
    use nom::error::ErrorKind;

    #[test]
    fn test_wstring() {
        assert_eq!(
            parse_u8_wstring::<'_, (&[u8], ErrorKind)>(&[2, 65, 0, 66, 0]),
            Ok((&[][..], String::from("AB")))
        );
    }
}
