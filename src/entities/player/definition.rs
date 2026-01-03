use crate::HasInventory;
use crate::{entities::Progression, inventory::Inventory, stats::StatSheet};
use crate::magic::effect::PassiveEffect;
use crate::magic::tome::Tome;
use crate::stats::{HasStats, StatType};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub gold: i32,
    pub prog: Progression,
    pub inventory: Inventory,
    pub stats: StatSheet,
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
