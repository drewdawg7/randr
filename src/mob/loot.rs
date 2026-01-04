//! Loot-related trait implementations for Mob

use crate::loot::{HasLoot, LootTable};

use super::Mob;

impl HasLoot for Mob {
    fn loot(&self) -> &LootTable {
        &self.loot_table
    }
}
