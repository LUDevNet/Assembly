pub mod sink;

/// `obj`
#[derive(Default, Debug, PartialEq)]
pub struct Object {
    /// Version
    pub attr_v: u32,
    /// Buffs
    pub buff: Option<Buff>,
    /// Destructible Component
    pub dest: Option<Destructible>,
    /// Inventory Component
    pub inv: Option<Inventory>,
    /// Minifigure Component
    pub mf: Option<Minifig>,
}

/// Buff Component
#[derive(Default, Debug, PartialEq)]
pub struct Buff {}

/// `dest`
#[derive(Default, Debug, PartialEq)]
pub struct Destructible {
    /// Current Armor
    pub attr_ac: Option<u32>,
    /// Maximum Armor
    pub attr_am: Option<u32>,
    /// Object is Dead
    pub attr_d: Option<bool>,
    /// Health Current
    pub attr_hc: Option<u32>,
    /// Maximum Health
    pub attr_hm: Option<u32>,
    /// Current Imagination
    pub attr_ic: Option<u32>,
    /// Maximum Imagination
    pub attr_im: Option<u32>,
    /// Immunity
    pub attr_imm: Option<u32>,
    /// Respawn Health
    pub attr_rsh: Option<u32>,
    /// Respawn Imagination
    pub attr_rsi: Option<u32>,
}

/// The inventory component
#[derive(Default, Debug, PartialEq)]
pub struct Inventory {
    /// Consumable Slot LOT
    pub attr_csl: Option<u32>,
    /// Inventory 'Bags'
    pub bag: Vec<Bag>,
    /// Groups
    pub grps: Vec<Group>,
    /// Items
    pub items: Items,
}

/// One compartment in the inventory
#[derive(Default, Debug, PartialEq)]
pub struct Bag {
    /// Type
    pub attr_t: u32,
    /// Maximum
    pub attr_m: u32,
}

/// One group
#[derive(Default, Debug, PartialEq)]
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
#[derive(Default, Debug, PartialEq)]
pub struct Items {
    pub attr_nn: String,
    pub children: Vec<ItemBag>,
}

/// A list of items for a bag
#[derive(Default, Debug, PartialEq)]
pub struct ItemBag {
    pub attr_t: u32,
    pub children: Vec<Item>,
}

/// An item in an inventory
#[derive(Default, Debug, PartialEq)]
pub struct Item {
    /// Is Bound
    pub attr_b: bool,
    /// Count
    pub attr_c: u32,
    /// Is Equipped
    pub attr_eq: bool,
    /// Object ID
    pub attr_id: u64,
    /// LOT
    pub attr_l: u32,
    /// Slot
    pub attr_s: u32,
    /// Subkey
    pub attr_sk: u32,
    /// Extra Info
    pub x: Option<ItemExtra>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ItemExtra {
    pub attr_b: String,
    /// Module Assembly
    pub attr_ma: String,
    pub attr_ub: String,
    pub attr_ud: String,
    pub attr_ui: String,
    pub attr_um: String,
    /// UGC name?
    pub attr_un: String,
    pub attr_uo: String,
    pub attr_up: String,
}

#[derive(Default, Debug, PartialEq)]
/// Minifigure Component
pub struct Minifig {
    /// Chest Decal
    pub attr_cd: u32,
    /// Eyebrow Style
    pub attr_es: u32,
    /// Eye Style.
    pub attr_ess: u32,
    /// Hair Color
    pub attr_hc: u32,
    /// Head Style
    pub attr_hd: u32,
    /// Head Color
    pub attr_hdc: u32,
    /// Hair Style
    pub attr_hs: u32,
    /// Legs
    pub attr_l: u32,
    /// Left Hand
    pub attr_lh: u32,
    /// Mouth Style.
    pub attr_ms: u32,
    /// Right Hand
    pub attr_rh: u32,
    /// Chest
    pub attr_t: u32,
}
