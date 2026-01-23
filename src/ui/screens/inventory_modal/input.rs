use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::ItemGrid;
use crate::ui::ModalCommands;

use super::state::InventoryModal;

const GRID_SIZE: usize = 4;

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

/// System to handle arrow key navigation within the inventory grid.
pub fn handle_inventory_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut item_grids: Query<&mut ItemGrid>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    let Ok(mut item_grid) = item_grids.get_single_mut() else {
        return;
    };

    let item_count = item_grid.items.len();
    if item_count == 0 {
        return;
    }

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            let current = item_grid.selected_index;
            let row = current / GRID_SIZE;
            let col = current % GRID_SIZE;

            let new_index = match direction {
                NavigationDirection::Left if col > 0 => current - 1,
                NavigationDirection::Right if col < GRID_SIZE - 1 => current + 1,
                NavigationDirection::Up if row > 0 => current - GRID_SIZE,
                NavigationDirection::Down if row < GRID_SIZE - 1 => current + GRID_SIZE,
                _ => current,
            };

            if new_index < item_count {
                item_grid.selected_index = new_index;
            }
        }
    }
}
