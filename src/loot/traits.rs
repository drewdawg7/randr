use crate::loot::definition::LootTable;

pub trait HasLoot {
    fn loot(&self) -> &LootTable;
    fn loot_mut(&mut self) -> &mut LootTable;
}
