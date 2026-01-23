//! Results modal input handling.

use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::screens::modal::{close_modal, ActiveModal, ModalType};

use super::state::{ResultsModalData, ResultsModalRoot};

/// System to handle closing the results modal on Enter or Escape.
pub fn handle_results_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<ResultsModalRoot>>,
) {
    for action in action_reader.read() {
        if matches!(action, GameAction::Select | GameAction::CloseModal) {
            if close_modal(
                &mut commands,
                &mut active_modal,
                &modal_query,
                ModalType::ResultsModal,
            ) {
                commands.remove_resource::<ResultsModalData>();
            }
        }
    }
}
