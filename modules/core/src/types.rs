//! # The general types used all over the place

#[cfg(feature = "serde-derives")]
use serde::Serialize;

/// Position in three dimensional space
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Vector3f {
    /// The X coordinate
    pub x: f32,
    /// The Y coordinate
    pub y: f32,
    /// The Z coordinate
    pub z: f32,
}

impl Vector3f {
    /// Create a new Vector of floats given `x`, `y`, and `z`
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
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

impl From<Vector3f> for [f32; 3] {
    fn from(v: Vector3f) -> [f32; 3] {
        [v.x, v.y, v.z]
    }
}

/// Rotation in three dimensional space
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
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

impl Quaternion {
    /// Create a new Quaternion given `x`, `y`, `z`, and `w`
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

/// Position and rotation in three dimensional space
#[derive(Debug)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct Placement3D {
    /// The position
    pub pos: Vector3f,
    /// The rotation
    pub rot: Quaternion,
}

/// Alias for u32 that represents a world map from the resources
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct WorldID(u32);

impl WorldID {
    /// Create a new world ID from an u32
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// Alias for u32 for an object template id
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct ObjectTemplate(u32);

impl ObjectTemplate {
    /// Create a new LOT from an object template
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// Object ID
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derives", derive(Serialize))]
pub struct ObjectID {
    /// The bitmask for the scope of this object
    pub scope: u32,
    /// The serial ID of this object
    pub id: u32,
}

impl ObjectID {
    /// Create a new ObjectID with a scope and flags
    pub fn new(scope: u32, id: u32) -> Self {
        Self { scope, id }
    }
}
