use std::collections::HashMap;
use std::time::Duration;

use once_cell::sync::Lazy;

use crate::mob::MobId;
use crate::item::ItemId;
use crate::location::enums::LocationId;
use crate::location::mine::RockId;

use super::definition::{
    AlchemistData, BlacksmithData, FieldData, LocationData, LocationSpec, MineData, StoreData,
};

pub static VILLAGE_STORE: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageStore,
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
});

pub static VILLAGE_BLACKSMITH: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageBlacksmith,
    name: "Village Blacksmith",
    description: "A forge where equipment can be upgraded and ore smelted",
    refresh_interval: None,
    min_level: None,
    data: LocationData::Blacksmith(BlacksmithData {
        max_upgrades: 10,
        base_upgrade_cost: 10,
    }),
});

pub static VILLAGE_ALCHEMIST: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageAlchemist,
    name: "Village Alchemist",
    description: "A mystical shop where potions are brewed from magical ingredients",
    refresh_interval: None,
    min_level: None,
    data: LocationData::Alchemist(AlchemistData {}),
});

pub static VILLAGE_FIELD: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageField,
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
});

pub static VILLAGE_MINE: Lazy<LocationSpec> = Lazy::new(|| LocationSpec {
    location_id: LocationId::VillageMine,
    name: "Village Mine",
    description: "A dark mine rich with ore deposits",
    refresh_interval: None,
    min_level: None,
    data: LocationData::Mine(MineData {
        rock_weights: HashMap::from([
            (RockId::Copper, 50),
            (RockId::Coal, 30),
            (RockId::Tin, 20),
        ]),
    }),
});
