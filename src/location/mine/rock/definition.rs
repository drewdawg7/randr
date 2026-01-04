use crate::{
    combat::IsKillable,
    item::{Item, ItemId},
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
    ///
    /// The `spawn_item` function is used to create items from the loot table.
    /// Pass `|id| game_state().spawn_item(id)` for production use.
    pub fn mine<F>(&mut self, damage: i32, magic_find: i32, spawn_item: F) -> Option<Vec<LootDrop>>
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        self.take_damage(damage);
        if !self.is_alive() {
            Some(self.on_death(magic_find, spawn_item).drops)
        } else {
            None
        }
    }
}
