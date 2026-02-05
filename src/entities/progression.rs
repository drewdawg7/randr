use bevy::prelude::*;

#[derive(Resource, Component, Debug, Default, Clone)]
pub struct Progression {
    pub level: i32,
    pub xp: i32,
    pub total_xp: i32,
}

impl Progression {
    pub fn new() -> Self {
        Self {
            level: 1,
            xp: 0,
            total_xp: 0,
        }
    }

    pub fn xp_to_next_level(level: i32) -> i32 {
        (50.0 * 1.1_f64.powi(level - 1)).round() as i32
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
