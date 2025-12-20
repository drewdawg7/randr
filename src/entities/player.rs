
use crate::combat::{Combatant, HasGold, Named};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub gold: i32,
}

impl Named for Player {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl HasGold for Player {
    fn gold(&self) -> i32 {
        self.gold
    }
    fn gold_mut(&mut self) -> &mut i32 {
        &mut self.gold
    }
}
impl Combatant for Player {
    fn attack_power(&self) -> i32 {
        self.attack
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn health_mut(&mut self) -> &mut i32 {
        &mut self.health
    }

} 

