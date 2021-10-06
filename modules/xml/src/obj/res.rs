//! Data for respawn positions

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Data for respawn positions (?)
pub struct Respawn {
    /// Respawn points
    #[serde(default, rename = "r")]
    pub children: Vec<RespawnPoint>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Single respawn point
pub struct RespawnPoint {
    /// World to which this entry applies
    ///
    /// Values: ID from the [`ZoneTable`](https://docs.lu-dev.net/en/latest/database/ZoneTable.html)
    #[serde(rename = "w")]
    pub world: u32,

    /// X-coordinate
    pub x: f32,
    /// Y-coordinate
    pub y: f32,
    /// Z-coordinate
    pub z: f32,
}
