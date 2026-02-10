use bevy::prelude::*;

use crate::game::{ItemDeposited, ItemWithdrawn, Storage};
use crate::inventory::{FindsItems, HasInventory, Inventory, InventoryError, ManagesItems};
use crate::player::PlayerMarker;

#[derive(Message, Debug, Clone)]
pub struct StorageWithdrawEvent {
    pub storage_index: usize,
}

#[derive(Message, Debug, Clone)]
pub struct StorageDepositEvent {
    pub inventory_index: usize,
}

#[derive(Message, Debug, Clone)]
pub enum StorageTransactionResult {
    WithdrawSuccess { item_name: String },
    WithdrawFailedInventoryFull,
    DepositSuccess { item_name: String },
    DepositFailed { reason: String },
}

pub struct StorageTransactionsPlugin;

impl Plugin for StorageTransactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StorageWithdrawEvent>()
            .add_message::<StorageDepositEvent>()
            .add_message::<StorageTransactionResult>()
            .add_systems(
                Update,
                (
                    handle_storage_withdraw.run_if(on_message::<StorageWithdrawEvent>),
                    handle_storage_deposit.run_if(on_message::<StorageDepositEvent>),
                ),
            );
    }
}

fn handle_storage_withdraw(
    mut withdraw_events: MessageReader<StorageWithdrawEvent>,
    mut result_events: MessageWriter<StorageTransactionResult>,
    mut withdrawn_events: MessageWriter<ItemWithdrawn>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    mut storage: ResMut<Storage>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in withdraw_events.read() {
        let Some(inv_item) = storage.inventory.items.get(event.storage_index) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();
        let item_uuid = inv_item.uuid();

        if inventory.items.len() >= inventory.max_slots() {
            result_events.write(StorageTransactionResult::WithdrawFailedInventoryFull);
            info!("Inventory is full! Cannot withdraw item.");
            continue;
        }

        let Some(inv_item) = storage.remove_item(item_uuid) else {
            continue;
        };

        if inventory.add_to_inv(inv_item.item).is_ok() {
            result_events.write(StorageTransactionResult::WithdrawSuccess {
                item_name: item_name.clone(),
            });
            withdrawn_events.write(ItemWithdrawn {
                item_name: item_name.clone(),
            });
            info!("Withdrew {} from storage", item_name);
        }
    }
}

fn handle_storage_deposit(
    mut deposit_events: MessageReader<StorageDepositEvent>,
    mut result_events: MessageWriter<StorageTransactionResult>,
    mut deposited_events: MessageWriter<ItemDeposited>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    mut storage: ResMut<Storage>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in deposit_events.read() {
        let Some(inv_item) = inventory.items.get(event.inventory_index) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();
        let item_uuid = inv_item.uuid();

        if storage.inventory().items.len() >= storage.inventory().max_slots() {
            result_events.write(StorageTransactionResult::DepositFailed {
                reason: "Storage is full".to_string(),
            });
            info!("Storage is full! Cannot deposit item.");
            continue;
        }

        let Some(inv_item) = inventory.remove_item(item_uuid) else {
            continue;
        };

        match storage.add_to_inv(inv_item.item.clone()) {
            Ok(_) => {
                result_events.write(StorageTransactionResult::DepositSuccess {
                    item_name: item_name.clone(),
                });
                deposited_events.write(ItemDeposited {
                    item_name: item_name.clone(),
                });
                info!("Deposited {} into storage", item_name);
            }
            Err(InventoryError::Full) => {
                result_events.write(StorageTransactionResult::DepositFailed {
                    reason: "Storage is full (unexpected)".to_string(),
                });
                warn!("Storage full despite capacity check. Re-adding item to inventory.");
                let _ = inventory.add_to_inv(inv_item.item);
            }
        }
    }
}
