// General Item Properties:
// Rarity, Tradability, Item Type ID, Instance ID, Name, Description


#[derive(Clone, Debug)]
pub struct Equipment {
    pub item_id: u32,          // unique identifier of each item
    pub item_instance_id: u32, // unique identifier of each instance of an item, used to be able to track and revert items that may have been duped

    pub durability: u32,
    pub max_durability: u32,

    pub equippable_slot: ItemSlot,
    pub rarity: Rarity,
}

#[derive(Clone, Debug)]
pub enum ItemSlot {
    Head,
    Torso,
    Legs,
    Feet,

    Weapon, // might not use this TODO!()
    RingL, // Left hand
    RingR, // Right hand
}

/**
Rarity is a quick identify of the quality/power of an item without reading it's stats.
Higher rarity items may have certain benefits over lower rarities such as the ability to have more modifications or the ability to repair it.

*/
#[derive(Clone, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Elite,
    Legendary,
    Divine, // ban hammer lol
            // Exotic / Unique / Divine (one of a kind/limited availability)
}
