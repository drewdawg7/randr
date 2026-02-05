use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::spawn::SpawnTable;
use crate::dungeon::tile_components::is_door;
use crate::dungeon::{ChestEntity, DoorEntity, RockEntity, StairsEntity};
use crate::rock::RockType;

use super::context::{spawn_n_entities, SpawnContext, TilemapData};

pub fn spawn_doors(
    commands: &mut Commands,
    door_tiles: &Query<(&TilePos, &is_door)>,
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    tilemap: &TilemapData,
) {
    for (tile_pos, _) in door_tiles.iter() {
        let world_pos = tilemap.tile_to_world(tile_pos);
        ctx.spawn_entity(commands, world_pos, DoorEntity);
        used.push(world_pos);
    }
}

pub fn spawn_chests(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.chest().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.chest().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |_| ChestEntity);
}

pub fn spawn_stairs(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.stairs().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.stairs().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |_| StairsEntity);
}

pub fn spawn_rocks(
    commands: &mut Commands,
    config: &SpawnTable,
    available: &[Vec2],
    used: &mut Vec<Vec2>,
    ctx: &SpawnContext,
    rng: &mut impl Rng,
) {
    if *config.rock().end() == 0 {
        return;
    }

    let count = rng.gen_range(config.rock().clone());

    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| {
        let rock_type = *RockType::ALL.choose(rng).unwrap_or(&RockType::Coal);
        RockEntity {
            rock_type,
            sprite_variant: rng.gen_range(0..RockType::SPRITE_VARIANT_COUNT),
        }
    });
}
