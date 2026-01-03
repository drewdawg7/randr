use crate::item::ItemId;
use crate::registry::Registry;
use crate::stats::Healable;

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumableEffect {
    /// Restores HP by a fixed amount
    RestoreHealth(i32),
    /// Restores HP by percentage of max HP
    RestoreHealthPercent(f32),
}

impl ConsumableEffect {
    /// Describe the effect that was applied, given the actual value that resulted
    pub fn describe(&self, actual: i32) -> String {
        match self {
            ConsumableEffect::RestoreHealth(_) | ConsumableEffect::RestoreHealthPercent(_) => {
                format!("Restored {} HP", actual)
            }
        }
    }
}

pub type ConsumableRegistry = Registry<ItemId, ConsumableEffect>;

/// Trait for entities that can receive consumable effects
pub trait ApplyEffect {
    /// Apply a consumable effect to this entity
    /// Returns the actual effect amount (e.g., actual HP restored after clamping to max)
    fn apply_effect(&mut self, effect: &ConsumableEffect) -> i32;

    /// Check if the effect can be applied (e.g., not already at full health)
    fn can_apply_effect(&self, effect: &ConsumableEffect) -> bool;
}

/// Blanket implementation of ApplyEffect for all Healable entities
impl<T: Healable> ApplyEffect for T {
    fn apply_effect(&mut self, effect: &ConsumableEffect) -> i32 {
        match effect {
            ConsumableEffect::RestoreHealth(amount) => self.heal(*amount),
            ConsumableEffect::RestoreHealthPercent(percent) => self.heal_percent(*percent),
        }
    }

    fn can_apply_effect(&self, effect: &ConsumableEffect) -> bool {
        match effect {
            ConsumableEffect::RestoreHealth(_) | ConsumableEffect::RestoreHealthPercent(_) => {
                self.can_heal()
            }
        }
    }
}
