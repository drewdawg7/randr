use bevy::prelude::*;
use tracing::instrument;

use crate::crafting_station::ForgeCraftingState;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::item::{ItemId, ItemType};
use crate::item::enums::MaterialType;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::widgets::ItemGrid;

use crate::ui::widgets::ItemGridEntry;
use super::state::{ActiveForgeEntity, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex};

pub fn handle_forge_modal_navigation(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut modal_state: Option<ResMut<ForgeModalState>>,
    mut player_grids: Query<&mut ItemGrid, With<ForgePlayerGrid>>,
) {
    let Some(focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if focus_state.is_focused(FocusPanel::ForgeCraftingSlots) {
                if let Some(ref mut modal_state) = modal_state {
                    match direction {
                        NavigationDirection::Left => {
                            modal_state.selected_slot = modal_state.selected_slot.prev();
                        }
                        NavigationDirection::Right => {
                            modal_state.selected_slot = modal_state.selected_slot.next();
                        }
                        _ => {}
                    }
                }
            } else if focus_state.is_focused(FocusPanel::ForgeInventory) {
                if let Ok(mut grid) = player_grids.single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

#[instrument(level = "debug", skip_all, fields(
    has_focus = focus_state.is_some(),
    has_modal_state = modal_state.is_some(),
    has_active_forge = active_forge.is_some(),
    active_forge_entity = ?active_forge.as_ref().map(|f| f.0)
))]
pub fn handle_forge_modal_select(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    mut inventory: ResMut<Inventory>,
    mut forge_state_query: Query<&mut ForgeCraftingState>,
    mut player_grids: Query<&mut ItemGrid, With<ForgePlayerGrid>>,
) {
    let Some(focus_state) = focus_state else { return };

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

        if focus_state.is_focused(FocusPanel::ForgeCraftingSlots) {
            match modal_state.selected_slot {
                ForgeSlotIndex::Coal => {
                    if let Some((item_id, quantity)) = forge_state.coal_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
                ForgeSlotIndex::Ore => {
                    if let Some((item_id, quantity)) = forge_state.ore_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
                ForgeSlotIndex::Product => {
                    if let Some((item_id, quantity)) = forge_state.product_slot.take() {
                        add_items_to_inventory(&mut inventory, item_id, quantity);
                        transfer_occurred = true;
                    }
                }
            }
        } else {
            let selected = player_grids
                .single()
                .map(|g| g.selected_index)
                .unwrap_or(0);

            let inv_items = inventory.get_inventory_items();
            if let Some(inv_item) = inv_items.get(selected) {
                let item_id = inv_item.item.item_id;
                let quantity = inv_item.quantity;

                if is_coal(item_id) {
                    if forge_state.coal_slot.is_none() {
                        forge_state.coal_slot = Some((item_id, quantity));
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    } else if forge_state.coal_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                        if let Some((_, existing_qty)) = forge_state.coal_slot.as_mut() {
                            *existing_qty += quantity;
                            inventory.decrease_item_quantity(item_id, quantity);
                            transfer_occurred = true;
                        }
                    }
                } else if is_ore(item_id) {
                    if forge_state.ore_slot.is_none() {
                        forge_state.ore_slot = Some((item_id, quantity));
                        inventory.decrease_item_quantity(item_id, quantity);
                        transfer_occurred = true;
                    } else if forge_state.ore_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                        if let Some((_, existing_qty)) = forge_state.ore_slot.as_mut() {
                            *existing_qty += quantity;
                            inventory.decrease_item_quantity(item_id, quantity);
                            transfer_occurred = true;
                        }
                    }
                }
                // Other items cannot be placed in forge slots
            }
        }

        // Refresh inventory grid when transfer occurred
        // Forge slot display is handled reactively via Changed<ForgeCraftingState>
        if transfer_occurred {
            if let Ok(mut grid) = player_grids.single_mut() {
                grid.items = ItemGridEntry::from_inventory(&inventory);
                grid.clamp_selection();
            }
        }
    }
}

/// Check if an item is coal.
fn is_coal(item_id: ItemId) -> bool {
    matches!(item_id, ItemId::Coal)
}

/// Check if an item is a smeltable ore.
fn is_ore(item_id: ItemId) -> bool {
    matches!(
        item_id.spec().item_type,
        ItemType::Material(MaterialType::Ore)
    )
}

/// Add items to inventory (handles stacking).
fn add_items_to_inventory(inventory: &mut Inventory, item_id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = item_id.spawn();
        let _ = inventory.add_to_inv(item);
    }
}
