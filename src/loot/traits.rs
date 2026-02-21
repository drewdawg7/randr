use crate::item::ItemRegistry;
use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    fn roll_drops(&self, magic_find: i32, registry: &ItemRegistry) -> Vec<LootDrop> {
        self.loot().roll_drops(magic_find, registry)
    }
}
