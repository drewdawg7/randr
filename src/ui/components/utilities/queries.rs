//! Game state query functions for UI.
//!
//! These functions access game state to collect player data for UI display.
//! They are isolated here to clearly mark the boundary between UI and game state.

use crate::inventory::{EquipmentSlot, InventoryItem, ManagesEquipment, ManagesItems};
use crate::system::game_state;

/// Collects all player items (equipped + inventory).
pub fn collect_player_items() -> Vec<InventoryItem> {
    let player = &game_state().player;
    let mut items = Vec::new();

    // Add equipped items first
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(inv_item.clone());
        }
    }

    // Add inventory items
    for inv_item in player.get_inventory_items() {
        items.push(inv_item.clone());
    }

    items
}

/// Collects player equipment items (equipped + inventory equipment only).
pub fn collect_player_equipment() -> Vec<InventoryItem> {
    let player = &game_state().player;
    let mut items = Vec::new();

    // Add equipped items
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(inv_item.clone());
        }
    }

    // Add inventory items (equipment only - materials can't be upgraded)
    for inv_item in player.get_inventory_items().iter() {
        if inv_item.item.item_type.is_equipment() {
            items.push(inv_item.clone());
        }
    }

    items
}
