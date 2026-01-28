//! Composable spawn rules for entity placement in dungeon layouts.
//!
//! The `SpawnRule` trait enables modular entity spawning. Each rule
//! encapsulates specific placement logic, and rules can be composed
//! via `ComposedSpawnRules` to build complex spawn configurations.

use std::ops::RangeInclusive;

use rand::seq::SliceRandom;
use rand::Rng;

use super::entity::DungeonEntity;
use super::grid::{GridPosition, GridSize};
use super::layout::DungeonLayout;
use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

/// A single spawn rule that can place entities in a layout.
///
/// Rules are composable and applied in sequence. Each rule returns
/// the number of entities it spawned.
pub trait SpawnRule {
    /// Apply this rule to the layout, returning the count of entities spawned.
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32;
}

/// Enum-based spawn rule for composability without dyn.
#[derive(Clone)]
pub enum SpawnRuleKind {
    Chest(ChestSpawner),
    Stairs(StairsSpawner),
    Rock(RockSpawner),
    CraftingStation(CraftingStationSpawner),
    ProbabilityCraftingStation(ProbabilityCraftingStationSpawner),
    Npc(NpcSpawner),
    ProbabilityNpc(ProbabilityNpcSpawner),
    GuaranteedMob(GuaranteedMobSpawner),
    WeightedMob(WeightedMobSpawner),
    FixedPosition(FixedPositionSpawner),
}

impl SpawnRule for SpawnRuleKind {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        match self {
            Self::Chest(s) => s.apply(layout, rng),
            Self::Stairs(s) => s.apply(layout, rng),
            Self::Rock(s) => s.apply(layout, rng),
            Self::CraftingStation(s) => s.apply(layout, rng),
            Self::ProbabilityCraftingStation(s) => s.apply(layout, rng),
            Self::Npc(s) => s.apply(layout, rng),
            Self::ProbabilityNpc(s) => s.apply(layout, rng),
            Self::GuaranteedMob(s) => s.apply(layout, rng),
            Self::WeightedMob(s) => s.apply(layout, rng),
            Self::FixedPosition(s) => s.apply(layout, rng),
        }
    }
}

/// Apply rules in sequence, summing spawn counts.
#[derive(Clone, Default)]
pub struct ComposedSpawnRules(Vec<SpawnRuleKind>);

impl ComposedSpawnRules {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(mut self, rule: SpawnRuleKind) -> Self {
        self.0.push(rule);
        self
    }

    pub fn push(&mut self, rule: SpawnRuleKind) {
        self.0.push(rule);
    }
}

impl SpawnRule for ComposedSpawnRules {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        self.0.iter().map(|r| r.apply(layout, rng)).sum()
    }
}

/// Spawns a fixed count of chests with random variants.
#[derive(Clone)]
pub struct ChestSpawner {
    count: RangeInclusive<u32>,
}

impl ChestSpawner {
    pub fn new(count: RangeInclusive<u32>) -> Self {
        Self { count }
    }
}

impl SpawnRule for ChestSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
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
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns stairs that advance the player to the next floor.
#[derive(Clone)]
pub struct StairsSpawner {
    count: RangeInclusive<u32>,
}

impl StairsSpawner {
    pub fn new(count: RangeInclusive<u32>) -> Self {
        Self { count }
    }
}

impl SpawnRule for StairsSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::Stairs {
                        size: GridSize::single(),
                    },
                );
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns rocks with random types (Copper, Coal, Tin).
#[derive(Clone)]
pub struct RockSpawner {
    count: RangeInclusive<u32>,
}

impl RockSpawner {
    pub fn new(count: RangeInclusive<u32>) -> Self {
        Self { count }
    }
}

impl SpawnRule for RockSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                let rock_type = match rng.gen_range(0..4u8) {
                    0 => RockType::Coal,
                    1 => RockType::Copper,
                    2 => RockType::Iron,
                    _ => RockType::Gold,
                };
                let sprite_variant = rng.gen_range(0..2u8);
                layout.add_entity(
                    pos,
                    DungeonEntity::Rock {
                        rock_type,
                        sprite_variant,
                        size: GridSize::single(),
                    },
                );
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns crafting stations of a specific type.
#[derive(Clone)]
pub struct CraftingStationSpawner {
    station_type: CraftingStationType,
    count: RangeInclusive<u32>,
}

