use crate::stats::{StatInstance, StatSheet, StatType};



pub trait HasStats {
    fn stats(&self) -> &StatSheet;
    fn stats_mut(&mut self) -> &mut StatSheet;

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

    // Stat getters
    fn goldfind(&self) -> i32 {self.value(StatType::GoldFind)}
    fn mining(&self) -> i32 {self.value(StatType::Mining)}
    fn hp(&self) -> i32 { self.value(StatType::Health) }
    fn max_hp(&self) -> i32 { self.max_value(StatType::Health) }
    fn attack(&self) -> i32 { self.value(StatType::Attack) }
    fn defense(&self) -> i32 { self.value(StatType::Defense) }


}

