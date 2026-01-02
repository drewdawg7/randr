use crate::{
    entities::mob::enums::{MobId, MobQuality},
    loot::LootTable,
    stats::{HasStats, StatSheet},
};

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobId,
    pub quality: MobQuality,
    pub name: &'static str,
    pub stats: StatSheet,
    pub gold: i32,
    pub dropped_xp: i32,
    pub loot_table: LootTable,
}

