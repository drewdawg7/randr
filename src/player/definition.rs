use std::collections::HashMap;

use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::stats::{StatInstance, StatSheet, StatType};

#[derive(Resource, Debug, Clone)]
pub struct PlayerName(pub String);

impl Default for PlayerName {
    fn default() -> Self {
        Self("Drew".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_name_default() {
        let name = PlayerName::default();
        assert_eq!(name.0, "Drew");
    }

    #[test]
    fn player_gold_default() {
        let gold = PlayerGold::default();
        assert_eq!(gold.0, 0);
    }

    #[test]
    fn player_gold_add() {
        let mut gold = PlayerGold(100);
        gold.add(50);
        assert_eq!(gold.0, 150);
    }

    #[test]
    fn player_gold_add_negative() {
        let mut gold = PlayerGold(100);
        gold.add(-30);
        assert_eq!(gold.0, 70);
    }

    #[test]
    fn player_gold_subtract() {
        let mut gold = PlayerGold(100);
        gold.subtract(30);
        assert_eq!(gold.0, 70);
    }

    #[test]
    fn player_gold_subtract_does_not_go_negative() {
        let mut gold = PlayerGold(50);
        gold.subtract(100);
        assert_eq!(gold.0, 0);
    }

    #[test]
    fn player_gold_subtract_exactly_to_zero() {
        let mut gold = PlayerGold(50);
        gold.subtract(50);
        assert_eq!(gold.0, 0);
    }

    #[test]
    fn default_player_stats_has_expected_values() {
        let stats = default_player_stats();
        assert_eq!(stats.value(StatType::Attack), 8);
        assert_eq!(stats.value(StatType::Defense), 3);
        assert_eq!(stats.value(StatType::GoldFind), 0);
        assert_eq!(stats.value(StatType::Mining), 100);
        assert_eq!(stats.value(StatType::Health), 100);
    }

    #[test]
    fn effective_magicfind_with_no_equipment() {
        let stats = default_player_stats();
        let inventory = Inventory::new();
        let result = effective_magicfind(&stats, &inventory);
        assert_eq!(result, 0);
    }

    #[test]
    fn effective_mining_with_no_equipment() {
        let stats = default_player_stats();
        let inventory = Inventory::new();
        let result = effective_mining(&stats, &inventory);
        assert_eq!(result, 100);
    }

    #[test]
    fn effective_goldfind_with_no_equipment() {
        let stats = default_player_stats();
        let inventory = Inventory::new();
        let result = effective_goldfind(&stats, &inventory);
        assert_eq!(result, 0);
    }
}

