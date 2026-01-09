use std::{collections::HashMap, fmt::Display};

use bevy::prelude::*;

use crate::inventory::{HasInventory, Inventory};
use crate::entities::Progression;
use crate::magic::effect::PassiveEffect;
use crate::magic::tome::Tome;
use crate::stats::{HasStats, StatInstance, StatSheet, StatType};

#[derive(Resource, Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub gold: i32,
    pub prog: Progression,
    pub inventory: Inventory,
    pub stats: StatSheet,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            gold: 0,
            name: "Drew",
            prog: Progression::new(),
            inventory: Inventory::new(),
            stats: {
                let stats: HashMap<StatType, StatInstance> = HashMap::new();
                let mut sheet = StatSheet { stats };
                sheet.insert(StatType::Attack.instance(8));
                sheet.insert(StatType::Defense.instance(3));
                sheet.insert(StatType::GoldFind.instance(0));
                sheet.insert(StatType::Mining.instance(100));
                sheet.insert(StatType::Health.instance(100));
                sheet
            },
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{})", self.name, self.hp(), self.max_hp())
    }
}

impl Player {
    /// Get the equipped tome, if any
    pub fn equipped_tome(&self) -> Option<&Tome> {
        self.inventory.equipped_tome()
    }

    /// Get mutable access to the equipped tome
    pub fn equipped_tome_mut(&mut self) -> Option<&mut Tome> {
        self.inventory.equipped_tome_mut()
    }

    /// Get all passive effects from the equipped tome
    pub fn tome_passive_effects(&self) -> Vec<&PassiveEffect> {
        self.equipped_tome()
            .map(|tome| tome.passive_effects())
            .unwrap_or_default()
    }

    /// Get all passive effects with their spell names from the equipped tome
    pub fn tome_passive_effects_with_names(&self) -> Vec<(&str, &PassiveEffect)> {
        self.equipped_tome()
            .map(|tome| tome.passive_effects_with_names())
            .unwrap_or_default()
    }

    /// Calculate passive bonus to attack from tome
    pub fn tome_attack_bonus(&self) -> i32 {
        self.tome_passive_effects()
            .iter()
            .filter_map(|effect| match effect {
                PassiveEffect::BonusAttack(amount) => Some(*amount),
                _ => None,
            })
            .sum()
    }

    /// Calculate passive bonus to defense from tome
    pub fn tome_defense_bonus(&self) -> i32 {
        self.tome_passive_effects()
            .iter()
            .filter_map(|effect| match effect {
                PassiveEffect::BonusDefense(amount) => Some(*amount),
                _ => None,
            })
            .sum()
    }

    /// Calculate passive bonus to gold find from tome
    pub fn tome_goldfind_bonus(&self) -> i32 {
        self.tome_passive_effects()
            .iter()
            .filter_map(|effect| match effect {
                PassiveEffect::BonusGoldFind(amount) => Some(*amount),
                _ => None,
            })
            .sum()
    }

    /// Calculate passive bonus to magic find from tome
    pub fn tome_magicfind_bonus(&self) -> i32 {
        self.tome_passive_effects()
            .iter()
            .filter_map(|effect| match effect {
                PassiveEffect::BonusMagicFind(amount) => Some(*amount),
                _ => None,
            })
            .sum()
    }

    pub fn effective_magicfind(&self) -> i32 {
        let magicfind = self.inventory().sum_equipment_stats(StatType::MagicFind);
        self.magicfind() + magicfind + self.tome_magicfind_bonus()
    }

    pub fn effective_mining(&self) -> i32 {
        let mining = self.inventory().sum_equipment_stats(StatType::Mining);
        self.mining() + mining
    }

    pub fn effective_goldfind(&self) -> i32 {
        let gf = self.inventory().sum_equipment_stats(StatType::GoldFind);
        self.goldfind() + gf + self.tome_goldfind_bonus()
    }
}
