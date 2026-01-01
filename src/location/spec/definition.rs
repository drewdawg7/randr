use std::collections::HashMap;
use std::time::Duration;

use crate::entities::mob::MobId;
use crate::item::ItemId;
use crate::location::mine::RockId;
use crate::registry::Registry;

use super::super::activity::ActivitySpec;
use super::super::enums::LocationId;

/// Unified spec that all locations share
#[derive(Clone)]
pub struct LocationSpec {
    // === Identity ===
    pub location_id: LocationId,
    pub name: &'static str,
    pub description: &'static str,

    // === Timer/Refresh (optional) ===
    pub refresh_interval: Option<Duration>,

    // === Entry Requirements (optional) ===
    pub min_level: Option<i32>,

    // === Activities ===
    pub activities: Vec<ActivitySpec>,

    // === Location-Specific Data ===
    pub data: LocationData,
}

/// Location-specific data that cannot be generalized
#[derive(Clone)]
pub enum LocationData {
    Store(StoreData),
    Blacksmith(BlacksmithData),
    Field(FieldData),
    Mine(MineData),
}

#[derive(Clone)]
pub struct StoreData {
    /// Initial stock: (item_id, max_quantity)
    pub initial_stock: Vec<(ItemId, i32)>,
}

#[derive(Clone)]
pub struct BlacksmithData {
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
}

#[derive(Clone)]
pub struct FieldData {
    pub mob_weights: HashMap<MobId, i32>,
}

#[derive(Clone)]
pub struct MineData {
    pub rock_weights: HashMap<RockId, i32>,
}

pub type LocationRegistry = Registry<LocationId, LocationSpec>;
