use std::ops::RangeInclusive;

use crate::{
    loot::LootTable,
    registry::Registry,
    stats::{HasStats, StatSheet},
};

pub type MobSpecId = usize;

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub name: &'static str,
    pub stats: StatSheet,
    pub gold: i32,
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

    pub fn increase_health(&mut self, amount: i32) {
        self.inc_hp(amount);
    }

    pub fn decrease_health(&mut self, amount: i32) {
        self.dec_hp(amount);
    }
}

pub struct MobSpec {
    pub name: &'static str,
    pub max_health: RangeInclusive<i32>,
    pub attack: RangeInclusive<i32>,
    pub dropped_gold: RangeInclusive<i32>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MobKind {
    Slime,
    Goblin,
}

pub type MobRegistry = Registry<MobKind, MobSpec>;


