//! Stats-related trait implementations for Mob

use crate::stats::{HasStats, StatSheet};

use super::Mob;

impl HasStats for Mob {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}
