use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct StatSheet {
   pub stats: HashMap<StatType, StatInstance>      
}

impl StatSheet {
    pub fn stats(&self) -> &HashMap<StatType, StatInstance> {
       &self.stats
    }
    pub fn stats_mut(&mut self) -> &mut HashMap<StatType, StatInstance> {
        &mut self.stats
    }
    pub fn get_stat(&self, stat: StatType) -> Option<&StatInstance> {
        self.stats.get(&stat)
    }

    pub fn get_stat_value(&self, stat: StatType) -> i32 {
        let si = self.get_stat(stat);
        match si {
            Some(si) => si.current_value,
            None     => 0,
        }
    }

    pub fn get_max_stat_value(&self, stat: StatType) -> i32 {
        let si = self.get_stat(stat);
        match si {
            Some(si) => si.max_value,
            None     => 0
        }
    }
    pub fn increase_stat(&mut self, stat: StatType, amount: i32) {

        if let Some(si) = self.stats_mut().get_mut(&stat) { si.increase(amount);}
    }

    pub fn decrease_stat(&mut self, stat: StatType, amount: i32) {
        if let Some(si) = self.stats_mut().get_mut(&stat) {si.decrease(amount);}
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StatInstance {
    pub stat_type: StatType,
    pub current_value: i32,
    pub max_value: i32,
}

impl StatInstance {

    pub fn increase(&mut self, amount: i32) {
        self.current_value += amount;
    }

    pub fn decrease(&mut self, amount: i32) {
        self.current_value = (self.current_value - amount).max(0);
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StatType {
    Health,
    Attack,
    Defense
}
