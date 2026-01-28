//! Location definitions using the entity_macros system
//!
//! This file consolidates:
//! - LocationSpec struct definition
//! - LocationId enum
//! - LocationData enum and variants
//! - All location spec constants
//! - The spec() method on LocationId

use std::collections::HashMap;
use std::time::Duration;

use crate::item::ItemId;
use crate::location::mine::RockId;
use crate::mob::MobId;
use crate::registry::RegistryDefaults;

// ─────────────────────────────────────────────────────────────────────────────
// Location-Specific Data Types
// ─────────────────────────────────────────────────────────────────────────────

/// Location-specific data that cannot be generalized
#[derive(Clone, Debug)]
pub enum LocationData {
    Store(StoreData),
    Blacksmith(BlacksmithData),
    Alchemist(AlchemistData),
    Field(FieldData),
    Mine(MineData),
    Dungeon(DungeonData),
}

#[derive(Clone, Debug)]
pub struct StoreData {
    /// Initial stock: (item_id, max_quantity)
    pub initial_stock: Vec<(ItemId, i32)>,
}

#[derive(Clone, Debug)]
pub struct BlacksmithData {
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
}

#[derive(Clone, Debug)]
pub struct AlchemistData {
    // Empty for now, but follows pattern for future expansion
}

#[derive(Clone, Debug)]
pub struct FieldData {
    pub mob_weights: HashMap<MobId, i32>,
}

#[derive(Clone, Debug)]
pub struct MineData {
    pub rock_weights: HashMap<RockId, i32>,
}

#[derive(Clone, Debug)]
pub struct DungeonData {
    // Dungeon-specific data is managed by DungeonRegistry/DungeonPlugin
}

// ─────────────────────────────────────────────────────────────────────────────
// LocationSpec and LocationId via entity_macros
// ─────────────────────────────────────────────────────────────────────────────

entity_macros::define_data! {
    spec LocationSpec {
        pub name: &'static str,
        pub description: &'static str,
        pub refresh_interval: Option<Duration>,
        pub min_level: Option<i32>,
        pub data: LocationData,
    }

    id LocationId;

    variants {
        // ─────────────────────────────────────────────────────────────────────
        // Commerce Locations
        // ─────────────────────────────────────────────────────────────────────
        VillageStore {
            name: "Village Store",
            description: "A humble shop selling basic supplies",
            refresh_interval: Some(Duration::from_secs(60)),
            min_level: None,
            data: LocationData::Store(StoreData {
                initial_stock: vec![
                    (ItemId::BasicHPPotion, 5),
                    (ItemId::Sword, 1),
                    (ItemId::BasicShield, 1),
                ],
            }),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Crafting Locations
        // ─────────────────────────────────────────────────────────────────────
        VillageBlacksmith {
            name: "Village Blacksmith",
            description: "A forge where equipment can be upgraded and ore smelted",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Blacksmith(BlacksmithData {
                max_upgrades: 10,
                base_upgrade_cost: 10,
            }),
        }
        VillageAlchemist {
            name: "Village Alchemist",
            description: "A mystical shop where potions are brewed from magical ingredients",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Alchemist(AlchemistData {}),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Combat Locations
        // ─────────────────────────────────────────────────────────────────────
        VillageField {
            name: "Village Field",
            description: "Rolling fields outside the village where monsters roam",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Field(FieldData {
                mob_weights: HashMap::from([
                    (MobId::Slime, 5),
                    (MobId::Cow, 5),
                    (MobId::Goblin, 3),
                ]),
            }),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Resource Locations
        // ─────────────────────────────────────────────────────────────────────
        VillageMine {
            name: "Village Mine",
            description: "A dark mine rich with ore deposits",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Mine(MineData {
                rock_weights: HashMap::from([
                    (RockId::Iron, 50),
                    (RockId::Coal, 30),
                    (RockId::Gold, 20),
                ]),
            }),
        }

        Home {
            name: "Home",
            description: "Your starting home with a door to the dungeon",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Dungeon(DungeonData {}),
        }
        MainDungeon {
            name: "Main Dungeon",
            description: "A dangerous 3-floor dungeon to conquer",
            refresh_interval: None,
            min_level: None,
            data: LocationData::Dungeon(DungeonData {}),
        }
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// RegistryDefaults
// ─────────────────────────────────────────────────────────────────────────────

impl RegistryDefaults<LocationId> for LocationSpec {
    fn defaults() -> impl IntoIterator<Item = (LocationId, Self)> {
        LocationId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional LocationId Methods
// ─────────────────────────────────────────────────────────────────────────────

/// High-level location categories with nested subtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationType {
    Commerce(CommerceSubtype),
    Crafting(CraftingSubtype),
    Combat(CombatSubtype),
    Resource(ResourceSubtype),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommerceSubtype {
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CraftingSubtype {
    Blacksmith,
    Alchemist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatSubtype {
    Field,
    Dungeon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceSubtype {
    Mine,
}

impl LocationId {
    /// Get the category type for this location
    pub fn location_type(&self) -> LocationType {
        match self {
            LocationId::VillageStore => LocationType::Commerce(CommerceSubtype::Store),
            LocationId::VillageBlacksmith => LocationType::Crafting(CraftingSubtype::Blacksmith),
            LocationId::VillageAlchemist => LocationType::Crafting(CraftingSubtype::Alchemist),
            LocationId::VillageField => LocationType::Combat(CombatSubtype::Field),
            LocationId::VillageMine => LocationType::Resource(ResourceSubtype::Mine),
            LocationId::Home | LocationId::MainDungeon => {
                LocationType::Combat(CombatSubtype::Dungeon)
            }
        }
    }
}
