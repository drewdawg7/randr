use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::events::{FloorTransition, MoveResult, PlayerMoveIntent};
use crate::dungeon::{
    DungeonEntity, DungeonEntityMarker, DungeonState, Occupancy, TileIndex, TileWorldSize,
};
use crate::input::NavigationDirection;

#[instrument(level = "debug", skip_all, fields(player_pos = ?state.player_pos))]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut state: ResMut<DungeonState>,
    occupancy: Option<ResMut<Occupancy>>,
    entity_query: Query<&DungeonEntityMarker>,
    tile_index: Option<Res<TileIndex>>,
    tile_size: Option<Res<TileWorldSize>>,
) {
    let Some(tile_index) = tile_index else {
        return;
    };
    let Some(mut occupancy) = occupancy else {
        return;
    };

    let step = tile_size.map(|t| t.0).unwrap_or(32.0);

    for event in events.read() {
        let delta: Vec2 = match event.direction {
            NavigationDirection::Up => Vec2::new(0.0, step),
            NavigationDirection::Down => Vec2::new(0.0, -step),
            NavigationDirection::Left => Vec2::new(-step, 0.0),
            NavigationDirection::Right => Vec2::new(step, 0.0),
        };

        let new_pos = state.player_pos + delta;

        let tile_x = (new_pos.x / step).floor() as u32;
        let tile_y = (new_pos.y / step).floor() as u32;

        if tile_index.is_door(tile_x, tile_y) {
            transition_events.write(FloorTransition::EnterDoor);
            return;
        }

        if !tile_index.is_walkable(tile_x, tile_y) {
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

#[instrument(level = "debug", skip_all, fields(pos = ?pos), ret)]
fn check_entity_collision(
    occupancy: &Occupancy,
    entity_query: &Query<&DungeonEntityMarker>,
    pos: Vec2,
    size: crate::dungeon::EntitySize,
) -> Option<(DungeonEntity, Entity, Vec2)> {
    if let Some(entity) = occupancy.entity_overlapping(pos, size) {
        if let Ok(marker) = entity_query.get(entity) {
            return Some((marker.entity_type, entity, marker.pos));
        }
    }
    None
}
