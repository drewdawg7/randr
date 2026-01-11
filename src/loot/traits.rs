use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops with Magic Find bonus using direct item spawning.
    fn roll_drops(&self, magic_find: i32) -> Vec<LootDrop> {
        self.loot().roll_drops(magic_find)
    }
}
