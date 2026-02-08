use bevy::prelude::*;

use crate::input::GameAction;
use crate::inventory::{EquipmentSlot, Inventory, ManagesEquipment};
use crate::player::PlayerMarker;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::widgets::ItemGrid;

use super::render::get_backpack_items;
use super::state::{BackpackGrid, EquipmentGrid};

/// System to handle arrow key navigation within the focused inventory grid.
/// Only runs when inventory modal is active (via run_if condition).
pub fn handle_inventory_modal_navigation(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    let Some(focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if focus_state.is_focused(FocusPanel::EquipmentGrid) {
                if let Ok(mut grid) = equipment_grids.single_mut() {
                    grid.navigate(*direction);
                }
            } else if focus_state.is_focused(FocusPanel::BackpackGrid) {
                if let Ok(mut grid) = backpack_grids.single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

pub fn handle_inventory_modal_select(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    equipment_grids: Query<&ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    backpack_grids: Query<&ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    let Some(focus_state) = focus_state else { return };
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let eq_focused = focus_state.is_focused(FocusPanel::EquipmentGrid);

        if eq_focused {
            // UNEQUIP: find the slot for the selected equipment item
            let Ok(equipment_grid) = equipment_grids.single() else {
                continue;
            };
            let selected = equipment_grid.selected_index;
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
            let Ok(backpack_grid) = backpack_grids.single() else {
                continue;
            };
            let selected = backpack_grid.selected_index;
            let backpack_items = get_backpack_items(&inventory);

            if let Some(inv_item) = backpack_items.get(selected) {
                if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                    let uuid = inv_item.uuid();
                    inventory.equip_from_inventory(uuid, slot);
                }
            }
        }

        // Grids will be refreshed reactively by sync_inventory_to_grids
    }
}
