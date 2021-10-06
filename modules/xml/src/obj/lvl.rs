//! ## Data for the [`LevelProgression` component](https://docs.lu-dev.net/en/latest/components/xxx-level-progression.html)

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Data for the [`LevelProgression` component](https://docs.lu-dev.net/en/latest/components/xxx-level-progression.html)
#[allow(missing_docs)]
pub struct Level {
    /// Base Player Level
    #[serde(rename = "l")]
    pub level: u32,
    pub cv: u32,
    pub sb: u32,
}
