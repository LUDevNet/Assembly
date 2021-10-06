//! ## Data for the [`Character` component](https://docs.lu-dev.net/en/latest/components/004-character.html)
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Data for the [`Character` component][c004]
///
/// [c004]: https://docs.lu-dev.net/en/latest/components/004-character.html
pub struct Character {
    /// Account ID
    #[serde(rename = "acct")]
    account: u32,

    /// Current amount of currency
    #[serde(rename = "cc")]
    currency_current: u32,

    /// GM level
    #[serde(rename = "gm")]
    gm_level: u32,

    /// FreeToPlay status
    #[serde(rename = "ft")]
    free_to_play: u32,

    /// Timestamp of last login as this character
    #[serde(rename = "llog")]
    last_login: u64,

    /// LEGO score / Uscore
    #[serde(rename = "ls")]
    lego_score: u32,

    /// Last world position X-coordinate
    lzx: f32,
    /// Last world position Y-coordinate
    lzy: f32,
    /// Last world position Z-coordinate
    lzz: f32,

    /// Last world rotation X component
    lzrx: f32,
    /// Last world rotation Y component
    lzry: f32,
    /// Last world rotation Z component
    lzrz: f32,
    /// Last world rotation W component
    lzrw: f32,

    /// Player stats
    stt: String,

    /// Last zone ID (packed)
    lzid: u32,
    /// ???
    lnzid: u32,
    /// Last world ID
    lwid: u32,

    /// ???
    tscene: String,
    /// ???
    lrid: u64,

    /// Total time played, in seconds
    time: u32,

    /// Unlocked emotes
    #[serde(rename = "ue")]
    pub unlocked_emotes: UnlockedEmotes,

    /// Zone summaries
    #[serde(default, rename = "vl")]
    pub visited_levels: VisitedLevels,

    /// Zone summaries
    #[serde(rename = "zs")]
    pub zone_summaries: ZoneSummaries,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// Unlocked emotes
pub struct UnlockedEmotes {
    /// List of unlocked emotes
    #[serde(rename = "e")]
    pub children: Vec<UnlockedEmote>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// A single unlocked emote
pub struct UnlockedEmote {
    /// The ID from the [`Emotes` tables](https://docs.lu-dev.net/en/latest/database/Emotes.html)
    pub id: u32,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// List of zone summaries
pub struct ZoneSummaries {
    /// The list of summaries
    #[serde(rename = "s")]
    pub children: Vec<ZoneSummary>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// A single zone summary
pub struct ZoneSummary {
    /// The relevant map ID from the [`ZoneTable`](https://docs.lu-dev.net/en/latest/database/ZoneTable.html)
    map: u32,
    /// Number of achievements
    #[serde(rename = "ac")]
    pub achievement_count: u32,
    /// Number of bricks collected
    #[serde(rename = "bc")]
    pub bricks_collected: u32,
    /// Number of coins collected
    #[serde(rename = "cc")]
    pub coins_collected: u32,
    /// Number of enemies smashed
    #[serde(rename = "es")]
    pub enemies_smashed: u32,
    /// Number of quick-builds constructed
    #[serde(rename = "qbc")]
    pub quick_builds_constructed: u32,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// List of zone summaries
pub struct VisitedLevels {
    /// The list of summaries
    #[serde(rename = "v")]
    pub children: Vec<VisitedLevel>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
/// A level the player visited
pub struct VisitedLevel {
    /// Clone ID (used for properties, 0 if not a property)
    #[serde(rename = "cid")]
    clone_id: u32,
    /// World ID.
    id: u32,
}
