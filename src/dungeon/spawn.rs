use std::ops::RangeInclusive;

use rand::seq::SliceRandom;
use rand::Rng;

use super::entity::DungeonEntity;
use super::grid::GridSize;
use super::layout::DungeonLayout;
use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

/// Type of entity that can be spawned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEntityType {
    Mob(MobId),
}

/// A spawn table entry with entity type, weight, and size.
#[derive(Debug, Clone)]
pub struct SpawnEntry {
    pub entity_type: SpawnEntityType,
    pub weight: u32,
    pub size: GridSize,
}

#[derive(Debug, Clone)]
pub struct SpawnTable {
    entries: Vec<SpawnEntry>,
    mob_count: RangeInclusive<u32>,
    chest_count: RangeInclusive<u32>,
    stairs_count: RangeInclusive<u32>,
    rock_count: RangeInclusive<u32>,
    crafting_station_count: RangeInclusive<u32>,
    guaranteed_mobs: Vec<(MobId, u32)>,
    npc_spawns: Vec<(MobId, RangeInclusive<u32>)>,
}

impl Default for SpawnTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            mob_count: 0..=0,
            chest_count: 0..=0,
            stairs_count: 0..=0,
            rock_count: 0..=0,
            crafting_station_count: 0..=0,
            guaranteed_mobs: Vec::new(),
            npc_spawns: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    /// Add mob with its defined grid size from MobSpec.
    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        let size = mob_id.spec().grid_size;
        self.entries.push(SpawnEntry {
            entity_type: SpawnEntityType::Mob(mob_id),
            weight,
            size,
        });
        self
    }

    pub fn mob_count(mut self, count: RangeInclusive<u32>) -> Self {
        self.mob_count = count;
        self
    }

    /// Add chest count range (always 1x1).
    pub fn chest(mut self, count: RangeInclusive<u32>) -> Self {
        self.chest_count = count;
        self
    }

    /// Add stairs count range (always 1x1).
    pub fn stairs(mut self, count: RangeInclusive<u32>) -> Self {
        self.stairs_count = count;
        self
    }

    /// Add rock count range (always 1x1).
    pub fn rock(mut self, count: RangeInclusive<u32>) -> Self {
        self.rock_count = count;
        self
    }

    /// Add forge count range (always 1x1).
    pub fn forge(mut self, count: RangeInclusive<u32>) -> Self {
        self.crafting_station_count = count;
        self
    }

    /// Guarantee exactly `count` of this mob spawn on the floor (before weighted selection).
    pub fn guaranteed_mob(mut self, mob_id: MobId, count: u32) -> Self {
        self.guaranteed_mobs.push((mob_id, count));
        self
    }

    /// Add NPC spawns (blocks movement, no combat).
    pub fn npc(mut self, mob_id: MobId, count: RangeInclusive<u32>) -> Self {
        self.npc_spawns.push((mob_id, count));
        self
    }

    pub fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) {
        // 1. Spawn chests first (1x1, prioritize)
        let chest_count = rng.gen_range(self.chest_count.clone());
        for _ in 0..chest_count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                let variant = rng.gen_range(0..4);
                layout.add_entity(
                    pos,
                    DungeonEntity::Chest {
                        variant,
                        size: GridSize::single(),
                    },
                );
            }
        }

        // 2. Spawn stairs (1x1)
        let stairs_count = rng.gen_range(self.stairs_count.clone());
        for _ in 0..stairs_count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::Stairs {
                        size: GridSize::single(),
                    },
                );
            }
        }

        // 3. Spawn rocks (1x1)
        let rock_count = rng.gen_range(self.rock_count.clone());
        for _ in 0..rock_count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                let rock_type = match rng.gen_range(0..3u8) {
                    0 => RockType::Copper,
                    1 => RockType::Coal,
                    _ => RockType::Tin,
                };
                layout.add_entity(
                    pos,
                    DungeonEntity::Rock {
                        rock_type,
                        size: GridSize::single(),
                    },
                );
            }
        }

        // 4. Spawn crafting stations (1x1)
        let station_count = rng.gen_range(self.crafting_station_count.clone());
        for _ in 0..station_count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::CraftingStation {
                        station_type: CraftingStationType::Forge,
                        size: GridSize::single(),
                    },
                );
            }
        }

        // 5. Spawn NPCs (before guaranteed mobs)
        for (mob_id, count_range) in &self.npc_spawns {
            let count = rng.gen_range(count_range.clone());
            for _ in 0..count {
                let areas = layout.spawn_areas(GridSize::single());
                if let Some(&pos) = areas.choose(rng) {
                    layout.add_entity(
                        pos,
                        DungeonEntity::Npc {
                            mob_id: *mob_id,
                            size: GridSize::single(),
                        },
                    );
                }
            }
        }

        // 6. Spawn guaranteed mobs
        for (mob_id, count) in &self.guaranteed_mobs {
            let size = mob_id.spec().grid_size;
            for _ in 0..*count {
                let areas = layout.spawn_areas(size);
                if let Some(&pos) = areas.choose(rng) {
                    layout.add_entity(pos, DungeonEntity::Mob { mob_id: *mob_id, size });
                }
            }
        }

        // 7. Spawn mobs by weighted selection
        let total_weight: u32 = self.entries.iter().map(|e| e.weight).sum();
        if total_weight == 0 {
            return;
        }

        let mob_count = rng.gen_range(self.mob_count.clone());
        for _ in 0..mob_count {
            let entry = self.weighted_entry_select(rng, total_weight);
            let areas = layout.spawn_areas(entry.size);
            if let Some(&pos) = areas.choose(rng) {
                let SpawnEntityType::Mob(mob_id) = entry.entity_type;
                layout.add_entity(pos, DungeonEntity::Mob { mob_id, size: entry.size });
            }
        }
    }

    fn weighted_entry_select(&self, rng: &mut impl Rng, total_weight: u32) -> &SpawnEntry {
        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;

        for entry in &self.entries {
            cumulative += entry.weight;
            if roll < cumulative {
                return entry;
            }
        }

        &self.entries[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::LayoutBuilder;

    #[test]
    fn mob_entry_stores_size_from_spec() {
        let table = SpawnTable::new().mob(MobId::Goblin, 1);

        assert_eq!(table.entries.len(), 1);
        let entry = &table.entries[0];
        assert_eq!(entry.entity_type, SpawnEntityType::Mob(MobId::Goblin));
        assert_eq!(entry.weight, 1);
        // Size should match MobSpec
        assert_eq!(entry.size, MobId::Goblin.spec().grid_size);
    }

    #[test]
    fn spawn_table_applies_chests() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(10, 10).entrance(5, 8).build();

        SpawnTable::new().chest(2..=2).apply(&mut layout, &mut rng);

        let chests: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Chest { .. }))
            .collect();
        assert_eq!(chests.len(), 2);
    }

    #[test]
    fn spawn_table_applies_mobs_with_size() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(20, 20).entrance(10, 18).build();

        SpawnTable::new()
            .mob(MobId::Goblin, 1)
            .mob_count(1..=1)
            .apply(&mut layout, &mut rng);

        let mobs: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Mob { .. }))
            .collect();
        assert_eq!(mobs.len(), 1);

        // Verify the mob has the correct size from MobSpec
        if let (_, DungeonEntity::Mob { size, .. }) = mobs[0] {
            assert_eq!(*size, MobId::Goblin.spec().grid_size);
        } else {
            panic!("Expected Mob entity");
        }
    }

    #[test]
    fn entities_do_not_overlap() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(30, 30).entrance(15, 28).build();

        // Spawn multiple entities
        SpawnTable::new()
            .mob(MobId::Goblin, 1)
            .mob(MobId::Slime, 1)
            .mob_count(3..=3)
            .chest(2..=2)
            .apply(&mut layout, &mut rng);

        let entities = layout.entities();

        // Check no two entities occupy the same cells
        for (i, (pos1, entity1)) in entities.iter().enumerate() {
            for (pos2, entity2) in entities.iter().skip(i + 1) {
                let overlaps = pos1
                    .occupied_cells(entity1.size())
                    .any(|(x1, y1)| pos2.occupied_cells(entity2.size()).any(|(x2, y2)| x1 == x2 && y1 == y2));
                assert!(!overlaps, "Entities overlap at positions {:?} and {:?}", pos1, pos2);
            }
        }
    }

    #[test]
    fn empty_spawn_table_adds_nothing() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(10, 10).entrance(5, 8).build();

        SpawnTable::empty().apply(&mut layout, &mut rng);

        assert!(layout.entities().is_empty());
    }

    #[test]
    fn spawn_table_applies_rocks() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(10, 10).entrance(5, 8).build();

        SpawnTable::new().rock(2..=2).apply(&mut layout, &mut rng);

        let rocks: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Rock { .. }))
            .collect();
        assert_eq!(rocks.len(), 2);
    }

    #[test]
    fn rocks_are_1x1_size() {
        let mut rng = rand::thread_rng();
        let mut layout = LayoutBuilder::new(10, 10).entrance(5, 8).build();

        SpawnTable::new().rock(3..=3).apply(&mut layout, &mut rng);

        for (_, entity) in layout.entities() {
            if matches!(entity, DungeonEntity::Rock { .. }) {
                assert_eq!(entity.size(), GridSize::single());
            }
        }
    }
}
