use crate::{
    loot::LootTable,
    registry::Registry,
    stats::StatSheet,
};

pub type MobSpecId = usize;

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub name: &'static str,
    pub stats: StatSheet,
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
    pub max_health: i32,
    pub attack: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MobKind {
    Slime,
    Goblin,
}

pub type MobRegistry = Registry<MobKind, MobSpec>;

use crate::stats::HasStats;

impl HasStats for Mob {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}
