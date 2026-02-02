use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use tracing::instrument;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::constants::DEFAULT_TILE_SIZE;
use crate::dungeon::spawn::{MobSpawnEntry, SpawnTable};
use crate::dungeon::tile_components::{can_have_entity, is_door};
use crate::dungeon::{
    ChestEntity, CraftingStationEntity, DoorEntity, DungeonEntityMarker, EntitySize, MobEntity,
    NpcEntity, RockEntity, StairsEntity, TileWorldSize,
};
use crate::rock::RockType;
use crate::ui::screens::FloorRoot;

const CHEST_VARIANT_COUNT: u8 = 4;
const ROCK_SPRITE_VARIANT_COUNT: u8 = 2;

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

    fn spawn_entity<C: Component>(&self, commands: &mut Commands, world_pos: Vec2, component: C) {
        let marker = DungeonEntityMarker {
            pos: world_pos,
            size: self.entity_size(),
        };
        let entity = commands.spawn((marker, component)).id();
        if let Some(root) = self.floor_root {
            commands.entity(entity).insert(ChildOf(root));
        }
    }
}

#[instrument(level = "debug", skip_all)]
pub fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    spawn_tiles: Query<(&TilePos, &can_have_entity)>,
    door_tiles: Query<(&TilePos, &is_door)>,
    tilemap_query: TilemapQuery,
    floor_root_query: Query<Entity, With<FloorRoot>>,
    tile_world_size: Option<Res<TileWorldSize>>,
    config: Option<Res<SpawnTable>>,
) {
    let tile_size = tile_world_size.map(|t| t.0).unwrap_or(DEFAULT_TILE_SIZE);
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

    commands.remove_resource::<SpawnTable>();
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

fn spawn_n_entities<R: Rng, C: Component, F>(
    commands: &mut Commands,
    count: u32,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut R,
    mut create_component: F,
) where
    F: FnMut(&mut R) -> C,
{
    for _ in 0..count {
        let Some(tile_pos) = find_spawn_position(available, used, rng) else {
            break;
        };
        let world_pos = ctx.tile_to_world(tile_pos);
        let component = create_component(rng);
        ctx.spawn_entity(commands, world_pos, component);
        used.push(tile_pos);
    }
}

fn spawn_doors(
    commands: &mut Commands,
    door_tiles: &Query<(&TilePos, &is_door)>,
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
) {
    for (tile_pos, _) in door_tiles.iter() {
        let world_pos = ctx.tile_to_world(*tile_pos);
        ctx.spawn_entity(commands, world_pos, DoorEntity);
        used.push(*tile_pos);
    }
}

fn spawn_chests(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.chest().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.chest().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| ChestEntity {
        variant: rng.gen_range(0..CHEST_VARIANT_COUNT),
    });
}

fn spawn_stairs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.stairs().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.stairs().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |_| StairsEntity);
}

fn spawn_rocks(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.rock().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.rock().clone());

    const ROCK_TYPES: [RockType; 4] =
        [RockType::Coal, RockType::Copper, RockType::Iron, RockType::Gold];

    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| {
        let rock_type = ROCK_TYPES[rng.gen_range(0..ROCK_TYPES.len())];
        RockEntity {
            rock_type,
            sprite_variant: rng.gen_range(0..ROCK_SPRITE_VARIANT_COUNT),
        }
    });
}

fn spawn_crafting_stations(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    let forge_count = if *config.forge().end() > 0 {
        rng.gen_range(config.forge().clone())
    } else if let Some(prob) = config.forge_chance() {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    spawn_n_entities(commands, forge_count, available, used, ctx, rng, |_| {
        CraftingStationEntity {
            station_type: CraftingStationType::Forge,
        }
    });

    let anvil_count = if *config.anvil().end() > 0 {
        rng.gen_range(config.anvil().clone())
    } else if let Some(prob) = config.anvil_chance() {
        if rng.gen_bool(prob) { 1 } else { 0 }
    } else {
        0
    };

    spawn_n_entities(commands, anvil_count, available, used, ctx, rng, |_| {
        CraftingStationEntity {
            station_type: CraftingStationType::Anvil,
        }
    });
}

fn spawn_npcs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    for (mob_id, count_range) in config.npc_spawns() {
        let count = rng.gen_range(count_range.clone());
        let mob_id = *mob_id;
        spawn_n_entities(commands, count, available, used, ctx, rng, |_| NpcEntity { mob_id });
    }

    for (mob_id, probability) in config.npc_chances() {
        if rng.gen_bool(*probability) {
            let mob_id = *mob_id;
            spawn_n_entities(commands, 1, available, used, ctx, rng, |_| NpcEntity { mob_id });
        }
    }
}

fn spawn_mobs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[TilePos],
    used: &mut Vec<TilePos>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    for (mob_id, count) in config.guaranteed_mobs() {
        let mob_id = *mob_id;
        spawn_n_entities(commands, *count, available, used, ctx, rng, |_| MobEntity { mob_id });
    }

    let weighted_mobs = config.weighted_mobs();
    if weighted_mobs.is_empty() || *config.mob_count().end() == 0 {
        return;
    }

    let total_weight: u32 = weighted_mobs.iter().map(|e| e.weight).sum();
    if total_weight == 0 {
        return;
    }

    let count = rng.gen_range(config.mob_count().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| {
        let Some(entry) = weighted_select(&weighted_mobs, total_weight, rng) else {
            return MobEntity { mob_id: weighted_mobs[0].mob_id };
        };
        MobEntity { mob_id: entry.mob_id }
    });
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
    use crate::mob::MobId;

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
