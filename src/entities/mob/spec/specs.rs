use once_cell::sync::Lazy;

use crate::{
    entities::mob::enums::MobQuality,
    item::ItemId,
    loot::LootTable,
};

use super::definition::MobSpec;

pub static COW: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Cow",
    quality: MobQuality::Normal,
    max_health: 20..=25,
    attack: 1..=4,
    dropped_gold: 1..=3,
    dropped_xp: 5..=9,
    loot: LootTable::new()
        .with(ItemId::Cowhide, 3, 4, 1..=3)
        .with(ItemId::GoldRing, 1, 1000, 1..=1),
});
pub static SLIME: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Slime",
    quality: MobQuality::Normal,
    max_health: 15..=22,
    attack: 2..=4,
    dropped_gold: 1..=3,
    dropped_xp: 5..=9,
    loot: LootTable::new()
        .with(ItemId::SlimeGel, 3, 4, 1..=4)
        .with(ItemId::GoldRing, 1, 100, 1..=1),
});

pub static GOBLIN: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Goblin",
    quality: MobQuality::Normal,
    max_health: 33..=41,
    attack: 10..=15,
    dropped_gold: 10..=19,
    dropped_xp: 13..=20,
    loot: LootTable::new()
        .with(ItemId::Sword, 1, 15, 1..=1)
        .with(ItemId::BasicShield, 1, 15, 1..=1)
        .with(ItemId::GoldRing, 1, 100, 1..=1),
});

pub static DRAGON: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Dragon",
    quality: MobQuality::Boss,
    max_health: 500..=700,
    attack: 25..=30,
    dropped_gold: 250..=350,
    dropped_xp: 500..=750,
    loot: LootTable::new()
        .with(ItemId::GoldRing, 1, 100, 1..=1)
        .with(ItemId::QualityUpgradeStone, 1, 1, 1..=1),
});
