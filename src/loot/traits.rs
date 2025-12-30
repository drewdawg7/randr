use crate::loot::definition::LootTable;

pub trait HasLoot {
    fn loot(&self) -> &LootTable;
    fn loot_mut(&mut self) -> &mut LootTable;
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


