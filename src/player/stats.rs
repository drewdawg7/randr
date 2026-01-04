//! Stats-related trait implementations for Player

use crate::stats::{HasStats, StatSheet};

use super::Player;

impl HasStats for Player {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}
