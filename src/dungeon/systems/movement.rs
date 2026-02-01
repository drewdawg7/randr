use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;
use tracing::instrument;

use crate::dungeon::events::{FloorTransition, MoveResult, PlayerMoveIntent};
use crate::dungeon::{
    occupied_cells, DungeonEntity, DungeonEntityMarker, DungeonState, GridOccupancy, GridSize,
    TileIndex,
};
use crate::input::NavigationDirection;

#[instrument(level = "debug", skip_all)]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut state: ResMut<DungeonState>,
    mut occupancy: ResMut<GridOccupancy>,
    entity_query: Query<&DungeonEntityMarker>,
    tile_index: Option<Res<TileIndex>>,
) {
    let Some(tile_index) = tile_index else {
        return;
    };

    for event in events.read() {
        let (dx, dy): (i32, i32) = match event.direction {
            NavigationDirection::Up => (0, -1),
            NavigationDirection::Down => (0, 1),
            NavigationDirection::Left => (-1, 0),
            NavigationDirection::Right => (1, 0),
        };

        let new_x = (state.player_pos.x as i32 + dx).max(0) as u32;
        let new_y = (state.player_pos.y as i32 + dy).max(0) as u32;
        let new_pos = TilePos::new(new_x, new_y);

        if tile_index.is_door(new_pos.x, new_pos.y) {
            transition_events.write(FloorTransition::EnterDoor);
            return;
        }

        if !all_cells_walkable(&tile_index, new_pos, state.player_size) {
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

        occupancy.unmark_blocked(state.player_pos, state.player_size);
        occupancy.mark_blocked(new_pos, state.player_size);
        state.player_pos = new_pos;

        result_events.write(MoveResult::Moved { new_pos });
    }
}

fn all_cells_walkable(tile_index: &TileIndex, pos: TilePos, size: GridSize) -> bool {
    occupied_cells(pos, size).all(|(x, y)| tile_index.is_walkable(x, y))
}

#[instrument(level = "debug", skip_all, fields(pos = ?pos), ret)]
fn check_entity_collision(
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
    pos: TilePos,
    size: GridSize,
) -> Option<(DungeonEntity, Entity, TilePos)> {
    for (x, y) in occupied_cells(pos, size) {
        if let Some(entity) = occupancy.entity_at(x, y) {
            if let Ok(marker) = entity_query.get(entity) {
                return Some((marker.entity_type, entity, marker.pos));
            }
        }
    }
    None
}
