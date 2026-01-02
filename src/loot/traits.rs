use crate::loot::definition::{LootDrop, LootTable};

pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    /// Roll loot drops and spawn items. Default implementation uses the loot table.
    fn roll_drops(&self) -> Vec<LootDrop> {
        self.loot().roll_drops()
    }
}

pub trait WorthGold {
    fn gold_value(&self) -> i32;
    fn purchase_price(&self) -> i32 {
        self.gold_value()
    }
    fn sell_price(&self) -> i32 {
        self.gold_value() / 2
    }
}


