use crate::loot::{HasLoot, LootTable};
use crate::rock::definition::Rock;

impl HasLoot for Rock {
    fn loot(&self) -> &LootTable {
        &self.loot
    }
}
