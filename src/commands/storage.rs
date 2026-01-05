//! Storage-related game commands.
//!
//! Handles depositing and withdrawing items from storage.

use uuid::Uuid;

use crate::inventory::{FindsItems, HasInventory, ManagesItems};
use crate::system::game_state;

use super::CommandResult;

/// Deposit an item from player inventory to storage.
pub fn deposit_item(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Find the item in player inventory
    let item = gs.player.find_item_by_uuid(item_uuid).cloned();

    match item {
        Some(inv_item) => {
            // Check if item is locked
            if inv_item.item.is_locked {
                return CommandResult::error("Cannot store locked items");
            }

            // Check if item is equipped
            if inv_item.item.is_equipped {
                return CommandResult::error("Cannot store equipped items");
            }

            let item_name = inv_item.item.name.to_string();

            // Remove from player and add to storage
            if let Some(removed_item) = gs.player.remove_item(item_uuid) {
                if gs.storage_mut().add_to_inv(removed_item.item).is_err() {
                    return CommandResult::error("Storage is full");
                }
                CommandResult::success(format!("Stored {}!", item_name))
            } else {
                CommandResult::error("Item not found")
            }
        }
        None => CommandResult::error("Item not found"),
    }
}

/// Withdraw an item from storage to player inventory.
pub fn withdraw_item(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Find the item in storage
    let item = gs.storage().find_item_by_uuid(item_uuid).cloned();

    match item {
        Some(inv_item) => {
            let item_name = inv_item.item.name.to_string();

            // Check if player inventory has room
            if gs.player.get_inventory_items().len() >= gs.player.inventory().max_slots() {
                return CommandResult::error("Inventory is full");
            }

            // Remove from storage and add to player
            if let Some(removed_item) = gs.storage_mut().remove_item(item_uuid) {
                if gs.player.add_to_inv(removed_item.item).is_err() {
                    return CommandResult::error("Inventory is full");
                }
                CommandResult::success(format!("Withdrew {}!", item_name))
            } else {
                CommandResult::error("Item not found")
            }
        }
        None => CommandResult::error("Item not found"),
    }
}
