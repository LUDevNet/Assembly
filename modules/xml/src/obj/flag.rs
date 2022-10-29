//! ## Data for the [`PlayerFlags` component](https://docs.lu-dev.net/en/latest/components/058-player-flags.html)

use serde::{Deserialize, Serialize};

/// Data for the [`PlayerFlags` component](https://docs.lu-dev.net/en/latest/components/058-player-flags.html)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Flags {
    /// List of flags
    #[serde(rename = "f")]
    pub children: Vec<Flag>,
}

#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Batch of 64 adjacent player flags
pub struct Flag {
    /// ID (offset / 64)
    pub id: u32,
    /// Value of 64 flags
    #[serde(rename = "v")]
    pub value: u64,
}
