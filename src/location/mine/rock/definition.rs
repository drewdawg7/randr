use crate::{
    combat::IsKillable,
    loot::{LootDrop, LootTable},
    stats::StatSheet,
};

use super::enums::RockId;

#[derive(Clone)]
pub struct Rock {
    pub rock_id: RockId,
    pub stats: StatSheet,
    pub loot: LootTable,
}

impl Rock {
    /// Mine this rock. Returns drops if rock was destroyed, None otherwise.
    pub fn mine(&mut self, damage: i32, magic_find: i32) -> Option<Vec<LootDrop>> {
        self.take_damage(damage);
        if !self.is_alive() {
            Some(self.on_death(magic_find).drops)
        } else {
            None
        }
    }
}
