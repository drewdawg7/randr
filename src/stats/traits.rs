use crate::stats::{StatInstance, StatSheet, StatType};

/// Trait for entities that can be healed.
/// Provides default implementations for healing by fixed amount or percentage.
pub trait Healable: HasStats {
    /// Heal the entity by a fixed amount, clamped to max HP.
    /// Returns the actual amount healed.
    fn heal(&mut self, amount: i32) -> i32 {
        let hp_before = self.hp();
        let max_hp = self.max_hp();
        let actual_heal = amount.min(max_hp - hp_before);
        self.inc(StatType::Health, actual_heal);
        actual_heal
    }

    /// Heal the entity by a percentage of max HP.
    /// Returns the actual amount healed.
    fn heal_percent(&mut self, percent: f32) -> i32 {
        let heal_amount = ((self.max_hp() as f32) * percent).round() as i32;
        self.heal(heal_amount)
    }

    /// Check if entity can be healed (not at full health).
    fn can_heal(&self) -> bool {
        self.hp() < self.max_hp()
    }
}

/// Blanket implementation of Healable for all entities with stats
impl<T: HasStats> Healable for T {}

#[allow(dead_code)]
pub trait HasStats {
    fn stats(&self) -> &StatSheet;
    fn stats_mut(&mut self) -> &mut StatSheet;

    #[allow(dead_code)]
    fn stat(&self, stat: StatType) -> Option<&StatInstance> {
        self.stats().stat(stat)
    }

    fn value(&self, stat: StatType) -> i32 {
        self.stats().value(stat)
    }

    fn max_value (&self, stat: StatType) -> i32 {
        self.stats().max_value(stat)
    }

    fn inc(&mut self, stat: StatType, amount: i32) {
        self.stats_mut().increase_stat(stat, amount);
    }

    fn dec(&mut self, stat: StatType, amount: i32) {
        self.stats_mut().decrease_stat(stat, amount);
    }

    fn inc_max(&mut self, stat: StatType, amount: i32) {
        self.stats_mut().increase_stat_max(stat, amount);
    }

    #[allow(dead_code)]
    fn dec_max(&mut self, stat: StatType, amount: i32) {
        self.stats_mut().decrease_stat_max(stat, amount);
    }

    // Stat getters
    fn magicfind(&self) -> i32 {self.value(StatType::MagicFind)}
    fn goldfind(&self) -> i32 {self.value(StatType::GoldFind)}
    fn mining(&self) -> i32 {self.value(StatType::Mining)}
    fn hp(&self) -> i32 { self.value(StatType::Health) }
    fn max_hp(&self) -> i32 { self.max_value(StatType::Health) }
    fn attack(&self) -> i32 { self.value(StatType::Attack) }
    fn defense(&self) -> i32 { self.value(StatType::Defense) }


}

