use crate::combat::Attack;
use crate::stats::{HasStats, StatType};

/// Trait for entities that can be killed (health reduced to zero)
pub trait IsKillable: HasStats {
    /// The result type returned when this entity dies
    type DeathResult;

    /// Returns current health
    fn health(&self) -> i32 {
        self.hp()
    }

    /// Apply damage to this entity
    fn take_damage(&mut self, amount: i32) {
        self.dec(StatType::Health, amount);
    }

    /// Check if entity is alive
    fn is_alive(&self) -> bool {
        self.health() > 0
    }

    /// Called when health reaches zero. Returns implementation-specific result.
    fn on_death(&mut self) -> Self::DeathResult;
}

/// Trait for entities that can deal damage.
/// Provides attack range derived from stats with configurable variance.
pub trait DealsDamage: HasStats {
    /// Variance percentage for attack range (e.g., 0.25 = ±25%)
    const ATTACK_VARIANCE: f64 = 0.25;

    /// Returns the Attack struct with damage range.
    /// Default implementation derives range from Attack stat with variance.
    fn get_attack(&self) -> Attack {
        let base = self.attack();
        let variance = (base as f64 * Self::ATTACK_VARIANCE).round() as i32;
        Attack::new(
            (base - variance).max(1),
            base + variance,
        )
    }

    /// Returns the average attack value (for display purposes)
    fn effective_attack(&self) -> i32 {
        self.get_attack().average()
    }
}

pub trait Combatant: Named + IsKillable + DealsDamage {
    /// Returns effective defense value for damage reduction calculation
    fn effective_defense(&self) -> i32 {
        self.def()
    }

    /// Returns current health
    fn effective_health(&self) -> i32 {
        self.health()
    }

    fn increase_health(&mut self, amount: i32) {
        self.inc(StatType::Health, amount);
    }
}

pub trait Named: {
    fn name(&self) -> &str;
}

pub trait DropsGold: {
    fn drop_gold(&self) -> i32;
}

pub trait HasGold {
    fn gold(&self) -> i32;
    fn gold_mut(&mut self) -> &mut i32;

    fn add_gold(&mut self, amount: i32) {
        *self.gold_mut() += amount;
    }

    fn dec_gold(&mut self, amount: i32) {
        *self.gold_mut() = (self.gold() - amount).max(0);
    }
}
