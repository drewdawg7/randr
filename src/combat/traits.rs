use crate::stats::{HasStats};

pub trait Combatant:Named + HasStats{
    fn effective_attack(&self) -> i32;
    fn increase_health(&mut self, amount: i32);
    fn decrease_health(&mut self, amount: i32);
    fn effective_defense(&self) -> i32 {
        self.def()
    }
    fn effective_health(&self) -> i32 {
        self.hp()
    }
    fn is_alive(&self) -> bool {
        self.effective_health() > 0
    }
    fn take_damage(&mut self, damage: i32) {
        self.decrease_health(damage);
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
