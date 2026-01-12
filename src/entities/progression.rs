use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone)]
pub struct Progression {
    pub level: i32,
    pub xp: i32,
    pub total_xp: i32
}


impl Progression {
    pub fn new() -> Self {
        Self { level: 1, xp: 0, total_xp: 0}
    }

    pub fn xp_to_next_level(level: i32) -> i32 {
        50 * level
    }

    pub fn add_xp(&mut self, xp: i32) -> i32 {
        self.total_xp += xp;
        self.xp += xp;
        let mut gained = 0;
        while self.xp >= Self::xp_to_next_level(self.level) {
            self.xp -= Self::xp_to_next_level(self.level);
            self.level += 1;
            gained += 1;
        }
        gained
    }
}

pub trait HasProgression {
    fn progression(&self) -> &Progression;
    fn progression_mut(&mut self) -> &mut Progression;
    fn level(&self) -> i32 { self.progression().level }
    fn on_level_up(&mut self);
    fn gain_xp(&mut self, amount: i32)  -> i32 {
        let gained = self.progression_mut().add_xp(amount);
        for _ in 0..gained {
            self.on_level_up();
        }
        gained
    }
    
}

pub trait GivesXP {
    fn give_xp(&self) -> i32;
}
