use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::ItemGrid;
use crate::ui::ModalCommands;

use super::state::{BackpackGrid, EquipmentGrid, InventoryModal};

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

/// System to handle Tab key toggling focus between equipment and backpack grids.
pub fn handle_inventory_modal_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            let Ok(mut eq_grid) = equipment_grids.get_single_mut() else {
                return;
            };
            let eq_was_focused = eq_grid.is_focused;
            eq_grid.is_focused = !eq_was_focused;

            let Ok(mut bp_grid) = backpack_grids.get_single_mut() else {
                return;
            };
            bp_grid.is_focused = eq_was_focused;
        }
    }
}

/// System to handle arrow key navigation within the focused inventory grid.
pub fn handle_inventory_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if let Ok(mut grid) = equipment_grids.get_single_mut() {
                if grid.is_focused {
                    navigate_grid(&mut grid, *direction);
                    continue;
                }
            }
            if let Ok(mut grid) = backpack_grids.get_single_mut() {
                if grid.is_focused {
                    navigate_grid(&mut grid, *direction);
                }
            }
        }
    }
}

fn navigate_grid(grid: &mut ItemGrid, direction: NavigationDirection) {
    let gs = grid.grid_size;
    let item_count = grid.items.len();
    if item_count == 0 {
        return;
    }

    let current = grid.selected_index;
    let row = current / gs;
    let col = current % gs;

    let new_index = match direction {
        NavigationDirection::Left if col > 0 => current - 1,
        NavigationDirection::Right if col < gs - 1 => current + 1,
        NavigationDirection::Up if row > 0 => current - gs,
        NavigationDirection::Down if row < gs - 1 => current + gs,
        _ => current,
    };

    if new_index < item_count {
        grid.selected_index = new_index;
    }
}
