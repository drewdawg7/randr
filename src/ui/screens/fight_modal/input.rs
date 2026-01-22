//! Fight modal input handling.

use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::ui::SelectionState;

use super::super::modal::{close_modal, ActiveModal, ModalType};
use super::state::{FightModalButtonSelection, FightModalMob, FightModalRoot};

/// System to handle closing the fight modal.
pub fn handle_fight_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<FightModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal
            && close_modal(
                &mut commands,
                &mut active_modal,
                &modal_query,
                ModalType::FightModal,
            )
        {
            commands.remove_resource::<FightModalMob>();
            commands.remove_resource::<FightModalButtonSelection>();
        }
    }
}

/// System to handle left/right button navigation.
pub fn handle_fight_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    mut selection: ResMut<FightModalButtonSelection>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Left) => selection.up(),
            GameAction::Navigate(NavigationDirection::Right) => selection.down(),
            _ => {}
        }
    }
}
