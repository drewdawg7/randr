//! Progression-related trait implementations for Mob

use crate::entities::progression::GivesXP;

use super::Mob;

impl GivesXP for Mob {
    fn give_xp(&self) -> i32 {
        self.dropped_xp
    }
}
