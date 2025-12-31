use std::ops::RangeInclusive;

use crate::{
    entities::mob::enums::MobQuality, loot::LootTable, registry::Registry, stats::{HasStats, StatSheet}
};

pub type MobSpecId = usize;

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub quality: MobQuality,
    pub name: &'static str,
    pub stats: StatSheet,
    pub gold: i32,
    pub dropped_xp: i32,
    pub loot_table: LootTable,
}

impl Mob {
    pub fn get_health(&self) -> i32 {
        self.hp()
    }

    pub fn get_attack(&self) -> i32 {
        self.attack()
    }

    pub fn get_max_health(&self) -> i32 {
        self.max_hp()
    }
}

pub struct MobSpec {
    pub name: &'static str,
    pub max_health: RangeInclusive<i32>,
    pub attack: RangeInclusive<i32>,
    pub dropped_gold: RangeInclusive<i32>,
    pub dropped_xp: RangeInclusive<i32>,
    pub quality: MobQuality,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MobKind {
    Slime,
    Goblin,
    Dragon,
}

pub type MobRegistry = Registry<MobKind, MobSpec>;


