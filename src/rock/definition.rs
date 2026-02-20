use crate::data::StatRange;
use crate::item::ItemId;
use crate::loot::LootTable;
use crate::rock::enums::RockType;

#[derive(Debug, Clone)]
pub struct Rock {
    pub rock_type: RockType,
    pub loot: LootTable,
}

impl Rock {
    pub fn new(rock_type: RockType) -> Self {
        let loot = match rock_type {
            RockType::Coal => LootTable::new().with(ItemId::Coal, 1, 1, StatRange(1, 2)).build(),
            RockType::Copper => LootTable::new().with(ItemId::CopperOre, 1, 1, StatRange(1, 3)).build(),
            RockType::Iron => LootTable::new().with(ItemId::IronOre, 1, 1, StatRange(1, 3)).build(),
            RockType::Gold => LootTable::new().with(ItemId::GoldOre, 1, 1, StatRange(1, 3)).build(),
        };

        Self { rock_type, loot }
    }
}
