use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{EquipmentSlot, Inventory, ManagesEquipment};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

use super::render::{get_backpack_items, get_equipment_items};
use super::state::{BackpackGrid, EquipmentGrid};

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

/// System to handle Enter key equipping/unequipping items in the inventory modal.
pub fn handle_inventory_modal_select(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut inventory: ResMut<Inventory>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let eq_focused = equipment_grids
            .get_single()
            .map(|g| g.is_focused)
            .unwrap_or(false);

        if eq_focused {
            // UNEQUIP: find the slot for the selected equipment item
            let selected = equipment_grids.get_single().unwrap().selected_index;
            let equipped_slots: Vec<EquipmentSlot> = EquipmentSlot::all()
                .iter()
                .copied()
                .filter(|slot| inventory.get_equipped_item(*slot).is_some())
                .collect();

            if let Some(&slot) = equipped_slots.get(selected) {
                let _ = inventory.unequip_item(slot);
            }
        } else {
            // EQUIP: get the backpack item and equip it
            let selected = backpack_grids.get_single().unwrap().selected_index;
            let backpack_items = get_backpack_items(&inventory);

            if let Some(inv_item) = backpack_items.get(selected) {
                if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                    let uuid = inv_item.uuid();
                    inventory.equip_from_inventory(uuid, slot);
                }
            }
        }

        // Refresh both grids with updated inventory data
        refresh_grids(&inventory, &mut equipment_grids, &mut backpack_grids);
    }
}

fn refresh_grids(
    inventory: &Inventory,
    equipment_grids: &mut Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    backpack_grids: &mut Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if let Ok(mut eq_grid) = equipment_grids.get_single_mut() {
        eq_grid.items = get_equipment_items(inventory)
            .iter()
            .map(|inv_item| ItemGridEntry {
                sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            })
            .collect();
        if !eq_grid.items.is_empty() {
            eq_grid.selected_index = eq_grid.selected_index.min(eq_grid.items.len() - 1);
        } else {
            eq_grid.selected_index = 0;
        }
    }

    if let Ok(mut bp_grid) = backpack_grids.get_single_mut() {
        bp_grid.items = get_backpack_items(inventory)
            .iter()
            .map(|inv_item| ItemGridEntry {
                sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            })
            .collect();
        if !bp_grid.items.is_empty() {
            bp_grid.selected_index = bp_grid.selected_index.min(bp_grid.items.len() - 1);
        } else {
            bp_grid.selected_index = 0;
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
