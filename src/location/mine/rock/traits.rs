use crate::{
    combat::{IsKillable, RockDeathResult},
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

    fn on_death(&mut self, magic_find: i32) -> RockDeathResult {
        RockDeathResult {
            drops: self.roll_drops_with_mf(magic_find),
        }
    }
}
