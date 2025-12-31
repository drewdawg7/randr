use crate::item::{Item, ItemId};

#[derive(Debug, Clone)]
pub struct AttackResult {
    pub attacker: String,
    pub defender: String,
    pub damage_to_target: i32,
    pub target_health_before: i32,
    pub target_health_after: i32,
    pub target_died: bool,
}

/// Result of a mob dying - contains rewards for the killer
#[derive(Debug, Clone, Default)]
pub struct MobDeathResult {
    pub gold_dropped: i32,
    pub xp_dropped: i32,
    pub loot_drops: Vec<(ItemId, i32)>, // (item_id, quantity)
}

/// Result of player dying
#[derive(Debug, Clone, Default)]
pub struct PlayerDeathResult {
    pub gold_lost: i32,
}

/// Result of a rock being destroyed - contains mined items
#[derive(Debug, Clone, Default)]
pub struct RockDeathResult {
    pub drops: Vec<Item>,
}
