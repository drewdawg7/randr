//! ECS components for mob entities.
//!
//! These components store combat-related data directly on mob entities,
//! replacing the trait-based Mob struct approach.

use bevy::prelude::*;

use crate::loot::LootTable;

use super::MobId;

/// Marker component identifying a mob entity and its type.
#[derive(Component, Debug, Clone, Copy)]
pub struct MobMarker(pub MobId);

/// Health component for entities that can take damage.
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

}

/// Combat stats for attack and defense calculations.
#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
}

/// Gold reward dropped when this entity dies.
#[derive(Component, Debug, Clone)]
pub struct GoldReward(pub i32);

/// XP reward given when this entity dies.
#[derive(Component, Debug, Clone)]
pub struct XpReward(pub i32);

/// Loot table for item drops on death.
#[derive(Component, Debug, Clone)]
pub struct MobLootTable(pub LootTable);

/// Guard against double death processing (matches Mob::death_processed).
#[derive(Component, Debug, Clone, Default)]
pub struct DeathProcessed(pub bool);
