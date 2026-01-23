use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::ModalCommands;

use super::state::InventoryModal;

/// System to handle closing the inventory modal with Escape.
pub fn handle_inventory_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            commands.close_modal::<InventoryModal>();
        }
    }
}
