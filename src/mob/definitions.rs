use std::ops::RangeInclusive;

use crate::dungeon::EntitySize;
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
        pub entity_size: EntitySize,
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: EntitySize::default(),
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
            entity_size: self.entity_size,
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
            entity_size: self.entity_size,
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
            entity_size: self.entity_size,
        }
    }
}

impl RegistryDefaults<MobId> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobId, Self)> {
        MobId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mob_id_spec_returns_valid_spec() {
        let spec = MobId::Goblin.spec();
        assert_eq!(spec.name, "Goblin");
    }

    #[test]
    fn mob_spec_with_multiplier_scales_stats() {
        let base = MobId::Goblin.spec();
        let scaled = base.with_multiplier(2.0);

        assert_eq!(*scaled.max_health.start(), (*base.max_health.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.max_health.end(), (*base.max_health.end() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.attack.start(), (*base.attack.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.attack.end(), (*base.attack.end() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.defense.start(), (*base.defense.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.defense.end(), (*base.defense.end() as f32 * 2.0).round() as i32);
    }

    #[test]
    fn mob_spec_with_multiplier_preserves_name() {
        let base = MobId::Goblin.spec();
        let scaled = base.with_multiplier(1.5);
        assert_eq!(scaled.name, "Goblin");
    }

    #[test]
    fn mob_spec_with_multiplier_preserves_quality() {
        let base = MobId::Dragon.spec();
        let scaled = base.with_multiplier(1.5);
        assert!(matches!(scaled.quality, MobQuality::Boss));
    }

    #[test]
    fn mob_spec_with_name_changes_name() {
        let base = MobId::Goblin.spec();
        let renamed = base.with_name("Elite Goblin");
        assert_eq!(renamed.name, "Elite Goblin");
    }

    #[test]
    fn mob_spec_with_name_preserves_stats() {
        let base = MobId::Goblin.spec();
        let renamed = base.with_name("Elite Goblin");
        assert_eq!(renamed.max_health, base.max_health);
        assert_eq!(renamed.attack, base.attack);
        assert_eq!(renamed.defense, base.defense);
    }

    #[test]
    fn mob_spec_with_quality_changes_quality() {
        let base = MobId::Goblin.spec();
        let boss = base.with_quality(MobQuality::Boss);
        assert!(matches!(boss.quality, MobQuality::Boss));
    }

    #[test]
    fn mob_spec_with_quality_preserves_stats() {
        let base = MobId::Goblin.spec();
        let boss = base.with_quality(MobQuality::Boss);
        assert_eq!(boss.max_health, base.max_health);
        assert_eq!(boss.attack, base.attack);
        assert_eq!(boss.name, base.name);
    }

    #[test]
    fn mob_id_all_contains_expected_mobs() {
        assert!(MobId::ALL.contains(&MobId::Goblin));
        assert!(MobId::ALL.contains(&MobId::Slime));
        assert!(MobId::ALL.contains(&MobId::Dragon));
    }

    #[test]
    fn slime_is_normal_quality() {
        let spec = MobId::Slime.spec();
        assert!(matches!(spec.quality, MobQuality::Normal));
    }

    #[test]
    fn dragon_is_boss_quality() {
        let spec = MobId::Dragon.spec();
        assert!(matches!(spec.quality, MobQuality::Boss));
    }

    #[test]
    fn black_dragon_is_boss_quality() {
        let spec = MobId::BlackDragon.spec();
        assert!(matches!(spec.quality, MobQuality::Boss));
    }

    #[test]
    fn merchant_has_zero_combat_stats() {
        let spec = MobId::Merchant.spec();
        assert_eq!(*spec.attack.start(), 0);
        assert_eq!(*spec.attack.end(), 0);
        assert_eq!(*spec.defense.start(), 0);
        assert_eq!(*spec.defense.end(), 0);
        assert_eq!(*spec.dropped_gold.start(), 0);
        assert_eq!(*spec.dropped_xp.start(), 0);
    }

    #[test]
    fn all_mobs_have_entity_size() {
        for mob_id in MobId::ALL {
            let spec = mob_id.spec();
            assert!(spec.entity_size.width > 0.0 && spec.entity_size.height > 0.0);
        }
    }
}
