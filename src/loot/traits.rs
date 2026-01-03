use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops and spawn items. Default implementation uses the loot table.
    fn roll_drops(&self) -> Vec<LootDrop> {
        self.loot().roll_drops()
    }
}
