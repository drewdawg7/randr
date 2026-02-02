use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::tile_components::{can_have_entity, is_door};
use crate::dungeon::{DungeonEntity, DungeonEntityMarker, EntitySize, TileWorldSize};
use crate::mob::MobId;
use crate::rock::RockType;
use crate::ui::screens::FloorRoot;

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

type TilemapQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static TilemapSize,
        &'static TilemapGridSize,
        &'static TilemapTileSize,
        &'static TilemapType,
        &'static TilemapAnchor,
        &'static GlobalTransform,
    ),
    With<TiledTilemap>,
>;

struct SpawnContext<'a> {
    tile_size: f32,
    tilemap: Option<(
        &'a TilemapSize,
        &'a TilemapGridSize,
        &'a TilemapTileSize,
        &'a TilemapType,
        &'a TilemapAnchor,
        &'a GlobalTransform,
    )>,
    floor_root: Option<Entity>,
}

impl SpawnContext<'_> {
    fn tile_to_world(&self, pos: TilePos) -> Vec2 {
        if let Some((map_size, grid_size, tile_size, map_type, anchor, gt)) = self.tilemap {
            let local_pos = pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
            gt.transform_point(local_pos.extend(0.0)).truncate()
        } else {
            Vec2::new(pos.x as f32 * self.tile_size, pos.y as f32 * self.tile_size)
        }
    }

    fn entity_size(&self) -> EntitySize {
        EntitySize::new(self.tile_size, self.tile_size)
    }

    fn spawn_entity(&self, commands: &mut Commands, marker: DungeonEntityMarker) {
        let entity = commands.spawn(marker).id();
        if let Some(root) = self.floor_root {
            commands.entity(entity).insert(ChildOf(root));
        }
    }
}

pub fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    spawn_tiles: Query<(&TilePos, &can_have_entity)>,
    door_tiles: Query<(&TilePos, &is_door)>,
    tilemap_query: TilemapQuery,
    floor_root_query: Query<Entity, With<FloorRoot>>,
    tile_world_size: Option<Res<TileWorldSize>>,
    config: Option<Res<FloorSpawnConfig>>,
) {
    let tile_size = tile_world_size.map(|t| t.0).unwrap_or(32.0);
    let tilemap = tilemap_query.single().ok();
    let floor_root = floor_root_query.single().ok();
    let ctx = SpawnContext { tile_size, tilemap, floor_root };

    let mut used_positions: Vec<TilePos> = Vec::new();

    spawn_doors(&mut commands, &door_tiles, &mut used_positions, &ctx);

    let Some(config) = config else {
        return;
    };

    let mut rng = rand::thread_rng();

    let available: Vec<TilePos> = spawn_tiles
        .iter()
        .filter(|(_, can_spawn)| can_spawn.0)
        .map(|(pos, _)| *pos)
        .collect();

    if available.is_empty() {
        return;
    }

    spawn_chests(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);
    spawn_stairs(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);
    spawn_rocks(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);
    spawn_crafting_stations(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);
    spawn_npcs(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);
    spawn_mobs(&mut commands, &config, &available, &mut used_positions, &ctx, &mut rng);

    commands.remove_resource::<FloorSpawnConfig>();
}

fn find_spawn_position(
    available: &[TilePos],
    used: &[TilePos],
    rng: &mut impl Rng,
) -> Option<TilePos> {
    let candidates: Vec<_> = available
        .iter()
        .filter(|pos| !used.contains(pos))
        .collect();

    candidates.choose(rng).copied().copied()
}

fn spawn_doors(
    commands: &mut Commands,
    door_tiles: &Query<(&TilePos, &is_door)>,
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
) {
    let size = ctx.entity_size();

    for (tile_pos, _) in door_tiles.iter() {
        let world_pos = ctx.tile_to_world(*tile_pos);
        let entity_type = DungeonEntity::Door { size };
        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(*tile_pos);
    }
}

fn spawn_chests(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.chest.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.chest.clone());
    let size = ctx.entity_size();

    for _ in 0..count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

        let entity_type = DungeonEntity::Chest {
            variant: rng.gen_range(0..4),
            size,
        };

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
    }
}

fn spawn_stairs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.stairs.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.stairs.clone());
    let size = ctx.entity_size();

    for _ in 0..count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

        let entity_type = DungeonEntity::Stairs { size };

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
    }
}

fn spawn_rocks(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.rock.end() == 0 {
        return;
    }

    let count = rng.gen_range(config.rock.clone());
    let size = ctx.entity_size();

    for _ in 0..count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

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

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
    }
}

fn spawn_crafting_stations(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    let size = ctx.entity_size();

    let forge_count = if *config.forge.end() > 0 {
        rng.gen_range(config.forge.clone())
    } else if let Some(prob) = config.forge_chance {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    for _ in 0..forge_count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

        let entity_type = DungeonEntity::CraftingStation {
            station_type: CraftingStationType::Forge,
            size,
        };

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
    }

    let anvil_count = if *config.anvil.end() > 0 {
        rng.gen_range(config.anvil.clone())
    } else if let Some(prob) = config.anvil_chance {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    for _ in 0..anvil_count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

        let entity_type = DungeonEntity::CraftingStation {
            station_type: CraftingStationType::Anvil,
            size,
        };

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
    }
}

fn spawn_npcs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    let size = ctx.entity_size();

    for (mob_id, count_range) in &config.npc_spawns {
        let count = rng.gen_range(count_range.clone());
        for _ in 0..count {
            let Some(tile_pos) = find_spawn_position(available, used, rng) else {
                break;
            };

            let world_pos = ctx.tile_to_world(tile_pos);

            let entity_type = DungeonEntity::Npc { mob_id: *mob_id, size };

            ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
            used.push(tile_pos);
        }
    }

    for (mob_id, probability) in &config.npc_chances {
        if rng.gen_bool(*probability) {
            let Some(tile_pos) = find_spawn_position(available, used, rng) else {
                continue;
            };

            let world_pos = ctx.tile_to_world(tile_pos);

            let entity_type = DungeonEntity::Npc { mob_id: *mob_id, size };

            ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
            used.push(tile_pos);
        }
    }
}

fn spawn_mobs(
    commands: &mut Commands,
    config: &FloorSpawnConfig,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    for (mob_id, count) in &config.guaranteed_mobs {
        let size = mob_id.spec().entity_size;
        for _ in 0..*count {
            let Some(tile_pos) = find_spawn_position(available, used, rng) else {
                break;
            };

            let world_pos = ctx.tile_to_world(tile_pos);

            let entity_type = DungeonEntity::Mob { mob_id: *mob_id, size };

            ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
            used.push(tile_pos);
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

        let size = entry.mob_id.spec().entity_size;
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };

        let world_pos = ctx.tile_to_world(tile_pos);

        let entity_type = DungeonEntity::Mob { mob_id: entry.mob_id, size };

        ctx.spawn_entity(commands, DungeonEntityMarker { pos: world_pos, entity_type });
        used.push(tile_pos);
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
