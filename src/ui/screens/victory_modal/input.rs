//! Victory modal input handling.

use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::screens::modal::{close_modal, ActiveModal, ModalType};

use super::state::{VictoryModalData, VictoryModalRoot};

/// System to handle closing the victory modal on Enter or Escape.
pub fn handle_victory_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<VictoryModalRoot>>,
) {
    for action in action_reader.read() {
        if matches!(action, GameAction::Select | GameAction::CloseModal) {
            if close_modal(
                &mut commands,
                &mut active_modal,
                &modal_query,
                ModalType::VictoryModal,
            ) {
                commands.remove_resource::<VictoryModalData>();
            }
        }
    }
}
