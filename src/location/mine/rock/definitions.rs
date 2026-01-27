//! Rock definitions using the entity_macros system
//!
//! This file consolidates:
//! - RockSpec struct definition
//! - RockId enum
//! - All rock spec constants
//! - The spec() method on RockId

use crate::item::ItemId;
use crate::loot::LootTable;
use crate::registry::{RegistryDefaults, SpawnFromSpec};
use crate::stats::{StatSheet, StatType};

use super::definition::Rock;

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
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
        }
        Coal {
            name: "Coal Rock",
            health: 50,
            loot: LootTable::new()
                .with(ItemId::Coal, 1, 1, 1..=2)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
        }
        Gold {
            name: "Gold Rock",
            health: 50,
            loot: LootTable::new()
                .with(ItemId::GoldOre, 1, 1, 1..=3)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
        }
        Mixed {
            name: "Mixed Rock",
            health: 100,
            loot: LootTable::new()
                .with(ItemId::GoldOre, 1, 2, 1..=4)
                .with(ItemId::IronOre, 1, 2, 1..=4)
                .with(ItemId::Coal, 1, 2, 1..=4)
                .with(ItemId::QualityUpgradeStone, 1, 100, 1..=1),
        }
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// Spawn Implementation
// ─────────────────────────────────────────────────────────────────────────────

impl SpawnFromSpec<RockId> for RockSpec {
    type Output = Rock;

    fn spawn_from_spec(kind: RockId, spec: &Self) -> Self::Output {
        Rock {
            rock_id: kind,  // Use the ID passed in, not stored in spec
            stats: StatSheet::new().with(StatType::Health, spec.health),
            loot: spec.loot.clone(),
        }
    }
}

impl RegistryDefaults<RockId> for RockSpec {
    fn defaults() -> impl IntoIterator<Item = (RockId, Self)> {
        RockId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience Methods
// ─────────────────────────────────────────────────────────────────────────────

impl RockId {
    /// Spawn a Rock instance from this RockId
    pub fn spawn(&self) -> Rock {
        RockSpec::spawn_from_spec(*self, self.spec())
    }
}
