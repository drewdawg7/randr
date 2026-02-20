use std::ops::RangeInclusive;

use bevy::prelude::*;
use bon::Builder;

use super::grid::EntitySize;
use crate::mob::MobId;

#[derive(Debug, Clone)]
pub struct MobSpawnEntry {
    pub mob_id: MobId,
    pub weight: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEntityType {
    Mob(MobId),
}

#[derive(Debug, Clone)]
pub struct SpawnEntry {
    pub entity_type: SpawnEntityType,
    pub weight: u32,
    pub size: EntitySize,
}

#[derive(Debug, Clone, Resource, Builder)]
pub struct SpawnTable {
    #[builder(field)]
    entries: Vec<SpawnEntry>,
    #[builder(field)]
    guaranteed_mobs: Vec<(MobId, u32)>,
    #[builder(field)]
    npc_spawns: Vec<(MobId, RangeInclusive<u32>)>,
    #[builder(field)]
    npc_chances: Vec<(MobId, f64)>,

    #[builder(default = 0..=0)]
    mob_count: RangeInclusive<u32>,
    #[builder(default = 0..=0)]
    chest: RangeInclusive<u32>,
    #[builder(default = 0..=0)]
    stairs: RangeInclusive<u32>,
    #[builder(default = 0..=0)]
    rock: RangeInclusive<u32>,
    #[builder(default = 0..=0)]
    forge: RangeInclusive<u32>,
    #[builder(default = 0..=0)]
    anvil: RangeInclusive<u32>,
    forge_chance: Option<f64>,
    anvil_chance: Option<f64>,
}

use spawn_table_builder::State;

impl<S: State> SpawnTableBuilder<S> {
    pub fn mob(mut self, mob_id: MobId, weight: u32) -> Self {
        let size = mob_id.spec().entity_size;
        self.entries.push(SpawnEntry {
            entity_type: SpawnEntityType::Mob(mob_id),
            weight,
            size,
        });
        self
    }

    pub fn guaranteed_mob(mut self, mob_id: MobId, count: u32) -> Self {
        self.guaranteed_mobs.push((mob_id, count));
        self
    }

    pub fn npc(mut self, mob_id: MobId, count: RangeInclusive<u32>) -> Self {
        self.npc_spawns.push((mob_id, count));
        self
    }

    pub fn npc_chance(mut self, mob_id: MobId, probability: f64) -> Self {
        self.npc_chances.push((mob_id, probability));
        self
    }
}

impl SpawnTable {
    pub fn new() -> SpawnTableBuilder {
        Self::builder()
    }

    pub fn empty() -> SpawnTableBuilder {
        Self::builder()
    }

    pub fn weighted_mobs(&self) -> Vec<MobSpawnEntry> {
        self.entries
            .iter()
            .map(|e| {
                let SpawnEntityType::Mob(mob_id) = e.entity_type;
                MobSpawnEntry {
                    mob_id,
                    weight: e.weight,
                }
            })
            .collect()
    }

    pub fn mob_count(&self) -> &RangeInclusive<u32> {
        &self.mob_count
    }

    pub fn guaranteed_mobs(&self) -> &[(MobId, u32)] {
        &self.guaranteed_mobs
    }

    pub fn npc_spawns(&self) -> &[(MobId, RangeInclusive<u32>)] {
        &self.npc_spawns
    }

    pub fn npc_chances(&self) -> &[(MobId, f64)] {
        &self.npc_chances
    }

    pub fn chest(&self) -> &RangeInclusive<u32> {
        &self.chest
    }

    pub fn stairs(&self) -> &RangeInclusive<u32> {
        &self.stairs
    }

    pub fn rock(&self) -> &RangeInclusive<u32> {
        &self.rock
    }

    pub fn forge(&self) -> &RangeInclusive<u32> {
        &self.forge
    }

    pub fn anvil(&self) -> &RangeInclusive<u32> {
        &self.anvil
    }

    pub fn forge_chance(&self) -> Option<f64> {
        self.forge_chance
    }

    pub fn anvil_chance(&self) -> Option<f64> {
        self.anvil_chance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        if crate::mob::data::specs_loaded() {
            return;
        }
        crate::item::data::init();
        crate::mob::data::init();
    }

    #[test]
    fn mob_entry_stores_size_from_spec() {
        init();
        let table = SpawnTable::new().mob(MobId::Goblin, 1).build();

        assert_eq!(table.entries.len(), 1);
        let entry = &table.entries[0];
        assert_eq!(entry.entity_type, SpawnEntityType::Mob(MobId::Goblin));
        assert_eq!(entry.weight, 1);
        assert_eq!(entry.size, MobId::Goblin.spec().entity_size);
    }

    #[test]
    fn chest_range() {
        let table = SpawnTable::new().chest(2..=5).build();
        assert_eq!(*table.chest(), 2..=5);
    }

    #[test]
    fn weighted_mobs() {
        init();
        let table = SpawnTable::new()
            .mob(MobId::Goblin, 5)
            .mob(MobId::Slime, 3)
            .mob_count(3..=4)
            .build();

        let mobs = table.weighted_mobs();
        assert_eq!(mobs.len(), 2);
        assert_eq!(mobs[0].mob_id, MobId::Goblin);
        assert_eq!(mobs[0].weight, 5);
        assert_eq!(mobs[1].mob_id, MobId::Slime);
        assert_eq!(mobs[1].weight, 3);
        assert_eq!(*table.mob_count(), 3..=4);
    }

    #[test]
    fn guaranteed_mobs() {
        let table = SpawnTable::new()
            .guaranteed_mob(MobId::DwarfKing, 1)
            .guaranteed_mob(MobId::DwarfWarrior, 2)
            .build();

        let mobs = table.guaranteed_mobs();
        assert_eq!(mobs.len(), 2);
        assert!(mobs.contains(&(MobId::DwarfKing, 1)));
        assert!(mobs.contains(&(MobId::DwarfWarrior, 2)));
    }

    #[test]
    fn npc_spawns() {
        let table = SpawnTable::new()
            .npc(MobId::Merchant, 1..=1)
            .npc_chance(MobId::Merchant, 0.33)
            .build();

        assert_eq!(table.npc_spawns().len(), 1);
        assert_eq!(table.npc_spawns()[0], (MobId::Merchant, 1..=1));
        assert_eq!(table.npc_chances().len(), 1);
        assert_eq!(table.npc_chances()[0].0, MobId::Merchant);
    }

    #[test]
    fn crafting_stations() {
        let table = SpawnTable::new()
            .forge(1..=1)
            .anvil(2..=2)
            .forge_chance(0.33)
            .anvil_chance(0.5)
            .build();

        assert_eq!(*table.forge(), 1..=1);
        assert_eq!(*table.anvil(), 2..=2);
        assert_eq!(table.forge_chance(), Some(0.33));
        assert_eq!(table.anvil_chance(), Some(0.5));
    }

    #[test]
    fn empty_spawn_table() {
        let table = SpawnTable::empty().build();

        assert_eq!(*table.chest().end(), 0);
        assert_eq!(*table.stairs().end(), 0);
        assert!(table.weighted_mobs().is_empty());
        assert!(table.guaranteed_mobs().is_empty());
    }

    #[test]
    fn rock_range() {
        let table = SpawnTable::new().rock(3..=3).build();
        assert_eq!(*table.rock(), 3..=3);
    }
}