impl CraftingStationSpawner {
    pub fn new(station_type: CraftingStationType, count: RangeInclusive<u32>) -> Self {
        Self {
            station_type,
            count,
        }
    }
}

impl SpawnRule for CraftingStationSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::CraftingStation {
                        station_type: self.station_type,
                        size: GridSize::single(),
                    },
                );
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns NPCs (non-combat entities that block movement).
#[derive(Clone)]
pub struct NpcSpawner {
    mob_id: MobId,
    count: RangeInclusive<u32>,
}

impl NpcSpawner {
    pub fn new(mob_id: MobId, count: RangeInclusive<u32>) -> Self {
        Self { mob_id, count }
    }
}

impl SpawnRule for NpcSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::Npc {
                        mob_id: self.mob_id,
                        size: GridSize::single(),
                    },
                );
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns a guaranteed count of a specific mob type.
#[derive(Clone)]
pub struct GuaranteedMobSpawner {
    mob_id: MobId,
    count: u32,
}

impl GuaranteedMobSpawner {
    pub fn new(mob_id: MobId, count: u32) -> Self {
        Self { mob_id, count }
    }
}

impl SpawnRule for GuaranteedMobSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let size = self.mob_id.spec().grid_size;
        let mut spawned = 0;

        for _ in 0..self.count {
            let areas = layout.spawn_areas(size);
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(pos, DungeonEntity::Mob { mob_id: self.mob_id, size });
                spawned += 1;
            }
        }

        spawned
    }
}

/// Entry for weighted mob selection.
#[derive(Debug, Clone)]
pub struct WeightedMobEntry {
    pub mob_id: MobId,
    pub weight: u32,
    pub size: GridSize,
}

/// Spawns mobs using weighted random selection.
#[derive(Clone)]
pub struct WeightedMobSpawner {
    entries: Vec<WeightedMobEntry>,
    count: RangeInclusive<u32>,
}

impl Default for WeightedMobSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl WeightedMobSpawner {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            count: 0..=0,
        }
    }

    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        let size = mob_id.spec().grid_size;
        self.entries.push(WeightedMobEntry { mob_id, weight, size });
        self
    }

    pub fn count(mut self, count: RangeInclusive<u32>) -> Self {
        self.count = count;
        self
    }

    fn weighted_select(&self, rng: &mut impl Rng, total_weight: u32) -> &WeightedMobEntry {
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

impl SpawnRule for WeightedMobSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let total_weight: u32 = self.entries.iter().map(|e| e.weight).sum();
        if total_weight == 0 {
            return 0;
        }

        let count = rng.gen_range(self.count.clone());
        let mut spawned = 0;

        for _ in 0..count {
            let entry = self.weighted_select(rng, total_weight);
            let areas = layout.spawn_areas(entry.size);
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::Mob {
                        mob_id: entry.mob_id,
                        size: entry.size,
                    },
                );
                spawned += 1;
            }
        }

        spawned
    }
}

/// Spawns a crafting station with a given probability (0.0 to 1.0).
#[derive(Clone)]
pub struct ProbabilityCraftingStationSpawner {
    station_type: CraftingStationType,
    probability: f64,
}

impl ProbabilityCraftingStationSpawner {
    pub fn new(station_type: CraftingStationType, probability: f64) -> Self {
        Self {
            station_type,
            probability,
        }
    }
}

impl SpawnRule for ProbabilityCraftingStationSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        if rng.gen_bool(self.probability) {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::CraftingStation {
                        station_type: self.station_type,
                        size: GridSize::single(),
                    },
                );
                return 1;
            }
        }
        0
    }
}

/// Spawns an NPC with a given probability (0.0 to 1.0).
#[derive(Clone)]
pub struct ProbabilityNpcSpawner {
    mob_id: MobId,
    probability: f64,
}

impl ProbabilityNpcSpawner {
    pub fn new(mob_id: MobId, probability: f64) -> Self {
        Self { mob_id, probability }
    }
}

impl SpawnRule for ProbabilityNpcSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        if rng.gen_bool(self.probability) {
            let areas = layout.spawn_areas(GridSize::single());
            if let Some(&pos) = areas.choose(rng) {
                layout.add_entity(
                    pos,
                    DungeonEntity::Npc {
                        mob_id: self.mob_id,
                        size: GridSize::single(),
                    },
                );
                return 1;
            }
        }
        0
    }
}

