use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::game::{ItemDeposited, ItemWithdrawn, Player, Storage};
use crate::inventory::{FindsItems, ManagesItems};
use crate::item::ItemId;

/// Event sent when player attempts to purchase an item from the store.
#[derive(Event, Debug, Clone)]
pub struct StorePurchaseEvent {
    /// The item to purchase
    pub item_id: ItemId,
    /// The price to pay
    pub price: i32,
    /// Display name for the item
    pub item_name: String,
}

/// Event sent when player attempts to sell an item.
#[derive(Event, Debug, Clone)]
pub struct StoreSellEvent {
    /// Index into player's inventory
    pub inventory_index: usize,
}

/// Event sent when player attempts to withdraw an item from storage.
#[derive(Event, Debug, Clone)]
pub struct StorageWithdrawEvent {
    /// Index into storage inventory
    pub storage_index: usize,
}

/// Event sent when player attempts to deposit an item into storage.
#[derive(Event, Debug, Clone)]
pub struct StorageDepositEvent {
    /// Index into player's inventory
    pub inventory_index: usize,
}

/// Result event for store and storage operations.
#[derive(Event, Debug, Clone)]
pub enum StoreTransactionResult {
    PurchaseSuccess { item_name: String, price: i32 },
    PurchaseFailedInsufficientGold { need: i32, have: i32 },
    PurchaseFailedInventoryFull,
    SellSuccess { item_name: String, price: i32 },
    WithdrawSuccess { item_name: String },
    WithdrawFailedInventoryFull,
    DepositSuccess { item_name: String },
    DepositFailed { reason: String },
}

/// Plugin for store transaction events and systems.
pub struct StoreTransactionsPlugin;

impl Plugin for StoreTransactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StorePurchaseEvent>()
            .add_event::<StoreSellEvent>()
            .add_event::<StorageWithdrawEvent>()
            .add_event::<StorageDepositEvent>()
            .add_event::<StoreTransactionResult>()
            .add_systems(
                Update,
                (
                    handle_store_purchase,
                    handle_store_sell,
                    handle_storage_withdraw,
                    handle_storage_deposit,
                ),
            );
    }
}

/// Handle purchase events from the store.
fn handle_store_purchase(
    mut purchase_events: EventReader<StorePurchaseEvent>,
    mut result_events: EventWriter<StoreTransactionResult>,
    mut player: ResMut<Player>,
) {
    for event in purchase_events.read() {
        // Check gold
        if player.gold < event.price {
            result_events.send(StoreTransactionResult::PurchaseFailedInsufficientGold {
                need: event.price,
                have: player.gold,
            });
            info!(
                "Not enough gold! Need {} but have {}",
                event.price, player.gold
            );
            continue;
        }

        // Spawn and add to inventory
        let new_item = event.item_id.spawn();

        match player.add_to_inv(new_item) {
            Ok(_) => {
                player.gold -= event.price;
                result_events.send(StoreTransactionResult::PurchaseSuccess {
                    item_name: event.item_name.clone(),
                    price: event.price,
                });
                info!("Purchased {} for {} gold", event.item_name, event.price);
            }
            Err(_) => {
                result_events.send(StoreTransactionResult::PurchaseFailedInventoryFull);
                info!("Inventory full!");
            }
        }
    }
}

/// Handle sell events.
fn handle_store_sell(
    mut sell_events: EventReader<StoreSellEvent>,
    mut result_events: EventWriter<StoreTransactionResult>,
    mut player: ResMut<Player>,
) {
    for event in sell_events.read() {
        let Some(inv_item) = player.inventory.items.get(event.inventory_index).cloned() else {
            continue;
        };

        let sell_price = inv_item.item.sell_price();
        let item_name = inv_item.item.name.clone();

        // Add gold and remove item
        player.gold += sell_price;
        player.decrease_item_quantity(&inv_item, 1);

        result_events.send(StoreTransactionResult::SellSuccess {
            item_name: item_name.clone(),
            price: sell_price,
        });
        info!("Sold {} for {} gold", item_name, sell_price);
    }
}

/// Handle storage withdraw events.
fn handle_storage_withdraw(
    mut withdraw_events: EventReader<StorageWithdrawEvent>,
    mut result_events: EventWriter<StoreTransactionResult>,
    mut withdrawn_events: EventWriter<ItemWithdrawn>,
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
) {
    for event in withdraw_events.read() {
        let Some(inv_item) = storage.inventory.items.get(event.storage_index).cloned() else {
            continue;
        };

        let item = inv_item.item.clone();
        let item_name = item.name.clone();
        let item_uuid = inv_item.uuid();

        // Try to add to player inventory
        match player.add_to_inv(item) {
            Ok(_) => {
                // Remove from storage
                storage.remove_item(item_uuid);
                result_events.send(StoreTransactionResult::WithdrawSuccess {
                    item_name: item_name.clone(),
                });
                withdrawn_events.send(ItemWithdrawn {
                    item_name: item_name.clone(),
                });
                info!("Withdrew {} from storage", item_name);
            }
            Err(_) => {
                result_events.send(StoreTransactionResult::WithdrawFailedInventoryFull);
                info!("Inventory is full! Cannot withdraw item.");
            }
        }
    }
}

/// Handle storage deposit events.
fn handle_storage_deposit(
    mut deposit_events: EventReader<StorageDepositEvent>,
    mut result_events: EventWriter<StoreTransactionResult>,
    mut deposited_events: EventWriter<ItemDeposited>,
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
) {
    for event in deposit_events.read() {
        let Some(inv_item) = player.inventory.items.get(event.inventory_index).cloned() else {
            continue;
        };

        let item = inv_item.item.clone();
        let item_name = item.name.clone();
        let item_uuid = inv_item.uuid();

        // Add to storage (storage has unlimited capacity)
        match storage.add_to_inv(item) {
            Ok(_) => {
                // Remove from player inventory
                player.remove_item(item_uuid);
                result_events.send(StoreTransactionResult::DepositSuccess {
                    item_name: item_name.clone(),
                });
                deposited_events.send(ItemDeposited {
                    item_name: item_name.clone(),
                });
                info!("Deposited {} into storage", item_name);
            }
            Err(e) => {
                result_events.send(StoreTransactionResult::DepositFailed {
                    reason: format!("{:?}", e),
                });
                info!("Failed to deposit item: {:?}", e);
            }
        }
    }
}
