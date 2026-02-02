use avian2d::prelude::*;
use bevy::prelude::*;
use tracing::{debug, instrument};

use crate::dungeon::events::{FloorTransition, MoveResult, PlayerMoveIntent};
use crate::dungeon::tile_components::is_door;
use crate::dungeon::{DoorEntity, DungeonEntityMarker, MobEntity, StairsEntity};
use crate::input::NavigationDirection;
use crate::ui::screens::DungeonPlayer;

const PLAYER_SPEED: f32 = 200.0;

#[instrument(level = "debug", skip_all, fields(event_count = events.len()))]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut player_query: Query<(&mut LinearVelocity, &Transform, &Collider), With<DungeonPlayer>>,
) {
    for event in events.read() {
        let Ok((mut velocity, transform, collider)) = player_query.single_mut() else {
            continue;
        };

        let direction: Vec2 = match event.direction {
            NavigationDirection::Up => Vec2::Y,
            NavigationDirection::Down => Vec2::NEG_Y,
            NavigationDirection::Left => Vec2::NEG_X,
            NavigationDirection::Right => Vec2::X,
        };

        let pos = transform.translation;
        let shape = collider.shape_scaled();
        let aabb = shape.compute_local_aabb();
        debug!(
            player_x = pos.x,
            player_y = pos.y,
            collider_min_x = pos.x + aabb.mins.x,
            collider_min_y = pos.y + aabb.mins.y,
            collider_max_x = pos.x + aabb.maxs.x,
            collider_max_y = pos.y + aabb.maxs.y,
            direction = ?event.direction,
            "player moving"
        );

        velocity.0 = direction * PLAYER_SPEED;
    }
}

pub fn stop_player_when_idle(
    mut player_query: Query<&mut LinearVelocity, With<DungeonPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let movement_keys = [
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ];

    if !movement_keys.iter().any(|k| input.pressed(*k)) {
        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.0 = Vec2::ZERO;
        }
    }
}

#[instrument(level = "debug", skip_all, fields(collision_count = collision_events.len()))]
pub fn handle_player_collisions(
    mut collision_events: MessageReader<CollisionStart>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    player_query: Query<Entity, With<DungeonPlayer>>,
    marker_query: Query<&DungeonEntityMarker>,
    mob_query: Query<&MobEntity>,
    stairs_query: Query<(), With<StairsEntity>>,
    door_entity_query: Query<(), With<DoorEntity>>,
    door_tile_query: Query<(), With<is_door>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for event in collision_events.read() {
        let other = if event.collider1 == player_entity {
            event.collider2
        } else if event.collider2 == player_entity {
            event.collider1
        } else {
            continue;
        };

        if let Ok(mob) = mob_query.get(other) {
            if let Ok(marker) = marker_query.get(other) {
                result_events.write(MoveResult::TriggeredCombat {
                    mob_id: mob.mob_id,
                    entity: other,
                    pos: marker.pos,
                });
            }
            continue;
        }

        if door_entity_query.get(other).is_ok() {
            transition_events.write(FloorTransition::EnterDoor);
            continue;
        }

        if stairs_query.get(other).is_ok() {
            transition_events.write(FloorTransition::AdvanceFloor);
            continue;
        }

        if door_tile_query.get(other).is_ok() {
            transition_events.write(FloorTransition::EnterDoor);
        }
    }
}
