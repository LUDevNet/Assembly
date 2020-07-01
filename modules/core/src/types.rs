//! # The general types used all over the place
use derive_new::new;

/// Position in three dimensional space
#[derive(Copy, Clone, Debug, new)]
pub struct Vector3f {
    /// The X coordinate
    pub x: f32,
    /// The Y coordinate
    pub y: f32,
    /// The Z coordinate
    pub z: f32,
}

impl From<[f32; 3]> for Vector3f {
    fn from(val: [f32; 3]) -> Self {
        Vector3f {
            x: val[0],
            y: val[1],
            z: val[2],
        }
    }
}

impl Into<[f32; 3]> for Vector3f {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Rotation in three dimensional space
#[derive(Debug, new)]
pub struct Quaternion {
    /// The X component
    pub x: f32,
    /// The Y component
    pub y: f32,
    /// The Z component
    pub z: f32,
    /// The W component
    pub w: f32,
}

/// Position and rotation in three dimensional space
#[derive(Debug)]
pub struct Placement3D {
    /// The position
    pub pos: Vector3f,
    /// The rotation
    pub rot: Quaternion,
}

/// Alias for u32 that represents a world map from the resources
#[derive(Debug, Clone, FromPrimitive, ToPrimitive, PartialEq)]
pub struct WorldID(u32);

/// Alias for u32 for an object template id
#[derive(Debug, Clone, FromPrimitive, ToPrimitive, PartialEq)]
pub struct ObjectTemplate(u32);

/// Object ID
#[derive(Debug, Clone, new)]
pub struct ObjectID {
    /// The bitmask for the scope of this object
    pub scope: u32,
    /// The serial ID of this object
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
