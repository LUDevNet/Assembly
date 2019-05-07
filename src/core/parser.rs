use nom::{le_f32, le_u32, le_u8};
use num_traits::FromPrimitive;
use encoding::{Encoding, DecoderTrap, all::UTF_16LE};
use super::types::{Vector3f, Quaternion, WorldID, ObjectID, ObjectTemplate};

// Helper method to dump some values
#[allow(dead_code)]
pub fn dump<T>(val: T) -> T
where T: std::fmt::Debug {
    println!("{:?}", val);
    val
}

named!(pub parse_vec3f<Vector3f>,
    do_parse!(
        a: le_f32 >>
        b: le_f32 >>
        c: le_f32 >>
        (Vector3f{x: a, y: b, z: c})
    )
);

named!(pub parse_quat<Quaternion>,
    do_parse!(
        a: le_f32 >>
        b: le_f32 >>
        c: le_f32 >>
        d: le_f32 >>
        (Quaternion{x: a, y: b, z: c, w: d})
    )
);

named!(pub parse_world_id<WorldID>,
    map_opt!(le_u32, WorldID::from_u32)
);

named!(pub parse_object_template<ObjectTemplate>,
    map_opt!(le_u32, ObjectTemplate::from_u32)
);

named!(pub parse_object_id<ObjectID>,
    do_parse!(
        id: le_u32 >>
        scope: le_u32 >>
        (ObjectID{scope, id})
    )
);

fn map_wstring(val: &[u8]) -> Result<String,std::borrow::Cow<'static, str>> {
    UTF_16LE.decode(val, DecoderTrap::Strict)
}

named!(pub parse_u8_wstring<String>,
    do_parse!(
        count: le_u8 >>
        string: map_res!(take!(usize::from(count) * 2), map_wstring) >>
        (string)
    )
);

named!(pub parse_u32_wstring<String>,
    do_parse!(
        count: le_u32 >>
        string: map_res!(take!(count * 2), map_wstring) >>
        (string)
    )
);

named_args!(pub parse_string_u16(i: u16)<String>,
    map_res!(map!(length_bytes!(value!(i)), Vec::from), String::from_utf8)
);

named!(pub parse_u8_string<String>,
    map_res!(map!(length_bytes!(le_u8), Vec::from), String::from_utf8)
);

named!(pub parse_u32_string<String>,
    map_res!(map!(length_bytes!(le_u32), Vec::from), String::from_utf8)
);

named!(pub parse_u32_bool<bool>,
    alt!(value!(false, tag!([0; 4])) | value!(true, le_u32))
);

named!(pub parse_u8_bool<bool>,
    alt!(value!(false, tag!([0; 1])) | value!(true, le_u8))
);

#[cfg(test)]
mod test {
    use super::parse_u8_wstring;

    #[test]
    fn test_wstring() {
        assert_eq!(parse_u8_wstring(&[2,65,0,66,0]), Ok((&[][..], String::from("AB"))));
    }
}
