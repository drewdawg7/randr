
use crate::{combat::{Combatant, HasGold, Named}, entities::{progression::HasProgression, Progression}};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub health: i32,
    pub attack: i32,
    pub gold: i32,
    pub prog: Progression,
}

impl Player {
    pub fn on_level_up(&mut self) {
    }
}

impl Named for Player {
    fn name(&self) -> &str {
        self.name
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

impl HasProgression for Player {
    fn progression(&self) -> &Progression { &self.prog }
    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.prog
    }
    fn on_level_up(&mut self) {
        self.health += 5;
        self.attack += 1;
    }
}
