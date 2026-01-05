use crate::item::{Item, ItemId};
use crate::loot::definition::{LootDrop, LootTable};

#[allow(dead_code)]
pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops with Magic Find bonus using direct item spawning.
    fn roll_drops(&self, magic_find: i32) -> Vec<LootDrop> {
        self.loot().roll_drops(magic_find)
    }

    /// Roll loot drops with Magic Find bonus and custom spawn function.
    ///
    /// This variant allows dependency injection for testing.
    /// For production code, prefer `roll_drops()` which uses `ItemId::spawn()` directly.
    fn roll_drops_with_spawner<F>(&self, magic_find: i32, spawn_item: F) -> Vec<LootDrop>
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        self.loot().roll_drops_with_spawner(magic_find, spawn_item)
    }
}
