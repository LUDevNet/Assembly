//! ## Data for the [`Destructible` component](https://docs.lu-dev.net/en/latest/components/007-destroyable.html)
use serde::{Deserialize, Serialize};

/// Data for the [`Destructible` component](https://docs.lu-dev.net/en/latest/components/007-destroyable.html)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Destructible {
    /// Current Armor
    #[serde(rename = "ac")]
    pub armor_current: u32,
    /// Maximum Armor
    #[serde(rename = "am")]
    pub armor_max: u32,
    /// Object is Dead
    #[serde(rename = "d")]
    pub dead: bool,
    /// Health Current
    #[serde(rename = "hc")]
    pub health_current: u32,
    /// Maximum Health
    #[serde(rename = "hm")]
    pub health_max: u32,
    /// Current Imagination
    #[serde(rename = "ic")]
    pub imagination_current: u32,
    /// Maximum Imagination
    #[serde(rename = "im")]
    pub imagination_max: u32,
    /// Immunity
    #[serde(rename = "imm")]
    pub immunity: Option<u32>,
    /// Respawn Health
    #[serde(rename = "rsh")]
    pub respawn_health: Option<u32>,
    /// Respawn Imagination
    #[serde(rename = "rsi")]
    pub respawn_imagination: Option<u32>,

    /// Buffs
    pub buff: Option<Buff>,
}

/// Buff Component
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Buff {}
