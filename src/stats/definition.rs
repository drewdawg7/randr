use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatSheet {
   pub stats: HashMap<StatType, StatInstance>      
}

impl Default for StatSheet {
    fn default() -> Self {
        Self { stats: HashMap::new() }
    }
}

impl StatSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, stat_type: StatType, value: i32) -> Self {
        if value != 0 {
            self.insert(stat_type.instance(value));
        }
        self
    }

    pub fn stats(&self) -> &HashMap<StatType, StatInstance> {
       &self.stats
    }
    pub fn stats_mut(&mut self) -> &mut HashMap<StatType, StatInstance> {
        &mut self.stats
    }
    pub fn stat(&self, t:StatType) -> Option<&StatInstance> {
        self.stats.get(&t)
    }
    pub fn stat_mut(&mut self, t: StatType) -> Option<&mut StatInstance> {
        self.stats.get_mut(&t)
    }
    pub fn value(&self, t: StatType) -> i32 {
        match self.stat(t) {
            Some(si) => si.current_value,
            None     => 0
        }
    }

    pub fn max_value(&self, t: StatType) -> i32 {
        match self.stat(t) {
            Some(si) => si.max_value,
            None     => 0
        }
    }
    pub fn insert(&mut self, si: StatInstance){
        self.stats.insert(si.stat_type, si);
    }

    pub fn increase_stat(&mut self, stat: StatType, amount: i32) {
        if let Some(si) = self.stats_mut().get_mut(&stat) { si.increase(amount);}
    }

    pub fn increase_stat_max(&mut self, stat: StatType, amount: i32) {
        if let Some(si) = self.stats_mut().get_mut(&stat) { si.increase_max(amount);}
    }
    pub fn decrease_stat(&mut self, stat: StatType, amount: i32) {
        if let Some(si) = self.stats_mut().get_mut(&stat) {si.decrease(amount);}
    }

    pub fn decrease_stat_max(&mut self, stat: StatType, amount: i32) {
        if let Some(si) = self.stats_mut().get_mut(&stat) {si.decrease_max(amount);}
    }
}

/// A single stat with current and max values.
///
/// Note: `max_value` is only meaningful for `Health`. For other stats
/// (Attack, Defense, Mining, GoldFind), `max_value` equals `current_value`
/// and is not used for capping. This design keeps the struct uniform
/// across all stat types.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StatInstance {
    pub stat_type: StatType,
    pub current_value: i32,
    /// Only meaningful for Health (used to cap healing). For other stats,
    /// this mirrors current_value.
    pub max_value: i32,
}

impl StatInstance {

    pub fn increase(&mut self, amount: i32) {
        self.current_value += amount;
    }
    pub fn increase_max(&mut self, amount: i32) {
        self.max_value += amount;
    }
    pub fn decrease(&mut self, amount: i32) {
        self.current_value = (self.current_value - amount).max(0);
    }
    pub fn decrease_max(&mut self, amount: i32) {
        self.max_value = (self.max_value - amount).max(0);
    }


}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StatType {
    Health,
    Attack,
    Defense,
    GoldFind,
    Mining
}

impl StatType {
    pub fn all() -> &'static [StatType] {
        &[StatType::Health, StatType::Attack, StatType::Defense, StatType::GoldFind, StatType::Mining]
    }

    /// Creates a StatInstance with both current and max set to base_value.
    /// For Health, max_value is used for capping heals. For other stats,
    /// max_value is unused but set for uniformity.
    pub fn instance(self, base_value: i32) -> StatInstance {
        StatInstance { stat_type: self, current_value: base_value, max_value: base_value }
    }
}
