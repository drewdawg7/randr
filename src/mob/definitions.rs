use std::ops::RangeInclusive;

use crate::dungeon::GridSize;
use crate::item::ItemId;
use crate::loot::LootTable;
use crate::registry::RegistryDefaults;

pub use super::enums::MobQuality;

entity_macros::define_entity! {
    spec MobSpec {
        pub name: String,
        pub max_health: RangeInclusive<i32>,
        pub attack: RangeInclusive<i32>,
        pub defense: RangeInclusive<i32>,
        pub dropped_gold: RangeInclusive<i32>,
        pub dropped_xp: RangeInclusive<i32>,
        pub quality: MobQuality,
        pub loot: LootTable,
        pub grid_size: GridSize,
    }

    id MobId;

    variants {
        Slime {
            name: String::from("Slime"),
            quality: MobQuality::Normal,
            max_health: 15..=22,
            attack: 2..=4,
            defense: 1..=3,
            dropped_gold: 1..=3,
            dropped_xp: 5..=9,
            loot: LootTable::new()
                .with(ItemId::SlimeGel, 3, 4, 1..=4)
                .with(ItemId::GoldRing, 1, 100, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        Cow {
            name: String::from("Cow"),
            quality: MobQuality::Normal,
            max_health: 20..=25,
            attack: 1..=4,
            defense: 0..=2,
            dropped_gold: 1..=3,
            dropped_xp: 5..=9,
            loot: LootTable::new()
                .with(ItemId::Cowhide, 3, 4, 1..=3)
                .with(ItemId::GoldRing, 1, 1000, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        Goblin {
            name: String::from("Goblin"),
            quality: MobQuality::Normal,
            max_health: 33..=41,
            attack: 10..=15,
            defense: 5..=10,
            dropped_gold: 10..=19,
            dropped_xp: 13..=20,
            loot: LootTable::new()
                .with(ItemId::Sword, 1, 15, 1..=1)
                .with(ItemId::BasicShield, 1, 15, 1..=1)
                .with(ItemId::GoldRing, 1, 100, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        DwarfDefender {
            name: String::from("Dwarf Defender"),
            quality: MobQuality::Normal,
            max_health: 50..=65,
            attack: 12..=18,
            defense: 12..=18,
            dropped_gold: 15..=25,
            dropped_xp: 20..=30,
            loot: LootTable::new()
                .with(ItemId::IronOre, 2, 4, 1..=2)
                .with(ItemId::GoldOre, 1, 6, 1..=2)
                .with(ItemId::Coal, 2, 4, 1..=3)
                .with(ItemId::IronIngot, 1, 8, 1..=1)
                .with(ItemId::GoldIngot, 1, 12, 1..=1)
                .with(ItemId::CopperIngot, 1, 10, 1..=1)
                .with(ItemId::IronHelmet, 1, 20, 1..=1)
                .with(ItemId::IronChestplate, 1, 25, 1..=1)
                .with(ItemId::IronGauntlets, 1, 18, 1..=1)
                .with(ItemId::IronSword, 1, 15, 1..=1)
                .with(ItemId::CopperSword, 1, 20, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        DwarfWarrior {
            name: String::from("Dwarf Warrior"),
            quality: MobQuality::Normal,
            max_health: 40..=50,
            attack: 18..=25,
            defense: 8..=12,
            dropped_gold: 18..=30,
            dropped_xp: 22..=32,
            loot: LootTable::new()
                .with(ItemId::IronOre, 2, 4, 1..=3)
                .with(ItemId::Coal, 2, 4, 1..=2)
                .with(ItemId::IronIngot, 1, 6, 1..=2)
                .with(ItemId::CopperIngot, 1, 8, 1..=1)
                .with(ItemId::IronSword, 1, 10, 1..=1)
                .with(ItemId::CopperSword, 1, 12, 1..=1)
                .with(ItemId::GoldSword, 1, 15, 1..=1)
                .with(ItemId::IronGreaves, 1, 20, 1..=1)
                .with(ItemId::IronLeggings, 1, 22, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        DwarfMiner {
            name: String::from("Dwarf Miner"),
            quality: MobQuality::Normal,
            max_health: 30..=40,
            attack: 8..=14,
            defense: 6..=10,
            dropped_gold: 8..=16,
            dropped_xp: 12..=18,
            loot: LootTable::new()
                .with(ItemId::IronOre, 3, 3, 1..=3)
                .with(ItemId::GoldOre, 2, 4, 1..=2)
                .with(ItemId::Coal, 3, 3, 1..=4)
                .with(ItemId::IronIngot, 1, 10, 1..=1)
                .with(ItemId::GoldIngot, 1, 15, 1..=1)
                .with(ItemId::CopperIngot, 1, 12, 1..=1)
                .with(ItemId::CopperPickaxe, 1, 20, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        DwarfKing {
            name: String::from("Dwarf King"),
            quality: MobQuality::Normal,
            max_health: 80..=100,
            attack: 25..=35,
            defense: 20..=28,
            dropped_gold: 40..=60,
            dropped_xp: 50..=70,
            loot: LootTable::new()
                .with(ItemId::IronHelmet, 1, 6, 1..=1)
                .with(ItemId::IronChestplate, 1, 6, 1..=1)
                .with(ItemId::IronGauntlets, 1, 5, 1..=1)
                .with(ItemId::IronGreaves, 1, 5, 1..=1)
                .with(ItemId::IronLeggings, 1, 6, 1..=1)
                .with(ItemId::GoldHelmet, 1, 10, 1..=1)
                .with(ItemId::GoldChestplate, 1, 10, 1..=1)
                .with(ItemId::IronSword, 1, 5, 1..=1)
                .with(ItemId::CopperSword, 1, 6, 1..=1)
                .with(ItemId::GoldSword, 1, 8, 1..=1)
                .with(ItemId::IronIngot, 1, 4, 1..=2)
                .with(ItemId::GoldIngot, 1, 5, 1..=2)
                .with(ItemId::CopperIngot, 1, 4, 1..=2)
                .with(ItemId::IronOre, 1, 6, 1..=3)
                .with(ItemId::GoldOre, 1, 8, 1..=2)
                .with(ItemId::GoldRing, 1, 10, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        Dragon {
            name: String::from("Dragon"),
            quality: MobQuality::Boss,
            max_health: 500..=700,
            attack: 50..=70,
            defense: 30..=50,
            dropped_gold: 250..=350,
            dropped_xp: 500..=750,
            loot: LootTable::new()
                .with(ItemId::GoldRing, 1, 100, 1..=1)
                .with(ItemId::QualityUpgradeStone, 1, 1, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        BlackDragon {
            name: String::from("Black Dragon"),
            quality: MobQuality::Boss,
            max_health: 550..=750,
            attack: 55..=75,
            defense: 35..=55,
            dropped_gold: 300..=400,
            dropped_xp: 550..=800,
            loot: LootTable::new()
                .with(ItemId::GoldRing, 1, 100, 1..=1)
                .with(ItemId::QualityUpgradeStone, 1, 1, 1..=1)
                .build(),
            grid_size: GridSize::single(),
        }
        Merchant {
            name: String::from("Merchant"),
            quality: MobQuality::Normal,
            max_health: 1..=1,
            attack: 0..=0,
            defense: 0..=0,
            dropped_gold: 0..=0,
            dropped_xp: 0..=0,
            loot: LootTable::new().build(),
            grid_size: GridSize::single(),
        }
    }
}

impl MobSpec {
    pub fn with_multiplier(&self, multiplier: f32) -> MobSpec {
        let scale_range = |r: &RangeInclusive<i32>| {
            let start = (*r.start() as f32 * multiplier).round() as i32;
            let end = (*r.end() as f32 * multiplier).round() as i32;
            start..=end
        };

        MobSpec {
            name: self.name.clone(),
            max_health: scale_range(&self.max_health),
            attack: scale_range(&self.attack),
            defense: scale_range(&self.defense),
            dropped_gold: scale_range(&self.dropped_gold),
            dropped_xp: scale_range(&self.dropped_xp),
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            grid_size: self.grid_size,
        }
    }

    pub fn with_name(&self, name: impl Into<String>) -> MobSpec {
        MobSpec {
            name: name.into(),
            max_health: self.max_health.clone(),
            attack: self.attack.clone(),
            defense: self.defense.clone(),
            dropped_gold: self.dropped_gold.clone(),
            dropped_xp: self.dropped_xp.clone(),
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            grid_size: self.grid_size,
        }
    }

    pub fn with_quality(&self, quality: MobQuality) -> MobSpec {
        MobSpec {
            name: self.name.clone(),
            max_health: self.max_health.clone(),
            attack: self.attack.clone(),
            defense: self.defense.clone(),
            dropped_gold: self.dropped_gold.clone(),
            dropped_xp: self.dropped_xp.clone(),
            quality,
            loot: self.loot.clone(),
            grid_size: self.grid_size,
        }
    }
}

impl RegistryDefaults<MobId> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobId, Self)> {
        MobId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}
