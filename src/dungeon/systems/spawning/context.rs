use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::{DepthSorting, DungeonEntityMarker, EntitySize, TilemapInfo};

pub const POSITION_PROXIMITY_THRESHOLD: f32 = 1.0;

pub type TilemapQuery<'w, 's> = Query<
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

pub struct TilemapData<'a> {
    pub map_size: &'a TilemapSize,
    pub grid_size: &'a TilemapGridSize,
    pub tile_size: &'a TilemapTileSize,
    pub map_type: &'a TilemapType,
    pub anchor: &'a TilemapAnchor,
    pub transform: &'a GlobalTransform,
}

impl TilemapData<'_> {
    pub fn tile_to_world(&self, pos: &TilePos) -> Vec2 {
        let local = pos.center_in_world(
            self.map_size,
            self.grid_size,
            self.tile_size,
            self.map_type,
            self.anchor,
        );
        self.transform.transform_point(local.extend(0.0)).truncate()
    }
}

pub struct SpawnContext {
    pub tile_size: f32,
    pub floor_root: Option<Entity>,
}

impl SpawnContext {
    pub fn entity_size(&self) -> EntitySize {
        EntitySize::new(self.tile_size, self.tile_size)
    }

    pub fn spawn_entity<C: Component>(&self, commands: &mut Commands, world_pos: Vec2, component: C) {
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

pub fn compute_tilemap_info(
    tilemap_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    transform: &GlobalTransform,
) -> TilemapInfo {
    let tile_vec = Vec2::new(tile_size.x, tile_size.y);
    let map_tiles = Vec2::new(tilemap_size.x as f32, tilemap_size.y as f32);
    let world_size = map_tiles * tile_vec;
    let local_center = world_size / 2.0;
    let center = transform.transform_point(local_center.extend(0.0)).truncate();

    TilemapInfo { tile_size: tile_vec, world_size, center }
}

pub fn compute_depth_sorting(tilemap_size: &TilemapSize, tile_size: &TilemapTileSize) -> DepthSorting {
    DepthSorting::from_map(tilemap_size.y as f32, tile_size.y)
}

pub fn is_position_used(pos: Vec2, used: &[Vec2]) -> bool {
    used.iter().any(|used_pos| pos.distance(*used_pos) < POSITION_PROXIMITY_THRESHOLD)
}

pub fn find_spawn_position(
    available: &[Vec2],
    used: &[Vec2],
    rng: &mut impl Rng,
) -> Option<Vec2> {
    let candidates: Vec<_> = available
        .iter()
        .filter(|pos| !is_position_used(**pos, used))
        .collect();

    candidates.choose(rng).copied().copied()
}

pub fn spawn_n_entities<R: Rng, C: Component, F>(
    commands: &mut Commands,
    count: u32,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut R,
    mut create_component: F,
) where
    F: FnMut(&mut R) -> C,
{
    for _ in 0..count {
        let Some(world_pos) = find_spawn_position(available, used, rng) else {
            break;
        };
        let component = create_component(rng);
        ctx.spawn_entity(commands, world_pos, component);
        used.push(world_pos);
    }
}
