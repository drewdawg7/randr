use crate::{chest::definition::Chest, loot::{HasLoot, LootTable}, ItemId};


impl HasLoot for Chest {
    fn loot(&self) -> &LootTable {
        &self.loot
    }
}


impl Default for Chest {
    fn default() -> Self {
        let loot = LootTable::new()
            .with(ItemId::GoldRing, 1, 3, 1..=1)
            .with(ItemId::BronzeChestplate, 1, 4, 1..=1)
            .with(ItemId::BronzeIngot, 1, 2, 4..=8)
            .with(ItemId::QualityUpgradeStone, 1, 3, 1..=2)
            .with(ItemId::BasicHPPotion, 1, 1, 3..=6);
        Self {
            loot,
            is_locked: false
        }
    }
}
