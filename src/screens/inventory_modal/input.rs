use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{FindsItems, ManagesEquipment};

use super::render::spawn_inventory_modal;
use super::state::{InventoryModalRoot, InventorySelection, ItemInfo};
use super::utils::get_all_inventory_items;
use crate::screens::modal::{ActiveModal, ModalType};

/// System to handle opening/closing the inventory modal with 'i' key.
pub fn handle_inventory_modal_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    mut selection: ResMut<InventorySelection>,
    player: Res<Player>,
    existing_modal: Query<Entity, With<InventoryModalRoot>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::OpenInventory => {
                // Toggle: close if open, open if closed
                if let Ok(entity) = existing_modal.get_single() {
                    commands.entity(entity).despawn_recursive();
                    active_modal.modal = None;
                } else {
                    selection.reset();
                    spawn_inventory_modal(&mut commands, &player, &mut selection);
                    active_modal.modal = Some(ModalType::Inventory);
                }
            }
            GameAction::CloseModal => {
                // Close if this modal is open
                if active_modal.modal == Some(ModalType::Inventory) {
                    if let Ok(entity) = existing_modal.get_single() {
                        commands.entity(entity).despawn_recursive();
                        active_modal.modal = None;
                    }
                }
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
    mut player: ResMut<Player>,
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
                toggle_equip(&mut player, &selection);
            }
            _ => {}
        }
    }
}

/// Toggle equipping/unequipping of the selected item.
fn toggle_equip(player: &mut Player, selection: &InventorySelection) {
    let items = get_all_inventory_items(player);
    if let Some(item_info) = items.get(selection.index) {
        match item_info {
            ItemInfo::Equipped(slot, _) => {
                // Unequip
                let _ = player.unequip_item(*slot);
            }
            ItemInfo::Backpack(item_uuid, _) => {
                // Try to equip
                if let Some(inv_item) = player.find_item_by_uuid(*item_uuid) {
                    if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                        // Equip from inventory using the trait method
                        player.equip_from_inventory(*item_uuid, slot);
                    }
                }
            }
        }
    }
}
