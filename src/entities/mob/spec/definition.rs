use std::ops::RangeInclusive;

use crate::{loot::LootTable, registry::Registry};

use super::super::enums::{MobId, MobQuality};

#[derive(Clone)]
pub struct MobSpec {
    pub name: &'static str,
    pub max_health: RangeInclusive<i32>,
    pub attack: RangeInclusive<i32>,
    pub dropped_gold: RangeInclusive<i32>,
    pub dropped_xp: RangeInclusive<i32>,
    pub quality: MobQuality,
    pub loot: LootTable,
}

pub type MobRegistry = Registry<MobId, MobSpec>;