/// Spawns an entity at a specific position.
#[derive(Clone)]
pub struct FixedPositionSpawner {
    pos: GridPosition,
    entity: DungeonEntity,
}

impl FixedPositionSpawner {
    pub fn new(pos: GridPosition, entity: DungeonEntity) -> Self {
        Self { pos, entity }
    }
}

impl SpawnRule for FixedPositionSpawner {
    fn apply(&self, layout: &mut DungeonLayout, _rng: &mut impl Rng) -> u32 {
        layout.add_entity(self.pos, self.entity.clone());
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{DungeonLayout, Tile, TileType};

    fn create_test_layout(width: usize, height: usize) -> DungeonLayout {
        let mut layout = DungeonLayout::new(width, height);
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    layout.set_tile(x, y, Tile::new(TileType::Wall));
                } else {
                    layout.set_tile(x, y, Tile::new(TileType::Floor));
                }
            }
        }
        layout
    }

    #[test]
    fn chest_spawner_places_chests() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

        let spawned = ChestSpawner::new(2..=2).apply(&mut layout, &mut rng);

        assert_eq!(spawned, 2);
        let chests: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Chest { .. }))
            .collect();
        assert_eq!(chests.len(), 2);
    }

    #[test]
    fn stairs_spawner_places_stairs() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

        let spawned = StairsSpawner::new(1..=1).apply(&mut layout, &mut rng);

        assert_eq!(spawned, 1);
        let stairs: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Stairs { .. }))
            .collect();
        assert_eq!(stairs.len(), 1);
    }

    #[test]
    fn rock_spawner_places_rocks() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

        let spawned = RockSpawner::new(3..=3).apply(&mut layout, &mut rng);

        assert_eq!(spawned, 3);
        let rocks: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Rock { .. }))
            .collect();
        assert_eq!(rocks.len(), 3);
    }

    #[test]
    fn crafting_station_spawner_places_forges() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

        let spawned =
            CraftingStationSpawner::new(CraftingStationType::Forge, 2..=2).apply(&mut layout, &mut rng);

        assert_eq!(spawned, 2);
        let stations: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| {
                matches!(
                    e,
                    DungeonEntity::CraftingStation {
                        station_type: CraftingStationType::Forge,
                        ..
                    }
                )
            })
            .collect();
        assert_eq!(stations.len(), 2);
    }

    #[test]
    fn guaranteed_mob_spawner_places_mobs() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(20, 20);

        let spawned = GuaranteedMobSpawner::new(MobId::Goblin, 2).apply(&mut layout, &mut rng);

        assert_eq!(spawned, 2);
        let mobs: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Mob { mob_id: MobId::Goblin, .. }))
            .collect();
        assert_eq!(mobs.len(), 2);
    }

    #[test]
    fn weighted_mob_spawner_places_mobs() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(20, 20);

        let spawned = WeightedMobSpawner::new()
            .mob(MobId::Goblin, 1)
            .mob(MobId::Slime, 1)
            .count(3..=3)
            .apply(&mut layout, &mut rng);

        assert_eq!(spawned, 3);
        let mobs: Vec<_> = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Mob { .. }))
            .collect();
        assert_eq!(mobs.len(), 3);
    }

    #[test]
    fn composed_rules_applies_all_rules() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(20, 20);

        let rules = ComposedSpawnRules::new()
            .add(SpawnRuleKind::Chest(ChestSpawner::new(1..=1)))
            .add(SpawnRuleKind::Stairs(StairsSpawner::new(1..=1)))
            .add(SpawnRuleKind::Rock(RockSpawner::new(1..=1)));

        let total = rules.apply(&mut layout, &mut rng);

        assert_eq!(total, 3);
        assert_eq!(layout.entities().len(), 3);
    }

    #[test]
    fn empty_composed_rules_spawns_nothing() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

        let rules = ComposedSpawnRules::new();
        let total = rules.apply(&mut layout, &mut rng);

        assert_eq!(total, 0);
        assert!(layout.entities().is_empty());
    }

    #[test]
    fn fixed_position_spawner_places_at_position() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);
        let pos = GridPosition::new(3, 3);

        let spawned = FixedPositionSpawner::new(
            pos,
            DungeonEntity::Chest {
                variant: 0,
                size: GridSize::single(),
            },
        )
        .apply(&mut layout, &mut rng);

        assert_eq!(spawned, 1);
        let entities = layout.entities();
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].0, pos);
    }
}

