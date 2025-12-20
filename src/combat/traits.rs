pub trait Combatant:Named {
    fn attack_power(&self) -> i32;
    fn health(&self) -> i32;
    fn health_mut(&mut self) -> &mut i32;
    fn is_alive(&self) -> bool {
        self.health() > 0
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
}
