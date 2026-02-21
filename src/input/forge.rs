use bevy::prelude::*;
use tracing::instrument;

use crate::crafting_station::ForgeCraftingState;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::item::enums::MaterialType;
use crate::item::{Item, ItemId, ItemRegistry, ItemType};
use crate::player::PlayerMarker;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::forge_modal::{ActiveForgeEntity, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex};
use crate::ui::widgets::{ItemGrid, ItemGridEntry, ItemGridSelection};

pub fn navigate_forge_ui(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut modal_state: Option<ResMut<ForgeModalState>>,
    mut player_grids: Query<(&ItemGrid, &mut ItemGridSelection), With<ForgePlayerGrid>>,
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
                if let Ok((grid, mut selection)) = player_grids.single_mut() {
                    selection.navigate(*direction, grid.grid_size);
                }
            }
        }
    }
}

pub fn transfer_forge_items(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    mut forge_state_query: Query<&mut ForgeCraftingState>,
    mut player_grids: Query<(&mut ItemGrid, &mut ItemGridSelection), With<ForgePlayerGrid>>,
    registry: Res<ItemRegistry>,
) {
    let Some(focus_state) = focus_state else { return };
    let Some(modal_state) = modal_state else { return };
    let Some(active_forge) = active_forge else { return };
    let Ok(mut inventory) = player.single_mut() else { return };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let transfer_occurred = process_forge_select(
            &focus_state,
            &modal_state,
            active_forge.0,
            inventory.as_mut(),
            &mut forge_state_query,
            &player_grids,
            &registry,
        );

        if transfer_occurred {
            if let Ok((mut grid, mut selection)) = player_grids.single_mut() {
                grid.items = ItemGridEntry::from_inventory(&inventory);
                selection.clamp(grid.items.len());
            }
        }
    }
}

#[instrument(level = "debug", skip_all, fields(
    selected_slot = ?modal_state.selected_slot,
    has_forge_state = forge_state_query.contains(entity),
    is_crafting_slots_focused = focus_state.is_focused(FocusPanel::ForgeCraftingSlots),
    product_slot_has_items = forge_state_query.get(entity).map(|s| s.product_slot.is_some()).unwrap_or(false)
))]
fn process_forge_select(
    focus_state: &FocusState,
    modal_state: &ForgeModalState,
    entity: Entity,
    inventory: &mut Inventory,
    forge_state_query: &mut Query<&mut ForgeCraftingState>,
    player_grids: &Query<(&mut ItemGrid, &mut ItemGridSelection), With<ForgePlayerGrid>>,
    registry: &ItemRegistry,
) -> bool {
    let Ok(mut forge_state) = forge_state_query.get_mut(entity) else {
        return false;
    };

    if focus_state.is_focused(FocusPanel::ForgeCraftingSlots) {
        match modal_state.selected_slot {
            ForgeSlotIndex::Coal => {
                if let Some((item_id, quantity)) = forge_state.coal_slot.take() {
                    add_items_to_inventory(inventory, item_id, quantity, registry);
                    return true;
                }
            }
            ForgeSlotIndex::Ore => {
                if let Some((item_id, quantity)) = forge_state.ore_slot.take() {
                    add_items_to_inventory(inventory, item_id, quantity, registry);
                    return true;
                }
            }
            ForgeSlotIndex::Product => {
                if let Some((item_id, quantity)) = forge_state.product_slot.take() {
                    add_items_to_inventory(inventory, item_id, quantity, registry);
                    return true;
                }
            }
        }
    } else {
        let selected = player_grids
            .single()
            .map(|(_, s)| s.selected_index)
            .unwrap_or(0);

        let inv_items = inventory.get_inventory_items();
        if let Some(inv_item) = inv_items.get(selected) {
            let item_id = inv_item.item.item_id;
            let quantity = inv_item.quantity;

            if is_coal(&inv_item.item) {
                if forge_state.coal_slot.is_none() {
                    forge_state.coal_slot = Some((item_id, quantity));
                    inventory.decrease_item_quantity(item_id, quantity);
                    return true;
                } else if forge_state.coal_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                    if let Some((_, existing_qty)) = forge_state.coal_slot.as_mut() {
                        *existing_qty += quantity;
                        inventory.decrease_item_quantity(item_id, quantity);
                        return true;
                    }
                }
            } else if is_ore(&inv_item.item) {
                if forge_state.ore_slot.is_none() {
                    forge_state.ore_slot = Some((item_id, quantity));
                    inventory.decrease_item_quantity(item_id, quantity);
                    return true;
                } else if forge_state.ore_slot.as_ref().map(|(id, _)| *id) == Some(item_id) {
                    if let Some((_, existing_qty)) = forge_state.ore_slot.as_mut() {
                        *existing_qty += quantity;
                        inventory.decrease_item_quantity(item_id, quantity);
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn is_coal(item: &Item) -> bool {
    matches!(item.item_id, ItemId::Coal)
}

fn is_ore(item: &Item) -> bool {
    matches!(
        item.item_type,
        ItemType::Material(MaterialType::Ore)
    )
}

fn add_items_to_inventory(inventory: &mut Inventory, item_id: ItemId, quantity: u32, registry: &ItemRegistry) {
    for _ in 0..quantity {
        let item = registry.spawn(item_id);
        let _ = inventory.add_to_inv(item);
    }
}
