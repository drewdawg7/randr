use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::modal_registry::ModalCommands;
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::screens::results_modal::ResultsModal;

pub fn close_results_modal(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
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
