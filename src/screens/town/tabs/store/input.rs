use bevy::prelude::*;

use crate::game::{
    Storage, StorageDepositEvent, StorageWithdrawEvent, StorePurchaseEvent, StoreSellEvent,
};
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::Inventory;

use super::constants::BUYABLE_ITEMS;
use super::state::{StoreModeKind, StoreMode, StoreSelections};

/// Handle input for the Store tab.
pub fn handle_store_input(
    mut store_mode: ResMut<StoreMode>,
    mut store_selections: ResMut<StoreSelections>,
    mut action_events: EventReader<GameAction>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    mut purchase_events: EventWriter<StorePurchaseEvent>,
    mut sell_events: EventWriter<StoreSellEvent>,
    mut withdraw_events: EventWriter<StorageWithdrawEvent>,
    mut deposit_events: EventWriter<StorageDepositEvent>,
) {
    for action in action_events.read() {
        match store_mode.mode {
            StoreModeKind::Menu => {
                handle_menu_input(&mut store_mode, &mut store_selections, action)
            }
            StoreModeKind::Buy => {
                handle_buy_input(
                    &mut store_mode,
                    &mut store_selections,
                    action,
                    &mut purchase_events,
                )
            }
            StoreModeKind::Sell => {
                handle_sell_input(
                    &mut store_mode,
                    &mut store_selections,
                    &inventory,
                    action,
                    &mut sell_events,
                )
            }
            StoreModeKind::StorageMenu => {
                handle_storage_menu_input(&mut store_mode, &mut store_selections, action)
            }
            StoreModeKind::StorageView => handle_storage_view_input(
                &mut store_mode,
                &mut store_selections,
                action,
                &storage,
                &mut withdraw_events,
            ),
            StoreModeKind::StorageDeposit => handle_storage_deposit_input(
                &mut store_mode,
                &mut store_selections,
                action,
                &inventory,
                &mut deposit_events,
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

/// Handle input for the buy screen with 2D grid navigation.
fn handle_buy_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    purchase_events: &mut EventWriter<StorePurchaseEvent>,
) {
    let items_count = BUYABLE_ITEMS.len();
    let grid_width = 5;

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            // Move up one row
            if store_selections.buy.selected >= grid_width {
                store_selections.buy.selected -= grid_width;
            }
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            // Move down one row
            if store_selections.buy.selected + grid_width < items_count {
                store_selections.buy.selected += grid_width;
            }
        }
        GameAction::Navigate(NavigationDirection::Left) => {
            // Move left one column
            let col = store_selections.buy.selected % grid_width;
            if col > 0 {
                store_selections.buy.selected -= 1;
            }
        }
        GameAction::Navigate(NavigationDirection::Right) => {
            // Move right one column
            let col = store_selections.buy.selected % grid_width;
            if col < grid_width - 1 && store_selections.buy.selected + 1 < items_count {
                store_selections.buy.selected += 1;
            }
        }
        GameAction::Select => {
            if let Some(buyable) = BUYABLE_ITEMS.get(store_selections.buy.selected) {
                purchase_events.send(StorePurchaseEvent {
                    item_id: buyable.item_id,
                    price: buyable.price,
                    item_name: buyable.name.to_string(),
                });
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
    inventory: &Inventory,
    action: &GameAction,
    sell_events: &mut EventWriter<StoreSellEvent>,
) {
    // Update selection count based on current inventory
    store_selections.sell.set_count(inventory.items.len());

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.sell.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.sell.move_down();
        }
        GameAction::Select => {
            if store_selections.sell.selected < inventory.items.len() {
                sell_events.send(StoreSellEvent {
                    inventory_index: store_selections.sell.selected,
                });

                // Adjust selection for removed item (will be processed next frame)
                let new_count = inventory.items.len().saturating_sub(1);
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
    storage: &Storage,
    withdraw_events: &mut EventWriter<StorageWithdrawEvent>,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.storage_view.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.storage_view.move_down();
        }
        GameAction::Select => {
            if store_selections.storage_view.selected < storage.inventory.items.len() {
                withdraw_events.send(StorageWithdrawEvent {
                    storage_index: store_selections.storage_view.selected,
                });
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
    inventory: &Inventory,
    deposit_events: &mut EventWriter<StorageDepositEvent>,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.deposit.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.deposit.move_down();
        }
        GameAction::Select => {
            if store_selections.deposit.selected < inventory.items.len() {
                deposit_events.send(StorageDepositEvent {
                    inventory_index: store_selections.deposit.selected,
                });
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::StorageMenu;
            store_selections.deposit.reset();
        }
        _ => {}
    }
}
