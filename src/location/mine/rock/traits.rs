use crate::{
    combat::{IsKillable, RockDeathResult},
    stats::{HasStats, StatSheet},
};

use super::Rock;

impl HasStats for Rock {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl IsKillable for Rock {
    type DeathResult = RockDeathResult;

    fn on_death(&mut self) -> RockDeathResult {
        RockDeathResult {
            drops: self.roll_drops(),
        }
    }
}
