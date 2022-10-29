//! ## Data for the [`Inventory` component](https://docs.lu-dev.net/en/latest/components/017-inventory.html)

use serde::{Deserialize, Serialize};

/// Data for the [`Inventory` component](https://docs.lu-dev.net/en/latest/components/017-inventory.html)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Inventory {
    /// LOT of the item in the consumable slot
    #[serde(rename = "csl")]
    pub consumable_slot_lot: i32,
    /// Inventory 'Bags'
    pub bag: Bags,
    /// Groups
    #[serde(default)]
    pub grps: Vec<Group>,
    /// Items
    pub items: Items,
}

#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A list of bags
pub struct Bags {
    #[serde(rename = "b")]
    /// List of bags
    children: Vec<Bag>,
}

/// A storage container
///
/// (e.g Items, Models, Vault Items, Behaviors)
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bag {
    /// Type of the bag. See `InventoryType` enum for values.
    #[serde(rename = "t")]
    pub ty: u32,
    /// Size of the bag i.e. Amount of slots
    #[serde(rename = "m")]
    pub max: u32,
}

/// One group
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Group {
    /// `user_group XXX`
    pub attr_id: String,
    /// Space-separated LOTs
    pub attr_l: String,
    /// Name
    pub attr_n: String,
    /// Type of the group
    pub attr_t: u32,
    /// Unknown
    pub attr_u: String,
}

/// All items
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Items {
    /// ??
    #[serde(rename = "nn")]
    pub attr_nn: Option<String>,

    /// Items in the storage container
    #[serde(rename = "in")]
    pub children: Vec<ItemBag>,
}

/// A list of items for a storage container
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemBag {
    /// Type of the bag. See `InventoryType` enum for values.
    #[serde(rename = "t")]
    pub ty: u32,
    /// Items in the bag
    #[serde(default, rename = "i")]
    pub children: Vec<Item>,
}

/// An item in an inventory
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Item {
    /// Whether the item is bound
    #[serde(rename = "b")]
    pub bound: bool,
    /// Amount of items for stackable items.
    #[serde(rename = "c")]
    pub count: u32,
    /// Boolean whether the item is equipped.
    ///
    /// If it isn’t, this attribute isn’t there at all, if it is, it’s set to 1.
    #[serde(default, rename = "eq")]
    pub equipped: bool,
    /// Object ID
    #[serde(rename = "id")]
    pub id: u64,
    /// LOT
    #[serde(rename = "l")]
    pub template: u32,
    /// Slot
    #[serde(rename = "s")]
    pub slot: u32,
    /// Some kind of ID for models. Investigate. Referred to by client strings as “subkey”?
    #[serde(rename = "sk")]
    pub subkey: u64,
    /// Extra Info
    #[serde(rename = "x")]
    pub x: Option<ItemExtra>,
}

/// Extra item information
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(missing_docs)]
pub struct ItemExtra {
    #[serde(rename = "b")]
    pub attr_b: Option<String>,
    /// Module Assembly
    #[serde(rename = "ma")]
    pub module_assembly: String,
    #[serde(rename = "ub")]
    pub attr_ub: Option<String>,
    #[serde(rename = "ud")]
    pub attr_ud: Option<String>,
    #[serde(rename = "ui")]
    pub attr_ui: Option<String>,
    #[serde(rename = "um")]
    pub attr_um: Option<String>,
    /// UGC name?
    #[serde(rename = "un")]
    pub attr_un: Option<String>,
    #[serde(rename = "uo")]
    pub attr_uo: Option<String>,
    #[serde(rename = "up")]
    pub attr_up: Option<String>,
}
