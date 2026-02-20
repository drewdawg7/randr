use crate::data::StatRange;
use crate::{chest::definition::Chest, loot::{HasLoot, LootTable}, ItemId};


impl HasLoot for Chest {
    fn loot(&self) -> &LootTable {
        &self.loot
    }
}


impl Default for Chest {
    fn default() -> Self {
        let loot = LootTable::new()
            .with(ItemId::GoldRing, 1, 3, StatRange(1, 1))
            .with(ItemId::CopperChestplate, 1, 4, StatRange(1, 1))
            .with(ItemId::CopperIngot, 1, 2, StatRange(4, 8))
            .with(ItemId::QualityUpgradeStone, 1, 3, StatRange(1, 2))
            .with(ItemId::BasicHPPotion, 1, 1, StatRange(3, 6))
            .build();
        Self {
            loot,
            is_locked: false
        }
    }
}
