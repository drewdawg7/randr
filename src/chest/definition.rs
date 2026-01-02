use crate::loot::LootTable;

#[derive(Debug)]
pub struct Chest {
    pub loot: LootTable,
    pub is_locked: bool,
}


