use bevy::prelude::*;

use crate::game::{Player, Storage};
use crate::input::{GameAction, NavigationDirection};
use crate::{FindsItems, ManagesItems};

use super::constants::BUYABLE_ITEMS;
use super::state::{StoreModeKind, StoreMode, StoreSelections};

/// Handle input for the Store tab.
pub fn handle_store_input(
    mut store_mode: ResMut<StoreMode>,
    mut store_selections: ResMut<StoreSelections>,
    mut action_events: EventReader<GameAction>,
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
) {
    for action in action_events.read() {
        match store_mode.mode {
            StoreModeKind::Menu => {
                handle_menu_input(&mut store_mode, &mut store_selections, action)
            }
            StoreModeKind::Buy => {
                handle_buy_input(&mut store_mode, &mut store_selections, &mut player, action)
            }
            StoreModeKind::Sell => {
                handle_sell_input(&mut store_mode, &mut store_selections, &mut player, action)
            }
            StoreModeKind::StorageMenu => {
                handle_storage_menu_input(&mut store_mode, &mut store_selections, action)
            }
            StoreModeKind::StorageView => handle_storage_view_input(
                &mut store_mode,
                &mut store_selections,
                action,
                &mut player,
                &mut storage,
            ),
            StoreModeKind::StorageDeposit => handle_storage_deposit_input(
                &mut store_mode,
                &mut store_selections,
                action,
                &mut player,
                &mut storage,
            ),
        }
    }
}

/// Handle input for the main menu.
fn handle_menu_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.menu.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.menu.move_down();
        }
        GameAction::Select => match store_selections.menu.selected {
            0 => {
                store_mode.mode = StoreModeKind::Buy;
                store_selections.buy.set_count(BUYABLE_ITEMS.len());
            }
            1 => {
                store_mode.mode = StoreModeKind::Sell;
                // sell count will be updated in render
            }
            2 => {
                store_mode.mode = StoreModeKind::StorageMenu;
            }
            _ => {}
        },
        _ => {}
    }
}

/// Handle input for the buy screen.
fn handle_buy_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    player: &mut Player,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.buy.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.buy.move_down();
        }
        GameAction::Select => {
            if let Some(buyable) = BUYABLE_ITEMS.get(store_selections.buy.selected) {
                if player.gold >= buyable.price {
                    let new_item = buyable.item_id.spawn();
                    if player.add_to_inv(new_item).is_ok() {
                        player.gold -= buyable.price;
                        info!("Purchased {} for {} gold", buyable.name, buyable.price);
                    } else {
                        info!("Inventory full!");
                    }
                } else {
                    info!(
                        "Not enough gold! Need {} but have {}",
                        buyable.price, player.gold
                    );
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.buy.reset();
        }
        _ => {}
    }
}

/// Handle input for the sell screen.
fn handle_sell_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Update selection count based on current inventory
    store_selections.sell.set_count(player.inventory.items.len());

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.sell.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.sell.move_down();
        }
        GameAction::Select => {
            if let Some(inv_item) = player
                .inventory
                .items
                .get(store_selections.sell.selected)
                .cloned()
            {
                let sell_price = (inv_item.item.gold_value as f32 * 0.5) as i32;
                let item_name = inv_item.item.name.clone();

                // Add gold and remove item
                player.gold += sell_price;
                player.decrease_item_quantity(&inv_item, 1);
                info!("Sold {} for {} gold", item_name, sell_price);

                // Update selection if we removed the last item
                let new_count = player.inventory.items.len();
                store_selections.sell.set_count(new_count);
                if store_selections.sell.selected >= new_count && new_count > 0 {
                    store_selections.sell.selected = new_count - 1;
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.sell.reset();
        }
        _ => {}
    }
}

/// Handle input for the storage menu.
fn handle_storage_menu_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.storage_menu.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.storage_menu.move_down();
        }
        GameAction::Select => match store_selections.storage_menu.selected {
            0 => store_mode.mode = StoreModeKind::StorageView,
            1 => store_mode.mode = StoreModeKind::StorageDeposit,
            _ => {}
        },
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.storage_menu.reset();
        }
        _ => {}
    }
}

/// Handle input for viewing/withdrawing storage items.
fn handle_storage_view_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    player: &mut Player,
    storage: &mut Storage,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.storage_view.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.storage_view.move_down();
        }
        GameAction::Select => {
            // Withdraw item from storage
            let storage_items = storage.inventory.items.as_slice();
            if let Some(inv_item) = storage_items.get(store_selections.storage_view.selected) {
                let item = inv_item.item.clone();

                // Try to add to player inventory
                match player.add_to_inv(item.clone()) {
                    Ok(_) => {
                        // Remove from storage
                        let item_uuid = inv_item.uuid();
                        storage.remove_item(item_uuid);
                        info!("Withdrew {} from storage", item.name);
                    }
                    Err(_) => {
                        info!("Inventory is full! Cannot withdraw item.");
                    }
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::StorageMenu;
            store_selections.storage_view.reset();
        }
        _ => {}
    }
}

/// Handle input for depositing items into storage.
fn handle_storage_deposit_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    player: &mut Player,
    storage: &mut Storage,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.deposit.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.deposit.move_down();
        }
        GameAction::Select => {
            // Deposit item into storage
            let inventory_items = player.inventory.items.as_slice();
            if let Some(inv_item) = inventory_items.get(store_selections.deposit.selected) {
                let item = inv_item.item.clone();

                // Add to storage (storage has unlimited capacity)
                match storage.add_to_inv(item.clone()) {
                    Ok(_) => {
                        // Remove from player inventory
                        let item_uuid = inv_item.uuid();
                        player.remove_item(item_uuid);
                        info!("Deposited {} into storage", item.name);
                    }
                    Err(e) => {
                        info!("Failed to deposit item: {:?}", e);
                    }
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::StorageMenu;
            store_selections.deposit.reset();
        }
        _ => {}
    }
}
