mod context;
mod crafting;
mod entities;
mod mobs;
mod npcs;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::dungeon::spawn::SpawnTable;
use crate::dungeon::tile_components::{can_have_entity, is_door};
use crate::dungeon::TileWorldSize;
use crate::ui::screens::FloorRoot;

use context::{compute_depth_sorting, compute_tilemap_info, SpawnContext, TilemapData, TilemapQuery};
use crafting::spawn_crafting_stations;
use entities::{spawn_chests, spawn_doors, spawn_rocks, spawn_stairs};
use mobs::spawn_mobs;
use npcs::spawn_npcs;

#[instrument(level = "debug", skip_all, fields(spawn_count = spawn_tiles.iter().count(), door_count = door_tiles.iter().count()))]
pub fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    spawn_tiles: Query<(&TilePos, &can_have_entity)>,
    door_tiles: Query<(&TilePos, &is_door)>,
    tilemap_query: TilemapQuery,
    floor_root_query: Query<Entity, With<FloorRoot>>,
    config: Option<Res<SpawnTable>>,
) {
    let Some((map_size, grid_size, tilemap_tile_size, map_type, anchor, transform)) =
        tilemap_query.single().ok()
    else {
        return;
    };

    let tile_size = tilemap_tile_size.x;
    commands.insert_resource(TileWorldSize(tile_size));

    let floor_root = floor_root_query.single().ok();
    let ctx = SpawnContext { tile_size, floor_root };

    let tilemap = TilemapData {
        map_size,
        grid_size,
        tile_size: tilemap_tile_size,
        map_type,
        anchor,
        transform,
    };

    let info = compute_tilemap_info(map_size, tilemap_tile_size, transform);
    commands.insert_resource(info);

    let depth_sorting = compute_depth_sorting(map_size, tilemap_tile_size);
    commands.insert_resource(depth_sorting);

    let mut used_positions: Vec<Vec2> = Vec::new();

    spawn_doors(&mut commands, &door_tiles, &mut used_positions, &ctx, &tilemap);

    let Some(config) = config else {
        return;
    };

    let mut rng = rand::thread_rng();

    let available: Vec<Vec2> = spawn_tiles
        .iter()
        .filter(|(_, can_spawn)| can_spawn.0)
        .map(|(tile_pos, _)| tilemap.tile_to_world(tile_pos))
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
