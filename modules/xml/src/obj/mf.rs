//! ## Data for the [`Minifig` component](https://docs.lu-dev.net/en/latest/components/035-minifig.html)

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Data for the [`Minifig` component](https://docs.lu-dev.net/en/latest/components/035-minifig.html)
pub struct Minifig {
    /// Chest Decal
    #[serde(rename = "cd")]
    pub chest_decal: u32,
    /// Eyebrow Style
    #[serde(rename = "es")]
    pub eyebrow_style: u32,
    /// Eye Style.
    #[serde(rename = "ess")]
    pub eyes_style: u32,
    /// Hair Color
    #[serde(rename = "hc")]
    pub hair_color: u32,
    /// Head Style
    #[serde(rename = "hd")]
    pub head_style: u32,
    /// Head Color
    #[serde(rename = "hdc")]
    pub head_color: u32,
    /// Hair Style
    #[serde(rename = "hs")]
    pub hair_style: u32,
    /// Legs
    #[serde(rename = "l")]
    pub legs: u32,
    /// Left Hand
    #[serde(rename = "lh")]
    pub left_hand: u32,
    /// Mouth Style.
    #[serde(rename = "ms")]
    pub mouth_style: u32,
    /// Right Hand
    #[serde(rename = "rh")]
    pub right_hand: u32,
    /// Torso
    #[serde(rename = "t")]
    pub torso: u32,
}
