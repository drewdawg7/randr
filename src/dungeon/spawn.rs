use std::ops::RangeInclusive;

use rand::Rng;

use super::grid::GridSize;
use super::layout::DungeonLayout;
use super::spawn_rules::{
    ChestSpawner, ComposedSpawnRules, CraftingStationSpawner, GuaranteedMobSpawner, NpcSpawner,
    ProbabilityCraftingStationSpawner, ProbabilityNpcSpawner, RockSpawner, SpawnRule,
    SpawnRuleKind, StairsSpawner, WeightedMobSpawner,
};
use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;

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
    forge_count: RangeInclusive<u32>,
    anvil_count: RangeInclusive<u32>,
    forge_chance: Option<f64>,
    anvil_chance: Option<f64>,
    guaranteed_mobs: Vec<(MobId, u32)>,
    npc_spawns: Vec<(MobId, RangeInclusive<u32>)>,
    npc_chances: Vec<(MobId, f64)>,
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
            forge_count: 0..=0,
            anvil_count: 0..=0,
            forge_chance: None,
            anvil_chance: None,
            guaranteed_mobs: Vec::new(),
            npc_spawns: Vec::new(),
            npc_chances: Vec::new(),
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
        self.forge_count = count;
        self
    }

    /// Add anvil count range (always 1x1).
    pub fn anvil(mut self, count: RangeInclusive<u32>) -> Self {
        self.anvil_count = count;
        self
    }

    /// Spawn 1 forge with given probability (0.0 to 1.0).
    pub fn forge_chance(mut self, probability: f64) -> Self {
        self.forge_chance = Some(probability);
        self
    }

    /// Spawn 1 anvil with given probability (0.0 to 1.0).
    pub fn anvil_chance(mut self, probability: f64) -> Self {
        self.anvil_chance = Some(probability);
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

    /// Spawn 1 NPC with given probability (0.0 to 1.0).
    pub fn npc_chance(mut self, mob_id: MobId, probability: f64) -> Self {
        self.npc_chances.push((mob_id, probability));
        self
    }

    /// Build a ComposedSpawnRules from this table's configuration.
    fn build_rules(&self) -> ComposedSpawnRules {
        let mut rules = ComposedSpawnRules::new();

        // 1. Chests
        if *self.chest_count.end() > 0 {
            rules.push(SpawnRuleKind::Chest(ChestSpawner::new(self.chest_count.clone())));
        }

        // 2. Stairs
        if *self.stairs_count.end() > 0 {
            rules.push(SpawnRuleKind::Stairs(StairsSpawner::new(self.stairs_count.clone())));
        }

        // 3. Rocks
        if *self.rock_count.end() > 0 {
            rules.push(SpawnRuleKind::Rock(RockSpawner::new(self.rock_count.clone())));
        }

        // 4. Forges (count range)
        if *self.forge_count.end() > 0 {
            rules.push(SpawnRuleKind::CraftingStation(CraftingStationSpawner::new(
                CraftingStationType::Forge,
                self.forge_count.clone(),
            )));
        }

        // 5. Forges (probability)
        if let Some(probability) = self.forge_chance {
            rules.push(SpawnRuleKind::ProbabilityCraftingStation(
                ProbabilityCraftingStationSpawner::new(CraftingStationType::Forge, probability),
            ));
        }

        // 6. Anvils (count range)
        if *self.anvil_count.end() > 0 {
            rules.push(SpawnRuleKind::CraftingStation(CraftingStationSpawner::new(
                CraftingStationType::Anvil,
                self.anvil_count.clone(),
            )));
        }

        // 7. Anvils (probability)
        if let Some(probability) = self.anvil_chance {
            rules.push(SpawnRuleKind::ProbabilityCraftingStation(
                ProbabilityCraftingStationSpawner::new(CraftingStationType::Anvil, probability),
            ));
        }

        // 8. NPCs (count range)
        for (mob_id, count_range) in &self.npc_spawns {
            rules.push(SpawnRuleKind::Npc(NpcSpawner::new(*mob_id, count_range.clone())));
        }

        // 9. NPCs (probability)
        for (mob_id, probability) in &self.npc_chances {
            rules.push(SpawnRuleKind::ProbabilityNpc(ProbabilityNpcSpawner::new(
                *mob_id, *probability,
            )));
        }

        // 10. Guaranteed mobs
        for (mob_id, count) in &self.guaranteed_mobs {
            rules.push(SpawnRuleKind::GuaranteedMob(GuaranteedMobSpawner::new(*mob_id, *count)));
        }

        // 11. Weighted mobs
        if !self.entries.is_empty() && *self.mob_count.end() > 0 {
            let mut weighted = WeightedMobSpawner::new().count(self.mob_count.clone());
            for entry in &self.entries {
                let SpawnEntityType::Mob(mob_id) = entry.entity_type;
                weighted = weighted.mob(mob_id, entry.weight);
            }
            rules.push(SpawnRuleKind::WeightedMob(weighted));
        }

        rules
    }

    pub fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) {
        self.build_rules().apply(layout, rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::{DungeonEntity, DungeonLayout, Tile, TileType};

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
    fn mob_entry_stores_size_from_spec() {
        let table = SpawnTable::new().mob(MobId::Goblin, 1);

        assert_eq!(table.entries.len(), 1);
        let entry = &table.entries[0];
        assert_eq!(entry.entity_type, SpawnEntityType::Mob(MobId::Goblin));
        assert_eq!(entry.weight, 1);
        assert_eq!(entry.size, MobId::Goblin.spec().grid_size);
    }

    #[test]
    fn spawn_table_applies_chests() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

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
        let mut layout = create_test_layout(20, 20);

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

        if let (_, DungeonEntity::Mob { size, .. }) = mobs[0] {
            assert_eq!(*size, MobId::Goblin.spec().grid_size);
        } else {
            panic!("Expected Mob entity");
        }
    }

    #[test]
    fn entities_do_not_overlap() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(30, 30);

        SpawnTable::new()
            .mob(MobId::Goblin, 1)
            .mob(MobId::Slime, 1)
            .mob_count(3..=3)
            .chest(2..=2)
            .apply(&mut layout, &mut rng);

        let entities = layout.entities();

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
        let mut layout = create_test_layout(10, 10);

        SpawnTable::empty().apply(&mut layout, &mut rng);

        assert!(layout.entities().is_empty());
    }

    #[test]
    fn spawn_table_applies_rocks() {
        let mut rng = rand::thread_rng();
        let mut layout = create_test_layout(10, 10);

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
        let mut layout = create_test_layout(10, 10);

        SpawnTable::new().rock(3..=3).apply(&mut layout, &mut rng);

        for (_, entity) in layout.entities() {
            if matches!(entity, DungeonEntity::Rock { .. }) {
                assert_eq!(entity.size(), GridSize::single());
            }
        }
    }
}

