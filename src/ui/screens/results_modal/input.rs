//! Results modal input handling.

use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::modal_registry::ModalCommands;
use crate::ui::screens::modal::{ActiveModal, ModalType};

use super::state::ResultsModal;

/// System to handle closing the results modal on Enter or Escape.
pub fn handle_results_modal_close(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    // Always consume events to advance reader cursor (prevents stale event processing)
    let should_close = action_reader
        .read()
        .any(|a| matches!(a, GameAction::Select | GameAction::CloseModal));

    if active_modal.modal != Some(ModalType::ResultsModal) {
        return;
    }

    if should_close {
        commands.close_modal::<ResultsModal>();
    }
}
