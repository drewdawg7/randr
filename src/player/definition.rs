use std::collections::HashMap;
use std::fmt::Display;

use bevy::prelude::*;

use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::magic::effect::PassiveEffect;
use crate::magic::tome::Tome;
use crate::stats::{HasStats, StatInstance, StatSheet, StatType};

// =============================================================================
// Individual Resources (new granular design)
// =============================================================================

/// Player's display name
#[derive(Resource, Debug, Clone)]
pub struct PlayerName(pub &'static str);

impl Default for PlayerName {
    fn default() -> Self {
        Self("Drew")
    }
}

/// Player's gold currency
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

/// Returns the default player stats
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

/// Helper functions for calculating effective stats from inventory and base stats
pub fn effective_magicfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::MagicFind);
    let equipment = inventory.sum_equipment_stats(StatType::MagicFind);
    let tome_bonus = tome_magicfind_bonus(inventory);
    base + equipment + tome_bonus
}

pub fn effective_mining(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::Mining);
    let equipment = inventory.sum_equipment_stats(StatType::Mining);
    base + equipment
}

pub fn effective_goldfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::GoldFind);
    let equipment = inventory.sum_equipment_stats(StatType::GoldFind);
    let tome_bonus = tome_goldfind_bonus(inventory);
    base + equipment + tome_bonus
}

/// Get all passive effects from the equipped tome
pub fn tome_passive_effects(inventory: &Inventory) -> Vec<&PassiveEffect> {
    inventory
        .equipped_tome()
        .map(|tome| tome.passive_effects())
        .unwrap_or_default()
}

/// Sum tome bonus for a given stat type
fn sum_tome_bonus(inventory: &Inventory, stat: StatType) -> i32 {
    tome_passive_effects(inventory)
        .iter()
        .filter_map(|e| e.bonus_value(stat))
        .sum()
}

/// Calculate passive bonus to attack from tome
pub fn tome_attack_bonus(inventory: &Inventory) -> i32 {
    sum_tome_bonus(inventory, StatType::Attack)
}

/// Calculate passive bonus to defense from tome
pub fn tome_defense_bonus(inventory: &Inventory) -> i32 {
    sum_tome_bonus(inventory, StatType::Defense)
}

/// Calculate passive bonus to gold find from tome
pub fn tome_goldfind_bonus(inventory: &Inventory) -> i32 {
    sum_tome_bonus(inventory, StatType::GoldFind)
}

/// Calculate passive bonus to magic find from tome
pub fn tome_magicfind_bonus(inventory: &Inventory) -> i32 {
    sum_tome_bonus(inventory, StatType::MagicFind)
}

// =============================================================================
// Player struct (for trait-based combat compatibility)
// =============================================================================

/// Combined player data for trait-based operations (e.g., combat).
/// This is NOT a Resource - use the individual resources above for Bevy systems.
/// Construct this temporarily when you need trait-based polymorphism.
#[derive(Debug, Clone)]
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
            stats: default_player_stats(),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{})", self.name, self.hp(), self.max_hp())
    }
}

impl Player {
    /// Construct from individual resources (clones the data).
    /// Use this when you need a Player for trait-based operations.
    pub fn from_resources(
        name: &PlayerName,
        gold: &PlayerGold,
        prog: &Progression,
        inventory: &Inventory,
        stats: &StatSheet,
    ) -> Self {
        Self {
            name: name.0,
            gold: gold.0,
            prog: prog.clone(),
            inventory: inventory.clone(),
            stats: stats.clone(),
        }
    }

    /// Write changes back to individual resources.
    /// Call this after combat operations that modify the player.
    pub fn write_back(
        &self,
        gold: &mut PlayerGold,
        prog: &mut Progression,
        inventory: &mut Inventory,
        stats: &mut StatSheet,
    ) {
        gold.0 = self.gold;
        *prog = self.prog.clone();
        *inventory = self.inventory.clone();
        *stats = self.stats.clone();
    }

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
        tome_passive_effects(&self.inventory)
    }

    /// Calculate passive bonus to attack from tome
    pub fn tome_attack_bonus(&self) -> i32 {
        tome_attack_bonus(&self.inventory)
    }

    /// Calculate passive bonus to defense from tome
    pub fn tome_defense_bonus(&self) -> i32 {
        tome_defense_bonus(&self.inventory)
    }

    /// Calculate passive bonus to gold find from tome
    pub fn tome_goldfind_bonus(&self) -> i32 {
        tome_goldfind_bonus(&self.inventory)
    }

    /// Calculate passive bonus to magic find from tome
    pub fn tome_magicfind_bonus(&self) -> i32 {
        tome_magicfind_bonus(&self.inventory)
    }

    pub fn effective_magicfind(&self) -> i32 {
        effective_magicfind(&self.stats, &self.inventory)
    }

    pub fn effective_mining(&self) -> i32 {
        effective_mining(&self.stats, &self.inventory)
    }

    pub fn effective_goldfind(&self) -> i32 {
        effective_goldfind(&self.stats, &self.inventory)
    }
}

// =============================================================================
// PlayerGuard (RAII pattern for automatic write-back)
// =============================================================================

/// RAII guard that holds a `Player` and automatically writes changes back
/// to the underlying resources when dropped. Use this in combat and other
/// contexts where you always need to persist changes regardless of exit path.
pub struct PlayerGuard<'a> {
    player: Player,
    gold: &'a mut PlayerGold,
    prog: &'a mut Progression,
    inventory: &'a mut Inventory,
    stats: &'a mut StatSheet,
}

impl<'a> PlayerGuard<'a> {
    /// Create a guard that will auto-write changes on drop.
    pub fn from_resources(
        name: &PlayerName,
        gold: &'a mut PlayerGold,
        prog: &'a mut Progression,
        inventory: &'a mut Inventory,
        stats: &'a mut StatSheet,
    ) -> Self {
        let player = Player::from_resources(name, gold, prog, inventory, stats);
        Self {
            player,
            gold,
            prog,
            inventory,
            stats,
        }
    }
}

impl std::ops::Deref for PlayerGuard<'_> {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}

impl std::ops::DerefMut for PlayerGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.player
    }
}

impl Drop for PlayerGuard<'_> {
    fn drop(&mut self) {
        self.player
            .write_back(self.gold, self.prog, self.inventory, self.stats);
    }
}
