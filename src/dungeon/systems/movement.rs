use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TilemapGridSize, TilemapSize};
use tracing::instrument;

use crate::dungeon::events::{FloorTransition, MoveResult, PlayerMoveIntent};
use crate::dungeon::tile_components::is_door;
use crate::dungeon::{DungeonEntity, DungeonEntityMarker, DungeonState, GameLayer};
use crate::input::NavigationDirection;

#[instrument(level = "debug", skip_all, fields(player_pos = ?state.player_pos, player_size = ?state.player_size))]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut state: ResMut<DungeonState>,
    spatial_query: SpatialQuery,
    entity_query: Query<&DungeonEntityMarker>,
    door_query: Query<(), With<is_door>>,
    tilemap_query: Query<(&TilemapGridSize, &GlobalTransform), With<TilemapSize>>,
) {
    let Ok((grid_size, map_transform)) = tilemap_query.single() else {
        return;
    };

    let scale = map_transform.to_scale_rotation_translation().0.x;
    let tile_world_size = grid_size.x * scale;
    let player_collider_size = state.player_size.width * scale * 0.9;

    for event in events.read() {
        let delta: Vec2 = match event.direction {
            NavigationDirection::Up => Vec2::new(0.0, tile_world_size),
            NavigationDirection::Down => Vec2::new(0.0, -tile_world_size),
            NavigationDirection::Left => Vec2::new(-tile_world_size, 0.0),
            NavigationDirection::Right => Vec2::new(tile_world_size, 0.0),
        };

        let new_pos = state.player_pos + delta;

        let filter = SpatialQueryFilter::from_mask([
            GameLayer::Tile,
            GameLayer::Mob,
            GameLayer::StaticEntity,
            GameLayer::Trigger,
        ]);

        let intersections = spatial_query.shape_intersections(
            &Collider::rectangle(player_collider_size, player_collider_size),
            new_pos,
            0.0,
            &filter,
        );

        if let Some(result) = process_collisions(&intersections, &entity_query, &door_query) {
            match result {
                CollisionResult::Blocked => {
                    result_events.write(MoveResult::Blocked);
                }
                CollisionResult::Combat { mob_id, entity, pos } => {
                    result_events.write(MoveResult::TriggeredCombat { mob_id, entity, pos });
                }
                CollisionResult::Door => {
                    transition_events.write(FloorTransition::EnterDoor);
                }
                CollisionResult::Stairs => {
                    transition_events.write(FloorTransition::AdvanceFloor);
                }
            }
            continue;
        }

        state.player_pos = new_pos;
        result_events.write(MoveResult::Moved { new_pos });
    }
}

enum CollisionResult {
    Blocked,
    Combat {
        mob_id: crate::mob::MobId,
        entity: Entity,
        pos: Vec2,
    },
    Door,
    Stairs,
}

fn process_collisions(
    intersections: &[Entity],
    entity_query: &Query<&DungeonEntityMarker>,
    door_query: &Query<(), With<is_door>>,
) -> Option<CollisionResult> {
    for &entity in intersections {
        if let Ok(marker) = entity_query.get(entity) {
            return Some(match marker.entity_type {
                DungeonEntity::Mob { mob_id, .. } => CollisionResult::Combat {
                    mob_id,
                    entity,
                    pos: marker.pos,
                },
                DungeonEntity::Door { .. } => CollisionResult::Door,
                DungeonEntity::Stairs { .. } => CollisionResult::Stairs,
                _ => CollisionResult::Blocked,
            });
        }

        if door_query.get(entity).is_ok() {
            return Some(CollisionResult::Door);
        }

        return Some(CollisionResult::Blocked);
    }

    None
}
