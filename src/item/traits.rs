use std::fmt::Display;

use crate::{economy::WorthGold, stats::HasStats, stats::StatSheet};

use super::Item;

impl HasStats for Item {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.attack())
    }
}

impl WorthGold for Item {
    fn gold_value(&self) -> i32 {
        let base = self.gold_value;
        let quality_multiplier = self.quality.value_multiplier();
        ((base as f64) * quality_multiplier).round() as i32
    }
}
