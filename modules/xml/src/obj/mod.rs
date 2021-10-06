//! # The XML `<obj>` format (character data)
//!
//! This module contains reader and writer and datastructures for
//! the character data as serialized to XML.

use serde::{Deserialize, Serialize};

use self::{
    char::Character, dest::Destructible, flag::Flags, inv::Inventory, lvl::Level, mf::Minifig,
    mis::Missions, pet::Pets, res::Respawn,
};

/// Data for a (player) object `<obj>`
#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "obj")]
pub struct Object {
    /// Version
    #[serde(rename = "v")]
    pub version: u32,
    /// Minifigure Component
    #[serde(rename = "mf")]
    pub minifig: Minifig,
    /// Character
    #[serde(rename = "char")]
    pub character: Character,
    /// Destructible Component
    #[serde(rename = "dest")]
    pub destructible: Destructible,
    /// Inventory Component
    #[serde(rename = "inv")]
    pub inventory: Inventory,
    /// Level Progression
    #[serde(rename = "lvl")]
    pub level: Level,
    /// Flags
    #[serde(rename = "flag")]
    pub flags: Flags,
    /// Respawn points
    #[serde(rename = "res")]
    pub respawn: Respawn,
    /// Missions
    #[serde(rename = "mis")]
    pub missions: Missions,
    /// Pets
    #[serde(rename = "pet")]
    pub pets: Pets,
}

pub mod char;
pub mod dest;
pub mod flag;
pub mod inv;
pub mod lvl;
pub mod mf;
pub mod mis;
pub mod pet;
pub mod res;
