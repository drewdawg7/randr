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

pub trait Combatant: Named + IsKillable {
    fn effective_attack(&self) -> i32;
    fn increase_health(&mut self, amount: i32) {
        self.inc(StatType::Health, amount);
    }
    fn effective_defense(&self) -> i32 {
        self.def()
    }
    fn effective_health(&self) -> i32 {
        self.health()
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
