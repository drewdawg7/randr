//! Inventory-related game commands.
//!
//! Handles equipping, unequipping, locking, and using items.

use uuid::Uuid;

use crate::inventory::{EquipmentSlot, FindsItems, ManagesEquipment};
use crate::item::consumable::use_consumable;
use crate::system::game_state;

use super::CommandResult;

/// Equip an item to a slot.
pub fn equip_item(item_uuid: Uuid, slot: EquipmentSlot) -> CommandResult {
    let gs = game_state();

    // Check if item exists before equipping
    if gs.player.find_item_by_uuid(item_uuid).is_some() {
        gs.player.equip_from_inventory(item_uuid, slot);
        CommandResult::ok()
    } else {
        CommandResult::error("Item not found")
    }
}

/// Unequip an item from a slot.
pub fn unequip_item(slot: EquipmentSlot) -> CommandResult {
    let gs = game_state();

    match gs.player.unequip_item(slot) {
        Ok(_) => CommandResult::ok(),
        Err(_) => CommandResult::error("Could not unequip item"),
    }
}

/// Toggle the lock status of an item.
pub fn toggle_lock(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    if let Some(inv_item) = gs.player.find_item_by_uuid_mut(item_uuid) {
        inv_item.item.toggle_lock();
        let status = if inv_item.item.is_locked {
            "locked"
        } else {
            "unlocked"
        };
        CommandResult::info(format!("Item {}", status))
    } else {
        CommandResult::error("Item not found")
    }
}

/// Use a consumable item.
pub fn consume_item(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Find the item first
    let inv_item = match gs.player.find_item_by_uuid(item_uuid) {
        Some(inv) => inv.clone(),
        None => return CommandResult::error("Item not found"),
    };

    // Use the consumable system
    match use_consumable(&mut gs.player, &inv_item) {
        Ok(result) => {
            // Remove the item from inventory
            gs.player.remove_item(item_uuid);
            CommandResult::success(result.describe())
        }
        Err(e) => {
            let msg = match e {
                crate::item::consumable::ConsumableError::NotConsumable => "Item is not consumable",
                crate::item::consumable::ConsumableError::NoEffectRegistered => "No effect defined",
                crate::item::consumable::ConsumableError::AlreadyAtFullHealth => "Already at full health",
            };
            CommandResult::error(msg)
        }
    }
}
