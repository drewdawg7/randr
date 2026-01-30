use crate::item::ItemId;
use crate::loot::LootTable;
use crate::stats::{HasStats, StatSheet, StatType};

entity_macros::define_entity! {
    spec RockSpec {
        pub name: &'static str,
        pub health: i32,
        pub loot: LootTable,
    }

    id RockId;

    variants {
        Iron {
            name: "Iron Rock",
            health: 50,
            loot: LootTable::new()
                .with(ItemId::IronOre, 1, 1, 1..=3)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1)
                .build(),
        }
        Coal {
            name: "Coal Rock",
            health: 50,
            loot: LootTable::new()
                .with(ItemId::Coal, 1, 1, 1..=2)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1)
                .build(),
        }
        Gold {
            name: "Gold Rock",
            health: 50,
            loot: LootTable::new()
                .with(ItemId::GoldOre, 1, 1, 1..=3)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1)
                .build(),
        }
        Mixed {
            name: "Mixed Rock",
            health: 100,
            loot: LootTable::new()
                .with(ItemId::GoldOre, 1, 2, 1..=4)
                .with(ItemId::IronOre, 1, 2, 1..=4)
                .with(ItemId::Coal, 1, 2, 1..=4)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1)
                .build(),
        }
    }
}

#[derive(Clone)]
pub struct MineRock {
    pub rock_id: RockId,
    pub stats: StatSheet,
    pub loot: LootTable,
}

impl HasStats for MineRock {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl RockId {
    pub fn spawn(&self) -> MineRock {
        let spec = self.spec();
        MineRock {
            rock_id: *self,
            stats: StatSheet::new().with(StatType::Health, spec.health),
            loot: spec.loot.clone(),
        }
    }
}
