use crate::{
    loot::LootTable,
    stats::{HasStats, StatSheet},
};

use super::definitions::RockId;

#[derive(Clone)]
pub struct Rock {
    pub rock_id: RockId,
    pub stats: StatSheet,
    pub loot: LootTable,
}

impl HasStats for Rock {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}
