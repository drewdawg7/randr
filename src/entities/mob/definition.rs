use crate::{
    entities::mob::enums::{MobKind, MobQuality},
    loot::LootTable,
    stats::{HasStats, StatSheet},
};

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub quality: MobQuality,
    pub name: &'static str,
    pub stats: StatSheet,
    pub gold: i32,
    pub dropped_xp: i32,
    pub loot_table: LootTable,
}

impl Mob {
    pub fn get_health(&self) -> i32 {
        self.hp()
    }

    pub fn get_attack(&self) -> i32 {
        self.attack()
    }

    pub fn get_max_health(&self) -> i32 {
        self.max_hp()
    }
}
