//! Store-related game commands.
//!
//! Handles purchasing and selling items.

use uuid::Uuid;

use crate::inventory::HasInventory;
use crate::location::{sell_player_item, StoreError};
use crate::system::game_state;

use super::CommandResult;

/// Purchase an item from the store.
pub fn purchase_item(store_idx: usize) -> CommandResult {
    let gs = game_state();

    let store = &mut gs.town.store;
    let player = &mut gs.player;

    match store.purchase_item(player, store_idx) {
        Ok(item) => CommandResult::success(format!("Purchased {}!", item.name)),
        Err(e) => {
            let msg = match e {
                StoreError::OutOfStock => "Out of stock",
                StoreError::NotEnoughGold => "Not enough gold",
                StoreError::InventoryFull => "Inventory is full",
                StoreError::InvalidIndex => "Item not found",
            };
            CommandResult::error(msg)
        }
    }
}

/// Sell an item from player inventory.
pub fn sell_item(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Find the item by UUID
    let item = gs
        .player
        .find_item_by_uuid(item_uuid)
        .map(|inv| inv.item.clone());

    match item {
        Some(item) => {
            let item_name = item.name.to_string();
            sell_player_item(&mut gs.player, &item);
            CommandResult::success(format!("Sold {}!", item_name))
        }
        None => CommandResult::error("Item not found"),
    }
}
