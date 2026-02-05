use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::inventory::{Inventory, ManagesItems};
use crate::player::{PlayerGold, PlayerMarker};

use super::Store;

/// Event to purchase an item from the store.
#[derive(Message, Debug, Clone)]
pub struct PurchaseEvent {
    /// Index into store inventory
    pub index: usize,
}

/// Event to sell an item from player's inventory.
#[derive(Message, Debug, Clone)]
pub struct SellEvent {
    /// Index into player's inventory
    pub inventory_index: usize,
}

/// Result event for store transactions.
#[derive(Message, Debug, Clone)]
pub enum TransactionResult {
    PurchaseSuccess { item_name: String, price: i32 },
    PurchaseFailedInsufficientGold { need: i32, have: i32 },
    PurchaseFailedInventoryFull,
    PurchaseFailedOutOfStock,
    SellSuccess { item_name: String, price: i32 },
}

pub fn handle_purchase(
    mut events: MessageReader<PurchaseEvent>,
    mut result_events: MessageWriter<TransactionResult>,
    mut store: ResMut<Store>,
    mut player: Query<(&mut PlayerGold, &mut Inventory), With<PlayerMarker>>,
) {
    let Ok((mut gold, mut inventory)) = player.single_mut() else {
        return;
    };

    for event in events.read() {
        let Some(store_item) = store.inventory.get_mut(event.index) else {
            continue;
        };

        let Some(item) = store_item.take_item() else {
            result_events.write(TransactionResult::PurchaseFailedOutOfStock);
            continue;
        };

        let price = item.purchase_price();
        let item_name = item.name.clone();

        if gold.0 < price {
            store.inventory[event.index].items.push(item);
            result_events.write(TransactionResult::PurchaseFailedInsufficientGold {
                need: price,
                have: gold.0,
            });
            continue;
        }

        if inventory.add_to_inv(item.clone()).is_err() {
            store.inventory[event.index].items.push(item);
            result_events.write(TransactionResult::PurchaseFailedInventoryFull);
            continue;
        }

        gold.subtract(price);
        result_events.write(TransactionResult::PurchaseSuccess {
            item_name,
            price,
        });
    }
}

pub fn handle_sell(
    mut events: MessageReader<SellEvent>,
    mut result_events: MessageWriter<TransactionResult>,
    mut player: Query<(&mut PlayerGold, &mut Inventory), With<PlayerMarker>>,
) {
    let Ok((mut gold, mut inventory)) = player.single_mut() else {
        return;
    };

    for event in events.read() {
        let Some(inv_item) = inventory.items.get(event.inventory_index) else {
            continue;
        };

        if inv_item.item.is_locked {
            continue;
        }

        let sell_price = inv_item.item.sell_price();
        let item_name = inv_item.item.name.clone();
        let item_id = inv_item.item.item_id;

        gold.add(sell_price);
        inventory.decrease_item_quantity(item_id, 1);

        result_events.write(TransactionResult::SellSuccess {
            item_name,
            price: sell_price,
        });
    }
}
