use std::ops::RangeInclusive;

use rand::seq::SliceRandom;
use rand::Rng;

use super::entity::DungeonEntity;
use super::grid::{GridPosition, GridSize};
use super::layout::DungeonLayout;
use crate::mob::MobId;

#[derive(Debug, Clone)]
pub struct SpawnTable {
    mobs: Vec<(MobId, u32)>,
    mob_count: RangeInclusive<u32>,
    chests: RangeInclusive<u32>,
}

impl Default for SpawnTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnTable {
    pub fn new() -> Self {
        Self {
            mobs: Vec::new(),
            mob_count: 0..=0,
            chests: 0..=0,
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        self.mobs.push((mob_id, weight));
        self
    }

    pub fn mob_count(mut self, count: RangeInclusive<u32>) -> Self {
        self.mob_count = count;
        self
    }

    pub fn chest(mut self, count: RangeInclusive<u32>) -> Self {
        self.chests = count;
        self
    }

    pub fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) {
        let mut spawn_points = layout.spawn_points();
        spawn_points.shuffle(rng);
        let mut spawn_iter = spawn_points.into_iter();

        let chest_count = rng.gen_range(self.chests.clone());
        for _ in 0..chest_count {
            if let Some((x, y)) = spawn_iter.next() {
                let variant = rng.gen_range(0..4);
                layout.add_entity(
                    GridPosition::new(x, y),
                    DungeonEntity::Chest {
                        variant,
                        size: GridSize::single(),
                    },
                );
            }
        }

        let total_weight: u32 = self.mobs.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return;
        }

        let mob_count = rng.gen_range(self.mob_count.clone());
        for _ in 0..mob_count {
            if let Some((x, y)) = spawn_iter.next() {
                let mob_id = self.weighted_mob_select(rng, total_weight);
                let size = mob_id.spec().grid_size;
                layout.add_entity(GridPosition::new(x, y), DungeonEntity::Mob { mob_id, size });
            }
        }
    }

    fn weighted_mob_select(&self, rng: &mut impl Rng, total_weight: u32) -> MobId {
        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;

        for (mob_id, weight) in &self.mobs {
            cumulative += weight;
            if roll < cumulative {
                return *mob_id;
            }
        }

        self.mobs[0].0
    }
}
