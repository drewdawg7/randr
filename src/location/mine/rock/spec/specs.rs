use once_cell::sync::Lazy;

use crate::{item::ItemId, loot::LootTable};

use super::super::RockId;
use super::definition::RockSpec;

pub static COPPER_ROCK: Lazy<RockSpec> = Lazy::new(|| RockSpec {
    rock_id: RockId::Copper,
    name: "Copper Rock",
    health: 50,
    loot: LootTable::new()
        .with(ItemId::CopperOre, 1, 1, 1..=3)
        .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
});

pub static COAL_ROCK: Lazy<RockSpec> = Lazy::new(|| RockSpec {
    rock_id: RockId::Coal,
    name: "Coal Rock",
    health: 50,
    loot: LootTable::new()
        .with(ItemId::Coal, 1, 1, 1..=2)
        .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
});

pub static TIN_ROCK: Lazy<RockSpec> = Lazy::new(|| RockSpec {
    rock_id: RockId::Tin,
    name: "Tin Rock",
    health: 50,
    loot: LootTable::new()
        .with(ItemId::TinOre, 1, 1, 1..=3)
        .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
});

pub static MIXED_ROCK: Lazy<RockSpec> = Lazy::new(|| RockSpec {
    rock_id: RockId::Mixed,
    name: "Mixed Rock",
    health: 100,
    loot: LootTable::new()
        .with(ItemId::TinOre, 1, 2, 1..=4)
        .with(ItemId::CopperOre, 1, 2, 1..=4)
        .with(ItemId::Coal, 1, 2, 1..=4)
        .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
});
