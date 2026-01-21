use bevy::prelude::*;

use crate::game::{Storage, StorageDepositEvent, StorageWithdrawEvent};
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::Inventory;
use crate::location::{PurchaseEvent, SellEvent, Store};
use crate::screens::modal::ActiveModal;
use crate::screens::town::shared::SelectionState;

use super::state::{BuyFocus, StoreMode, StoreModeKind, StoreSelections};

/// Handle input for the Store tab.
pub fn handle_store_input(
    mut store_mode: ResMut<StoreMode>,
    mut store_selections: ResMut<StoreSelections>,
    mut action_events: EventReader<GameAction>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    store: Res<Store>,
    mut purchase_events: EventWriter<PurchaseEvent>,
    mut sell_events: EventWriter<SellEvent>,
    mut withdraw_events: EventWriter<StorageWithdrawEvent>,
    mut deposit_events: EventWriter<StorageDepositEvent>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal.is_some() {
        return;
    }

    for action in action_events.read() {
        match store_mode.mode {
            StoreModeKind::Menu => {
                handle_menu_input(&mut store_mode, &mut store_selections, &store, action)
            }
            StoreModeKind::Buy => {
                handle_buy_input(
                    &mut store_mode,
                    &mut store_selections,
                    action,
                    &store,
                    &inventory,
                    &mut purchase_events,
                    &mut sell_events,
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
    store: &Store,
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
                store_selections.buy.set_count(store.inventory.len());
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
/// Space toggles between store and inventory focus.
/// Enter purchases from store or sells from inventory based on focus.
fn handle_buy_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    store: &Store,
    inventory: &Inventory,
    purchase_events: &mut EventWriter<PurchaseEvent>,
    sell_events: &mut EventWriter<SellEvent>,
) {
    let grid_width = 5;

    // Update inventory count for proper navigation bounds
    store_selections.buy_inventory.set_count(inventory.items.len());

    match action {
        GameAction::Mine => {
            // Toggle focus between store and inventory
            store_selections.buy_focus = match store_selections.buy_focus {
                BuyFocus::Store => BuyFocus::Inventory,
                BuyFocus::Inventory => BuyFocus::Store,
            };
        }
        GameAction::Navigate(dir) => {
            // Navigate within the focused grid
            match store_selections.buy_focus {
                BuyFocus::Store => {
                    navigate_grid(
                        &mut store_selections.buy,
                        *dir,
                        store.inventory.len(),
                        grid_width,
                    );
                }
                BuyFocus::Inventory => {
                    navigate_grid(
                        &mut store_selections.buy_inventory,
                        *dir,
                        inventory.items.len(),
                        grid_width,
                    );
                }
            }
        }
        GameAction::Select => {
            match store_selections.buy_focus {
                BuyFocus::Store => {
                    // Purchase item from store
                    if store_selections.buy.selected < store.inventory.len() {
                        purchase_events.send(PurchaseEvent {
                            index: store_selections.buy.selected,
                        });
                    }
                }
                BuyFocus::Inventory => {
                    // Sell item from inventory
                    if store_selections.buy_inventory.selected < inventory.items.len() {
                        sell_events.send(SellEvent {
                            inventory_index: store_selections.buy_inventory.selected,
                        });

                        // Adjust selection for removed item
                        let new_count = inventory.items.len().saturating_sub(1);
                        store_selections.buy_inventory.set_count(new_count);
                        if store_selections.buy_inventory.selected >= new_count && new_count > 0 {
                            store_selections.buy_inventory.selected = new_count - 1;
                        }
                    }
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.buy.reset();
            store_selections.buy_inventory.reset();
            store_selections.buy_focus = BuyFocus::Store;
        }
        _ => {}
    }
}

/// Helper to navigate within a 2D grid.
fn navigate_grid(
    selection: &mut SelectionState,
    dir: NavigationDirection,
    count: usize,
    grid_width: usize,
) {
    if count == 0 {
        return;
    }

    match dir {
        NavigationDirection::Up => {
            if selection.selected >= grid_width {
                selection.selected -= grid_width;
            }
        }
        NavigationDirection::Down => {
            if selection.selected + grid_width < count {
                selection.selected += grid_width;
            }
        }
        NavigationDirection::Left => {
            let col = selection.selected % grid_width;
            if col > 0 {
                selection.selected -= 1;
            }
        }
        NavigationDirection::Right => {
            let col = selection.selected % grid_width;
            if col < grid_width - 1 && selection.selected + 1 < count {
                selection.selected += 1;
            }
        }
    }
}

/// Handle input for the sell screen.
fn handle_sell_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    inventory: &Inventory,
    action: &GameAction,
    sell_events: &mut EventWriter<SellEvent>,
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
                sell_events.send(SellEvent {
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
