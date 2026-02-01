use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::tile_components::{can_have_entity, is_door, is_solid};
use crate::dungeon::tile_index::TileIndex;
use crate::dungeon::{DungeonEntity, DungeonEntityMarker, GridOccupancy, GridSize};
use crate::mob::MobId;
use crate::rock::RockType;

#[derive(Debug, Clone)]
pub struct MobSpawnEntry {
    pub mob_id: MobId,
    pub weight: u32,
}

#[derive(Resource, Debug, Clone)]
pub struct FloorSpawnConfig {
    pub chest: RangeInclusive<u32>,
    pub stairs: RangeInclusive<u32>,
    pub rock: RangeInclusive<u32>,
    pub forge: RangeInclusive<u32>,
    pub anvil: RangeInclusive<u32>,
    pub forge_chance: Option<f64>,
    pub anvil_chance: Option<f64>,
    pub weighted_mobs: Vec<MobSpawnEntry>,
    pub mob_count: RangeInclusive<u32>,
    pub guaranteed_mobs: Vec<(MobId, u32)>,
    pub npc_spawns: Vec<(MobId, RangeInclusive<u32>)>,
    pub npc_chances: Vec<(MobId, f64)>,
}

impl Default for FloorSpawnConfig {
    fn default() -> Self {
        Self {
            chest: 0..=0,
            stairs: 0..=0,
            rock: 0..=0,
            forge: 0..=0,
            anvil: 0..=0,
            forge_chance: None,
            anvil_chance: None,
            weighted_mobs: Vec::new(),
            mob_count: 0..=0,
            guaranteed_mobs: Vec::new(),
            npc_spawns: Vec::new(),
            npc_chances: Vec::new(),
        }
    }
}

pub fn build_tile_index(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    solid_tiles: Query<&TilePos, With<is_solid>>,
    door_tiles: Query<&TilePos, With<is_door>>,
) {
    let mut index = TileIndex::default();
    for pos in solid_tiles.iter() {
        index.solid.insert((pos.x, pos.y));
    }
    for pos in door_tiles.iter() {
        index.doors.insert((pos.x, pos.y));
    }
    commands.insert_resource(index);
}

pub fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    spawn_tiles: Query<&TilePos, With<can_have_entity>>,
    config: Option<Res<FloorSpawnConfig>>,
    occupancy: Option<ResMut<GridOccupancy>>,
) {
    let Some(config) = config else {
        return;
    };
    let Some(mut occupancy) = occupancy else {
        return;
    };

    let mut rng = rand::thread_rng();

    let available: Vec<TilePos> = spawn_tiles.iter().copied().collect();

    if available.is_empty() {
        return;
    }

    let mut used_positions: Vec<TilePos> = Vec::new();

    spawn_chests(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);
    spawn_stairs(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);
    spawn_rocks(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);
    spawn_crafting_stations(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);
    spawn_npcs(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);
    spawn_mobs(&mut commands, &config, &available, &mut used_positions, &mut occupancy, &mut rng);

    commands.remove_resource::<FloorSpawnConfig>();
}

fn find_spawn_position(
    available: &[TilePos],
    used: &[TilePos],
    occupancy: &GridOccupancy,
    size: GridSize,
    rng: &mut impl Rng,
) -> Option<TilePos> {
    let candidates: Vec<_> = available
        .iter()
        .filter(|pos| !used.contains(pos) && occupancy.can_place(**pos, size))
        .collect();

    candidates.choose(rng).copied().copied()
}

fn spawn_chests(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    if *config.chest.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.chest.clone());
    let size = GridSize::single();

    for _ in 0..count {
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let entity_type = DungeonEntity::Chest {
            variant: rng.gen_range(0..4),
            size,
        };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }
}

fn spawn_stairs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    if *config.stairs.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.stairs.clone());
    let size = GridSize::single();

    for _ in 0..count {
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let entity_type = DungeonEntity::Stairs { size };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }
}

fn spawn_rocks(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    if *config.rock.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.rock.clone());
    let size = GridSize::single();

    for _ in 0..count {
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let rock_type = match rng.gen_range(0..4u8) {
            0 => RockType::Coal,
            1 => RockType::Copper,
            2 => RockType::Iron,
            _ => RockType::Gold,
        };

        let entity_type = DungeonEntity::Rock {
            rock_type,
            sprite_variant: rng.gen_range(0..2u8),
            size,
        };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }
}

