use nom::{le_f32, le_u32};
use num_traits::FromPrimitive;
use super::types::{Vector3f, Quaternion, WorldID};

// Helper method to dump some values
#[allow(dead_code)]
fn dump<T>(val: T) -> T
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
