use bevy::prelude::*;

use crate::game::{ItemDeposited, ItemWithdrawn, Storage};
use crate::inventory::{FindsItems, HasInventory, Inventory, ManagesItems};

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

/// Result event for storage operations.
#[derive(Event, Debug, Clone)]
pub enum StorageTransactionResult {
    WithdrawSuccess { item_name: String },
    WithdrawFailedInventoryFull,
    DepositSuccess { item_name: String },
    DepositFailed { reason: String },
}

/// Plugin for storage transaction events and systems.
pub struct StorageTransactionsPlugin;

impl Plugin for StorageTransactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StorageWithdrawEvent>()
            .add_event::<StorageDepositEvent>()
            .add_event::<StorageTransactionResult>()
            .add_systems(
                Update,
                (
                    handle_storage_withdraw.run_if(on_event::<StorageWithdrawEvent>),
                    handle_storage_deposit.run_if(on_event::<StorageDepositEvent>),
                ),
            );
    }
}

/// Handle storage withdraw events.
fn handle_storage_withdraw(
    mut withdraw_events: EventReader<StorageWithdrawEvent>,
    mut result_events: EventWriter<StorageTransactionResult>,
    mut withdrawn_events: EventWriter<ItemWithdrawn>,
    mut inventory: ResMut<Inventory>,
    mut storage: ResMut<Storage>,
) {
    for event in withdraw_events.read() {
        // Get item info from reference first
        let Some(inv_item) = storage.inventory.items.get(event.storage_index) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();
        let item_uuid = inv_item.uuid();

        // Check if inventory has room before removing from storage
        if inventory.items.len() >= inventory.max_slots() {
            result_events.send(StorageTransactionResult::WithdrawFailedInventoryFull);
            info!("Inventory is full! Cannot withdraw item.");
            continue;
        }

        // Remove from storage to get ownership
        let Some(inv_item) = storage.remove_item(item_uuid) else {
            continue;
        };

        // Add to player inventory (should succeed since we checked capacity)
        if inventory.add_to_inv(inv_item.item).is_ok() {
            result_events.send(StorageTransactionResult::WithdrawSuccess {
                item_name: item_name.clone(),
            });
            withdrawn_events.send(ItemWithdrawn {
                item_name: item_name.clone(),
            });
            info!("Withdrew {} from storage", item_name);
        }
    }
}

/// Handle storage deposit events.
fn handle_storage_deposit(
    mut deposit_events: EventReader<StorageDepositEvent>,
    mut result_events: EventWriter<StorageTransactionResult>,
    mut deposited_events: EventWriter<ItemDeposited>,
    mut inventory: ResMut<Inventory>,
    mut storage: ResMut<Storage>,
) {
    for event in deposit_events.read() {
        // Get item info from reference first
        let Some(inv_item) = inventory.items.get(event.inventory_index) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();
        let item_uuid = inv_item.uuid();

        // Check if storage has room before removing from inventory
        if storage.inventory().items.len() >= storage.inventory().max_slots() {
            result_events.send(StorageTransactionResult::DepositFailed {
                reason: "Storage is full".to_string(),
            });
            info!("Storage is full! Cannot deposit item.");
            continue;
        }

        // Remove from player inventory to get ownership
        let Some(inv_item) = inventory.remove_item(item_uuid) else {
            continue;
        };

        // Add to storage (should succeed since we checked capacity)
        storage
            .add_to_inv(inv_item.item)
            .expect("Storage capacity already verified");
        result_events.send(StorageTransactionResult::DepositSuccess {
            item_name: item_name.clone(),
        });
        deposited_events.send(ItemDeposited {
            item_name: item_name.clone(),
        });
        info!("Deposited {} into storage", item_name);
    }
}
