use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops without Magic Find bonus.
    fn roll_drops(&self) -> Vec<LootDrop> {
        self.loot().roll_drops()
    }

    /// Roll loot drops with Magic Find bonus rolls.
    fn roll_drops_with_mf(&self, magic_find: i32) -> Vec<LootDrop> {
        self.loot().roll_drops_with_mf(magic_find)
    }
}
