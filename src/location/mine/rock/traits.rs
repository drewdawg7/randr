use crate::{
    combat::{IsKillable, RockDeathResult},
    item::{Item, ItemId},
    loot::{HasLoot, LootTable},
    stats::{HasStats, StatSheet},
};

use super::Rock;

impl HasLoot for Rock {
    fn loot(&self) -> &LootTable {
        &self.loot
    }
}

impl HasStats for Rock {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl IsKillable for Rock {
    type DeathResult = RockDeathResult;

    fn on_death<F>(&mut self, magic_find: i32, spawn_item: F) -> RockDeathResult
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        RockDeathResult {
            drops: self.roll_drops(magic_find, spawn_item),
        }
    }
}
