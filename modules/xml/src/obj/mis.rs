//! ## Data for the [`Mission` component](https://docs.lu-dev.net/en/latest/components/084-mission.html)

use serde::{Deserialize, Serialize};

/// Data for the [`Mission` component](https://docs.lu-dev.net/en/latest/components/084-mission.html)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Missions {
    /// Completed missions
    pub done: MissionList,
    /// Currently active missions
    #[serde(rename = "cur")]
    pub current: MissionList,
}

#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A list of missions
pub struct MissionList {
    /// List of missions
    #[serde(rename = "m")]
    pub missions: Vec<Mission>,
}

/// A single mission
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mission {
    /// State of the mission
    state: u8, // FIXME: DLU specific?

    /// ID from the [`Missions` table](https://docs.lu-dev.net/en/latest/database/Missions.html)
    id: u32,
    /// Amount of times completed (Can be more than 1 for repeatable missions)
    #[serde(default, rename = "cct")]
    completion_count: u32,
    /// Timestamp of last completion in seconds.
    #[serde(rename = "cts")]
    completion_time: Option<u64>,

    #[serde(default, rename = "sv")]
    /// For achievements like collecting flags, there is one of this that has the displayed
    /// progress N, and N other `<sv>` elements that seem to have a bitflag in the id?
    sub_value: Vec<MissionSubValue>,
}

/// Progress for a task
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MissionSubValue {
    /// Value of the progress.
    #[serde(rename = "v")]
    value: u32,
}
