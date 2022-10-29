//! ## Data for the [`Pet Control` component](https://docs.lu-dev.net/en/latest/components/034-pet-control.html)

use serde::{Deserialize, Serialize};

/// Data for the [`Pet Control` component](https://docs.lu-dev.net/en/latest/components/034-pet-control.html)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pets {
    /// List of pets
    #[serde(default, rename = "p")]
    pub children: Vec<Pet>,
}

/// A single pet
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pet {
    /// Pet ObjectID
    pub id: u64,
    /// Pet template (LOT)
    #[serde(rename = "l")]
    pub lot: u32,

    /// Moderation status (?)
    #[serde(rename = "m")]
    pub moderation_status: u8,

    /// Name of the pet
    #[serde(rename = "n")]
    pub name: String,

    /// ???
    pub t: u8,
}
