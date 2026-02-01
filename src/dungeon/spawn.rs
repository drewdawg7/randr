use std::ops::RangeInclusive;

use bon::Builder;

use super::grid::GridSize;
use super::systems::spawning::{FloorSpawnConfig, MobSpawnEntry};
use crate::mob::MobId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEntityType {
    Mob(MobId),
}

#[derive(Debug, Clone)]
pub struct SpawnEntry {
    pub entity_type: SpawnEntityType,
    pub weight: u32,
    pub size: GridSize,
}

#[derive(Debug, Clone, Builder)]
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
        let size = mob_id.spec().grid_size;
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

    pub fn to_config(&self) -> FloorSpawnConfig {
        FloorSpawnConfig {
            chest: self.chest.clone(),
            stairs: self.stairs.clone(),
            rock: self.rock.clone(),
            forge: self.forge.clone(),
            anvil: self.anvil.clone(),
            forge_chance: self.forge_chance,
            anvil_chance: self.anvil_chance,
            weighted_mobs: self
                .entries
                .iter()
                .map(|e| {
                    let SpawnEntityType::Mob(mob_id) = e.entity_type;
                    MobSpawnEntry {
                        mob_id,
                        weight: e.weight,
                    }
                })
                .collect(),
            mob_count: self.mob_count.clone(),
            guaranteed_mobs: self.guaranteed_mobs.clone(),
            npc_spawns: self.npc_spawns.clone(),
            npc_chances: self.npc_chances.clone(),
        }
    }
}

impl From<&SpawnTable> for FloorSpawnConfig {
    fn from(table: &SpawnTable) -> Self {
        table.to_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mob_entry_stores_size_from_spec() {
        let table = SpawnTable::new().mob(MobId::Goblin, 1).build();

        assert_eq!(table.entries.len(), 1);
        let entry = &table.entries[0];
        assert_eq!(entry.entity_type, SpawnEntityType::Mob(MobId::Goblin));
        assert_eq!(entry.weight, 1);
        assert_eq!(entry.size, MobId::Goblin.spec().grid_size);
    }

    #[test]
    fn to_config_converts_chest_range() {
        let table = SpawnTable::new().chest(2..=5).build();
        let config = table.to_config();

        assert_eq!(config.chest, 2..=5);
    }

    #[test]
    fn to_config_converts_weighted_mobs() {
        let table = SpawnTable::new()
            .mob(MobId::Goblin, 5)
            .mob(MobId::Slime, 3)
            .mob_count(3..=4)
            .build();
        let config = table.to_config();

        assert_eq!(config.weighted_mobs.len(), 2);
        assert_eq!(config.weighted_mobs[0].mob_id, MobId::Goblin);
        assert_eq!(config.weighted_mobs[0].weight, 5);
        assert_eq!(config.weighted_mobs[1].mob_id, MobId::Slime);
        assert_eq!(config.weighted_mobs[1].weight, 3);
        assert_eq!(config.mob_count, 3..=4);
    }

    #[test]
    fn to_config_converts_guaranteed_mobs() {
        let table = SpawnTable::new()
            .guaranteed_mob(MobId::DwarfKing, 1)
            .guaranteed_mob(MobId::DwarfWarrior, 2)
            .build();
        let config = table.to_config();

        assert_eq!(config.guaranteed_mobs.len(), 2);
        assert!(config.guaranteed_mobs.contains(&(MobId::DwarfKing, 1)));
        assert!(config.guaranteed_mobs.contains(&(MobId::DwarfWarrior, 2)));
    }

    #[test]
    fn to_config_converts_npc_spawns() {
        let table = SpawnTable::new()
            .npc(MobId::Merchant, 1..=1)
            .npc_chance(MobId::Merchant, 0.33)
            .build();
        let config = table.to_config();

        assert_eq!(config.npc_spawns.len(), 1);
        assert_eq!(config.npc_spawns[0], (MobId::Merchant, 1..=1));
        assert_eq!(config.npc_chances.len(), 1);
        assert_eq!(config.npc_chances[0].0, MobId::Merchant);
    }

    #[test]
    fn to_config_converts_crafting_stations() {
        let table = SpawnTable::new()
            .forge(1..=1)
            .anvil(2..=2)
            .forge_chance(0.33)
            .anvil_chance(0.5)
            .build();
        let config = table.to_config();

        assert_eq!(config.forge, 1..=1);
        assert_eq!(config.anvil, 2..=2);
        assert_eq!(config.forge_chance, Some(0.33));
        assert_eq!(config.anvil_chance, Some(0.5));
    }

    #[test]
    fn empty_spawn_table_creates_empty_config() {
        let table = SpawnTable::empty().build();
        let config = table.to_config();

        assert_eq!(*config.chest.end(), 0);
        assert_eq!(*config.stairs.end(), 0);
        assert!(config.weighted_mobs.is_empty());
        assert!(config.guaranteed_mobs.is_empty());
    }

    #[test]
    fn from_trait_works() {
        let table = SpawnTable::new().rock(3..=3).build();
        let config: FloorSpawnConfig = (&table).into();

        assert_eq!(config.rock, 3..=3);
    }
}

