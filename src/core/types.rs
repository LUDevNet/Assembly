/// Position in three dimensional space
#[derive(Copy, Clone, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32;3]> for Vector3f {
    fn from(val: [f32;3]) -> Self {
        Vector3f{x: val[0], y: val[1], z: val[2]}
    }
}

impl Into<[f32;3]> for Vector3f {
    fn into(self) -> [f32;3] {
        [self.x, self.y, self.z]
    }
}

/// Rotation in three dimensional space
#[derive(Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Position and rotation in three dimensional space
#[derive(Debug)]
pub struct Placement3D {
    pub pos: Vector3f,
    pub rot: Quaternion,
}

/// Alias for u32 that represents a world map from the resources
#[derive(Debug, Clone, FromPrimitive, ToPrimitive, PartialEq)]
pub struct WorldID(u32);

/// Alias for u32 for an object template id
#[derive(Debug, Clone, FromPrimitive, ToPrimitive, PartialEq)]
pub struct ObjectTemplate(u32);

/// Object ID
#[derive(Debug, Clone)]
pub struct ObjectID {
    pub scope: u32,
    pub id: u32,
}

#[cfg(test)]
mod test {
    use super::{ObjectTemplate, WorldID};
    use num_traits::FromPrimitive;

    #[test]
    fn test_newtypes() {
        assert_eq!(ObjectTemplate::from_u32(100), Some(ObjectTemplate(100)));
        assert_eq!(WorldID::from_u32(1001), Some(WorldID(1001)));
    }
}
