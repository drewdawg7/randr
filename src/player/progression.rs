//! Progression-related trait implementations for Player

use crate::{
    entities::{progression::HasProgression, Progression},
    stats::{HasStats, StatType},
};

use super::Player;

impl HasProgression for Player {
    fn progression(&self) -> &Progression {
        &self.prog
    }
    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.prog
    }
    fn on_level_up(&mut self) {
        if self.level() % 10 == 0 {
            self.inc(StatType::Defense, 1);
        }
        self.inc(StatType::Health, 5);
        self.inc_max(StatType::Health, 5);
        self.inc(StatType::Attack, 1);
    }
}
