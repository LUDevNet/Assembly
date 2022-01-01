//! # The XML `<Behavior>` format
//!
//! This is used in:
//! - The `ObjectBehaviors` table

use serde::{Deserialize, Serialize};

mod element {
    use std::{fmt, str::FromStr};

    use serde::{
        de::Deserializer,
        ser::{SerializeMap, Serializer},
        Deserialize, Serialize,
    };

    #[derive(Serialize, Deserialize)]
    struct Element<T> {
        #[serde(rename = "$value")]
        value: T,
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T: FromStr>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T::Err: fmt::Display,
    {
        Element::<String>::deserialize(deserializer).and_then(|e| {
            e.value
                .parse()
                .map_err(|e: T::Err| <D::Error as serde::de::Error>::custom(e.to_string()))
        })
    }

    pub fn serialize<S: Serializer, T: Serialize>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("$value", value)?;
        map.end()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One model behavior
pub struct Behavior {
    /// Version 1.0
    version: String,

    /// Name of this behavior
    #[serde(with = "element", rename = "Name")]
    name: String,

    /// Actions associated with the behavior
    #[serde(rename = "CompoundAction")]
    actions: Vec<CompoundAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A group of actions
pub struct CompoundAction {
    /// The actions for this compound
    #[serde(rename = "Action")]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type", rename_all = "PascalCase")]
/// A single action
pub enum Action {
    /// When the object is attacked
    OnAttack,
    /// When a player comes close
    OnEnterProximity {
        /// Distance to trigger
        #[serde(with = "element", rename = "Distance")]
        distance: f32,
    },
    /// When receiving a chat message
    OnChat {
        /// Message to post to chat
        #[serde(with = "element", rename = "Message")]
        message: String,
    },
    /// Play a sound
    PlaySound {
        /// GUID (`{...}`) of the audio
        #[serde(with = "element")]
        sound: String,
    },
    /// Send a chat message
    Chat {
        /// Message to post to chat
        #[serde(with = "element", rename = "Message")]
        message: String,
    },
    /// Smash the object
    Smash {
        /// Unclear
        #[serde(with = "element", rename = "Opacity")]
        opacity: f32,
        /// Unclear
        #[serde(with = "element")]
        force: f32,
    },
    /// Wait for some time
    Wait {
        #[serde(with = "element", rename = "Delay")]
        /// The amount of time to wait
        delay: f32,
    },
    /// Revert a smashing
    UnSmash {
        #[serde(with = "element", rename = "Duration")]
        /// Unclear
        duration: f32,
    },
    /// Move to the interactor
    MoveToInteractor {
        #[serde(with = "element", rename = "Distance")]
        /// Unclear
        distance: f32,
    },
}

#[cfg(test)]
mod tests {
    use super::Behavior;

    #[test]
    fn test_deserialize() {
        let text = include_str!("../test/Droid.xml");
        let _data: Behavior = quick_xml::de::from_str(text).unwrap();
    }
}
