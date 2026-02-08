use bevy::prelude::*;

use crate::dungeon::PlayerMoveIntent;
use crate::input::GameAction;
use crate::states::StateTransitionRequest;

pub fn emit_move_intent(
    mut action_reader: MessageReader<GameAction>,
    mut move_events: MessageWriter<PlayerMoveIntent>,
) {
    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            move_events.write(PlayerMoveIntent { direction: *direction });
        }
    }
}

pub fn request_menu_transition(
    mut action_events: MessageReader<GameAction>,
    mut state_requests: MessageWriter<StateTransitionRequest>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            state_requests.write(StateTransitionRequest::Menu);
        }
    }
}
