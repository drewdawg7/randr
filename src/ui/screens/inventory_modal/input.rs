use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{FindsItems, Inventory, ManagesEquipment};

use super::render::spawn_inventory_modal;
use super::state::{InventoryModalRoot, InventorySelection, ItemInfo};
use super::utils::get_all_inventory_items;
use super::super::modal::{close_modal, toggle_modal, ActiveModal, ModalAction, ModalType};

/// System to handle opening/closing the inventory modal with 'i' key.
pub fn handle_inventory_modal_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    mut selection: ResMut<InventorySelection>,
    inventory: Res<Inventory>,
    existing_modal: Query<Entity, With<InventoryModalRoot>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::OpenInventory => {
                if let Some(ModalAction::Open) = toggle_modal(
                    &mut commands,
                    &mut active_modal,
                    &existing_modal,
                    ModalType::Inventory,
                ) {
                    selection.reset();
                    spawn_inventory_modal(&mut commands, &inventory, &mut selection);
                }
            }
            GameAction::CloseModal => {
                close_modal(
                    &mut commands,
                    &mut active_modal,
                    &existing_modal,
                    ModalType::Inventory,
                );
            }
            _ => {}
        }
    }
}

/// System to handle input when inventory modal is open.
pub fn handle_inventory_modal_input(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut selection: ResMut<InventorySelection>,
    mut inventory: ResMut<Inventory>,
) {
    if active_modal.modal != Some(ModalType::Inventory) {
        return;
    }

    for action in action_reader.read() {
        match action {
            GameAction::Navigate(dir) => match dir {
                NavigationDirection::Up => selection.up(),
                NavigationDirection::Down => selection.down(),
                _ => {}
            },
            GameAction::Select => {
                // Equip/unequip the selected item
                toggle_equip(&mut inventory, &selection);
            }
            _ => {}
        }
    }
}

/// Toggle equipping/unequipping of the selected item.
fn toggle_equip(inventory: &mut Inventory, selection: &InventorySelection) {
    let items = get_all_inventory_items(inventory);
    if let Some(item_info) = items.get(selection.index) {
        match item_info {
            ItemInfo::Equipped(slot, _) => {
                // Unequip
                let _ = inventory.unequip_item(*slot);
            }
            ItemInfo::Backpack(item_uuid, _) => {
                // Try to equip
                if let Some(inv_item) = inventory.find_item_by_uuid(*item_uuid) {
                    if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                        // Equip from inventory using the trait method
                        inventory.equip_from_inventory(*item_uuid, slot);
                    }
                }
            }
        }
    }
}