fn spawn_crafting_stations(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    let size = GridSize::single();

    let forge_count = if *config.forge.end() > 0 {
        rng.gen_range(config.forge.clone())
    } else if let Some(prob) = config.forge_chance {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    for _ in 0..forge_count {
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let entity_type = DungeonEntity::CraftingStation {
            station_type: CraftingStationType::Forge,
            size,
        };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }

    let anvil_count = if *config.anvil.end() > 0 {
        rng.gen_range(config.anvil.clone())
    } else if let Some(prob) = config.anvil_chance {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    for _ in 0..anvil_count {
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let entity_type = DungeonEntity::CraftingStation {
            station_type: CraftingStationType::Anvil,
            size,
        };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }
}

fn spawn_npcs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    let size = GridSize::single();

    for (mob_id, count_range) in &config.npc_spawns {
        let count = rng.gen_range(count_range.clone());
        for _ in 0..count {
            let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
                break;
            };

            let entity_type = DungeonEntity::Npc { mob_id: *mob_id, size };

            let entity = commands
                .spawn(DungeonEntityMarker { pos, entity_type })
                .id();

            occupancy.occupy(pos, size, entity);
            used.push(pos);
        }
    }

    for (mob_id, probability) in &config.npc_chances {
        if rng.gen_bool(*probability) {
            let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
                continue;
            };

            let entity_type = DungeonEntity::Npc { mob_id: *mob_id, size };

            let entity = commands
                .spawn(DungeonEntityMarker { pos, entity_type })
                .id();

            occupancy.occupy(pos, size, entity);
            used.push(pos);
        }
    }
}

fn spawn_mobs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    occupancy: &mut GridOccupancy,
    rng: &mut impl Rng,
) {
    for (mob_id, count) in &config.guaranteed_mobs {
        let size = mob_id.spec().grid_size;
        for _ in 0..*count {
            let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
                break;
            };

            let entity_type = DungeonEntity::Mob { mob_id: *mob_id, size };

            let entity = commands
                .spawn(DungeonEntityMarker { pos, entity_type })
                .id();

            occupancy.occupy(pos, size, entity);
            used.push(pos);
        }
    }

    if config.weighted_mobs.is_empty() || *config.mob_count.end() == 0 {
        return;
    }

    let total_weight: u32 = config.weighted_mobs.iter().map(|e| e.weight).sum();
    if total_weight == 0 {
        return;
    }

    let count = rng.gen_range(config.mob_count.clone());

    for _ in 0..count {
        let Some(entry) = weighted_select(&config.weighted_mobs, total_weight, rng) else {
            continue;
        };

        let size = entry.mob_id.spec().grid_size;
        let Some(pos) = find_spawn_position(available, used, occupancy, size, rng) else {
            break;
        };

        let entity_type = DungeonEntity::Mob { mob_id: entry.mob_id, size };

        let entity = commands
            .spawn(DungeonEntityMarker { pos, entity_type })
            .id();

        occupancy.occupy(pos, size, entity);
        used.push(pos);
    }
}

fn weighted_select<'a>(
    entries: &'a [MobSpawnEntry],
    total_weight: u32,
    rng: &mut impl Rng,
) -> Option<&'a MobSpawnEntry> {
    if entries.is_empty() {
        return None;
    }

    let roll = rng.gen_range(0..total_weight);
    let mut cumulative = 0;

    for entry in entries {
        cumulative += entry.weight;
        if roll < cumulative {
            return Some(entry);
        }
    }

    entries.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_spawn_config_default() {
        let config = FloorSpawnConfig::default();
        assert_eq!(*config.chest.end(), 0);
        assert_eq!(*config.stairs.end(), 0);
        assert!(config.weighted_mobs.is_empty());
    }

    #[test]
    fn weighted_select_returns_entry() {
        let entries = vec![
            MobSpawnEntry { mob_id: MobId::Goblin, weight: 5 },
            MobSpawnEntry { mob_id: MobId::Slime, weight: 3 },
        ];

        let mut rng = rand::thread_rng();
        let result = weighted_select(&entries, 8, &mut rng);
        assert!(result.is_some());
    }

    #[test]
    fn weighted_select_empty_returns_none() {
        let entries: Vec<MobSpawnEntry> = vec![];
        let mut rng = rand::thread_rng();
        let result = weighted_select(&entries, 0, &mut rng);
        assert!(result.is_none());
    }
}
