use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TilemapGridSize, TilemapSize};
use tracing::instrument;

use crate::dungeon::events::{FloorTransition, MoveResult, PlayerMoveIntent};
use crate::dungeon::grid::rects_overlap;
use crate::dungeon::tile_components::{is_door, is_solid};
use crate::dungeon::{DungeonEntity, DungeonEntityMarker, DungeonState, EntitySize, Occupancy};
use crate::input::NavigationDirection;

#[instrument(level = "debug", skip_all, fields(player_pos = ?state.player_pos, player_size = ?state.player_size))]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut state: ResMut<DungeonState>,
    occupancy: Option<ResMut<Occupancy>>,
    entity_query: Query<&DungeonEntityMarker>,
    solid_tiles: Query<&GlobalTransform, (With<is_solid>, Without<is_door>)>,
    door_tiles: Query<&GlobalTransform, With<is_door>>,
    tilemap_query: Query<(&TilemapGridSize, &GlobalTransform), With<TilemapSize>>,
) {
    let Some(mut occupancy) = occupancy else {
        return;
    };

    let Ok((grid_size, map_transform)) = tilemap_query.single() else {
        return;
    };

    let scale = map_transform.to_scale_rotation_translation().0.x;
    let tile_world_size = grid_size.x * scale;
    let tile_size = EntitySize::single(tile_world_size);

    for event in events.read() {
        let delta: Vec2 = match event.direction {
            NavigationDirection::Up => Vec2::new(0.0, tile_world_size),
            NavigationDirection::Down => Vec2::new(0.0, -tile_world_size),
            NavigationDirection::Left => Vec2::new(-tile_world_size, 0.0),
            NavigationDirection::Right => Vec2::new(tile_world_size, 0.0),
        };

        let new_pos = state.player_pos + delta;

        if overlaps_any_tile(&door_tiles, new_pos, state.player_size, tile_size) {
            transition_events.write(FloorTransition::EnterDoor);
            return;
        }

        if overlaps_any_tile(&solid_tiles, new_pos, state.player_size, tile_size) {
            result_events.write(MoveResult::Blocked);
            continue;
        }

        if let Some((entity_type, entity, pos)) =
            check_entity_collision(&occupancy, &entity_query, new_pos, state.player_size)
        {
            match entity_type {
                DungeonEntity::Mob { mob_id, .. } => {
                    result_events.write(MoveResult::TriggeredCombat { mob_id, entity, pos });
                }
                DungeonEntity::Door { .. } => {
                    transition_events.write(FloorTransition::EnterDoor);
                }
                DungeonEntity::Stairs { .. } => {
                    transition_events.write(FloorTransition::AdvanceFloor);
                }
                _ => {
                    result_events.write(MoveResult::Blocked);
                }
            }
            continue;
        }

        occupancy.update_player_pos(new_pos);
        state.player_pos = new_pos;

        result_events.write(MoveResult::Moved { new_pos });
    }
}

fn overlaps_any_tile<F: bevy::ecs::query::QueryFilter>(
    tiles: &Query<&GlobalTransform, F>,
    pos: Vec2,
    player_size: EntitySize,
    tile_size: EntitySize,
) -> bool {
    tiles.iter().any(|tile_transform| {
        let tile_pos = tile_transform.translation().truncate();
        rects_overlap(pos, player_size, tile_pos, tile_size)
    })
}

#[instrument(level = "debug", skip_all, fields(pos = ?pos), ret)]
fn check_entity_collision(
    occupancy: &Occupancy,
    entity_query: &Query<&DungeonEntityMarker>,
    pos: Vec2,
    size: EntitySize,
) -> Option<(DungeonEntity, Entity, Vec2)> {
    if let Some(entity) = occupancy.entity_overlapping(pos, size) {
        if let Ok(marker) = entity_query.get(entity) {
            return Some((marker.entity_type, entity, marker.pos));
        }
    }
    None
}
