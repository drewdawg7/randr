use std::collections::HashMap;

use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::stats::{StatInstance, StatSheet, StatType};

#[derive(Resource, Debug, Clone)]
pub struct PlayerName(pub &'static str);

impl Default for PlayerName {
    fn default() -> Self {
        Self("Drew")
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct PlayerGold(pub i32);

impl PlayerGold {
    pub fn add(&mut self, amount: i32) {
        self.0 += amount;
    }

    pub fn subtract(&mut self, amount: i32) {
        self.0 = (self.0 - amount).max(0);
    }
}

pub fn default_player_stats() -> StatSheet {
    let stats: HashMap<StatType, StatInstance> = HashMap::new();
    let mut sheet = StatSheet { stats };
    sheet.insert(StatType::Attack.instance(8));
    sheet.insert(StatType::Defense.instance(3));
    sheet.insert(StatType::GoldFind.instance(0));
    sheet.insert(StatType::Mining.instance(100));
    sheet.insert(StatType::Health.instance(100));
    sheet
}

pub fn effective_magicfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::MagicFind);
    let equipment = inventory.sum_equipment_stats(StatType::MagicFind);
    base + equipment
}

pub fn effective_mining(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::Mining);
    let equipment = inventory.sum_equipment_stats(StatType::Mining);
    base + equipment
}

pub fn effective_goldfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::GoldFind);
    let equipment = inventory.sum_equipment_stats(StatType::GoldFind);
    base + equipment
}

pub fn apply_level_up_bonuses(stats: &mut StatSheet, new_level: i32) {
    if new_level % 10 == 0 {
        stats.increase_stat(StatType::Defense, 1);
    }
    stats.increase_stat(StatType::Health, 5);
    stats.increase_stat_max(StatType::Health, 5);
    stats.increase_stat(StatType::Attack, 1);
}

pub fn add_xp_with_bonuses(
    prog: &mut crate::entities::Progression,
    stats: &mut StatSheet,
    xp: i32,
) -> i32 {
    let old_level = prog.level;
    let levels_gained = prog.add_xp(xp);
    for level in (old_level + 1)..=(old_level + levels_gained) {
        apply_level_up_bonuses(stats, level);
    }
    levels_gained
}
