use avian2d::prelude::*;
use bevy::prelude::*;

use crate::dungeon::{MovementConfig, PlayerMoveIntent, TileWorldSize};
use crate::input::GameAction;
use crate::states::StateTransitionRequest;

use super::components::{DungeonPlayer, FacingDirection};

pub fn handle_dungeon_movement(
    mut action_reader: MessageReader<GameAction>,
    mut move_events: MessageWriter<PlayerMoveIntent>,
) {
    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            move_events.write(PlayerMoveIntent { direction: *direction });
        }
    }
}

pub fn handle_back_action(
    mut action_events: MessageReader<GameAction>,
    mut state_requests: MessageWriter<StateTransitionRequest>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            state_requests.write(StateTransitionRequest::Menu);
        }
    }
}

pub fn update_player_sprite_direction(
    mut query: Query<(&LinearVelocity, &mut Sprite, &mut FacingDirection), With<DungeonPlayer>>,
    movement: Res<MovementConfig>,
    tile_size: Res<TileWorldSize>,
) {
    let threshold = movement.flip_threshold(tile_size.0);

    for (velocity, mut sprite, mut facing) in &mut query {
        if velocity.x.abs() > velocity.y.abs() {
            if velocity.x < -threshold {
                sprite.flip_x = true;
                *facing = FacingDirection::Left;
            } else if velocity.x > threshold {
                sprite.flip_x = false;
                *facing = FacingDirection::Right;
            }
        } else {
            if velocity.y > threshold {
                *facing = FacingDirection::Up;
            } else if velocity.y < -threshold {
                *facing = FacingDirection::Down;
            }
        }
    }
}
