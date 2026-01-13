use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::inventory::{Inventory, ManagesItems};
use crate::player::PlayerGold;

use super::Store;

/// Event to purchase an item from the store.
#[derive(Event, Debug, Clone)]
pub struct PurchaseEvent {
    /// Index into store inventory
    pub index: usize,
}

/// Event to sell an item from player's inventory.
#[derive(Event, Debug, Clone)]
pub struct SellEvent {
    /// Index into player's inventory
    pub inventory_index: usize,
}

/// Result event for store transactions.
#[derive(Event, Debug, Clone)]
pub enum TransactionResult {
    PurchaseSuccess { item_name: String, price: i32 },
    PurchaseFailedInsufficientGold { need: i32, have: i32 },
    PurchaseFailedInventoryFull,
    PurchaseFailedOutOfStock,
    SellSuccess { item_name: String, price: i32 },
}

/// Handle purchase events.
pub fn handle_purchase(
    mut events: EventReader<PurchaseEvent>,
    mut result_events: EventWriter<TransactionResult>,
    mut store: ResMut<Store>,
    mut gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
) {
    for event in events.read() {
        let Some(store_item) = store.inventory.get_mut(event.index) else {
            continue;
        };

        // Take item from store
        let Some(item) = store_item.take_item() else {
            result_events.send(TransactionResult::PurchaseFailedOutOfStock);
            continue;
        };

        let price = item.purchase_price();
        let item_name = item.name.clone();

        // Check gold
        if gold.0 < price {
            // Not enough gold - put item back
            store.inventory[event.index].items.push(item);
            result_events.send(TransactionResult::PurchaseFailedInsufficientGold {
                need: price,
                have: gold.0,
            });
            continue;
        }

        // Try to add to inventory
        if inventory.add_to_inv(item.clone()).is_err() {
            // Inventory full - put item back
            store.inventory[event.index].items.push(item);
            result_events.send(TransactionResult::PurchaseFailedInventoryFull);
            continue;
        }

        // Deduct gold
        gold.subtract(price);
        result_events.send(TransactionResult::PurchaseSuccess {
            item_name,
            price,
        });
    }
}

/// Handle sell events.
pub fn handle_sell(
    mut events: EventReader<SellEvent>,
    mut result_events: EventWriter<TransactionResult>,
    mut gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
) {
    for event in events.read() {
        let Some(inv_item) = inventory.items.get(event.inventory_index) else {
            continue;
        };

        // Can't sell locked items
        if inv_item.item.is_locked {
            continue;
        }

        let sell_price = inv_item.item.sell_price();
        let item_name = inv_item.item.name.clone();
        let item_id = inv_item.item.item_id;

        // Add gold and remove item
        gold.add(sell_price);
        inventory.decrease_item_quantity(item_id, 1);

        result_events.send(TransactionResult::SellSuccess {
            item_name,
            price: sell_price,
        });
    }
}
