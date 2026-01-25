use bevy::prelude::*;

use crate::crafting_station::ForgeCraftingState;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::item::ItemId;
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::ItemGrid;

use super::render::get_player_inventory_entries;
use super::state::{
    ActiveForgeEntity, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex, ForgeSlotRefresh,
};

/// Handle Tab key toggling focus between crafting slots and player inventory.
pub fn handle_forge_modal_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut modal_state: Option<ResMut<ForgeModalState>>,
    mut player_grids: Query<&mut ItemGrid, With<ForgePlayerGrid>>,
) {
    if active_modal.modal != Some(ModalType::ForgeModal) {
        return;
    }

    let Some(ref mut modal_state) = modal_state else {
        return;
    };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            // Toggle focus
            modal_state.crafting_focused = !modal_state.crafting_focused;

            // Update ItemGrid focus state
            if let Ok(mut grid) = player_grids.get_single_mut() {
                grid.is_focused = !modal_state.crafting_focused;
            }
        }
    }
}

/// Handle arrow key navigation within the forge modal.
pub fn handle_forge_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut modal_state: Option<ResMut<ForgeModalState>>,
    mut player_grids: Query<&mut ItemGrid, With<ForgePlayerGrid>>,
) {
    if active_modal.modal != Some(ModalType::ForgeModal) {
        return;
    }

    let Some(ref mut modal_state) = modal_state else {
        return;
    };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if modal_state.crafting_focused {
                // Navigate within crafting slots (horizontal only)
                match direction {
                    NavigationDirection::Left => {
                        modal_state.selected_slot = modal_state.selected_slot.prev();
                    }
                    NavigationDirection::Right => {
                        modal_state.selected_slot = modal_state.selected_slot.next();
                    }
                    _ => {}
                }
            } else {
                // Navigate within player inventory grid
                if let Ok(mut grid) = player_grids.get_single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

/// Handle Enter key for moving items between inventory and forge slots.
pub fn handle_forge_modal_select(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    modal_state: Option<Res<ForgeModalState>>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    mut inventory: ResMut<Inventory>,
    mut forge_state_query: Query<&mut ForgeCraftingState>,
    mut player_grids: Query<&mut ItemGrid, With<ForgePlayerGrid>>,
) {
    if active_modal.modal != Some(ModalType::ForgeModal) {
        return;
    }

    let Some(modal_state) = modal_state else {
        return;
    };

    let Some(active_forge) = active_forge else {
        return;
    };

    let Ok(mut forge_state) = forge_state_query.get_mut(active_forge.0) else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let mut transfer_occurred = false;

        if modal_state.crafting_focused {
            // Crafting slots focused - take items back or collect product
            match modal_state.selected_slot {
                ForgeSlotIndex::Coal => {
                    // Return coal to inventory
                    if let Some((item_id, quantity)) = forge_state.coal_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
                ForgeSlotIndex::Ore => {
                    // Return ore to inventory
                    if let Some((item_id, quantity)) = forge_state.ore_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
                ForgeSlotIndex::Product => {
                    // Collect ingots to inventory
                    if let Some((item_id, quantity)) = forge_state.product_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
            }
        } else {
            // Inventory focused - move items to forge slots
            let selected = player_grids
                .get_single()
                .map(|g| g.selected_index)
                .unwrap_or(0);

            let inv_items = inventory.get_inventory_items();
            if let Some(inv_item) = inv_items.get(selected) {
                let item_id = inv_item.item.item_id;
                let quantity = inv_item.quantity;

                // Determine which slot this item can go to
                if is_coal(item_id) {
                    // Move to coal slot
                    if forge_state.coal_slot.is_none() {
                        // Move all to empty slot
                        forge_state.coal_slot = Some((item_id, quantity));
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    } else if forge_state.coal_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                        // Stack onto existing
                        let (_, existing_qty) = forge_state.coal_slot.as_mut().unwrap();
                        *existing_qty += quantity;
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    }
                } else if is_ore(item_id) {
                    // Move to ore slot
                    if forge_state.ore_slot.is_none() {
                        forge_state.ore_slot = Some((item_id, quantity));
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    } else if forge_state.ore_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                        let (_, existing_qty) = forge_state.ore_slot.as_mut().unwrap();
                        *existing_qty += quantity;
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    }
                }
                // Other items cannot be placed in forge slots
            }
        }

        if transfer_occurred {
            // Refresh inventory grid
            if let Ok(mut grid) = player_grids.get_single_mut() {
                grid.items = get_player_inventory_entries(&inventory);
                if !grid.items.is_empty() {
                    grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
                } else {
                    grid.selected_index = 0;
                }
            }

            // Trigger slot refresh
            commands.insert_resource(ForgeSlotRefresh);
        }
    }
}

/// Check if an item is coal.
fn is_coal(item_id: ItemId) -> bool {
    matches!(item_id, ItemId::Coal)
}

/// Check if an item is an ore (copper or tin).
fn is_ore(item_id: ItemId) -> bool {
    matches!(item_id, ItemId::CopperOre | ItemId::TinOre)
}

/// Add items to inventory (handles stacking).
fn add_items_to_inventory(inventory: &mut Inventory, item_id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = item_id.spawn();
        let _ = inventory.add_to_inv(item);
    }
}
