use crate::item::{Item, ItemId};
use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops with Magic Find bonus and custom spawn function.
    ///
    /// The spawn function is required to make the dependency explicit.
    /// Use `|id| game_state().spawn_item(id)` for production code.
    fn roll_drops<F>(&self, magic_find: i32, spawn_item: F) -> Vec<LootDrop>
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        self.loot().roll_drops_with_spawner(magic_find, spawn_item)
    }
}
