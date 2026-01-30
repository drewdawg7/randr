use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::input::GameAction;
use crate::inventory::{Inventory, ManagesItems};
use crate::player::PlayerGold;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::widgets::ItemGrid;

use super::state::{MerchantPlayerGrid, MerchantStock, MerchantStockGrid};

/// System to handle Tab key toggling focus between merchant stock and player inventory grids.
/// Only runs when merchant modal is active (via run_if condition).
pub fn handle_merchant_modal_tab(
    mut action_reader: EventReader<GameAction>,
    mut focus_state: Option<ResMut<FocusState>>,
) {
    let Some(ref mut focus_state) = focus_state else {
        return;
    };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            focus_state.toggle_between(FocusPanel::MerchantStock, FocusPanel::PlayerInventory);
        }
    }
}

/// System to handle arrow key navigation within the focused merchant modal grid.
/// Only runs when merchant modal is active (via run_if condition).
pub fn handle_merchant_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if focus_state.is_focused(FocusPanel::MerchantStock) {
                if let Ok(mut grid) = stock_grids.get_single_mut() {
                    grid.navigate(*direction);
                }
            } else if focus_state.is_focused(FocusPanel::PlayerInventory) {
                if let Ok(mut grid) = player_grids.get_single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

/// System to handle Enter key for buying/selling items.
/// Only runs when merchant modal is active (via run_if condition).
pub fn handle_merchant_modal_select(
    mut action_reader: EventReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut player_gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
    mut stock: Option<ResMut<MerchantStock>>,
    stock_grids: Query<&ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    player_grids: Query<&ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    let Some(ref mut stock) = stock else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let stock_focused = focus_state.is_focused(FocusPanel::MerchantStock);

        let mut transaction_occurred = false;

        if stock_focused {
            // BUY: Purchase item from merchant
            let Ok(stock_grid) = stock_grids.get_single() else {
                continue;
            };
            let selected = stock_grid.selected_index;

            if let Some(store_item) = stock.items.get_mut(selected) {
                if let Some(item) = store_item.display_item() {
                    let price = item.purchase_price();

                    // Check if player has enough gold
                    if player_gold.0 >= price {
                        // Check if inventory has space
                        let has_space = inventory.get_inventory_items().len() < inventory.max_slots();
                        if has_space {
                            // Take item from stock
                            if let Some(purchased_item) = store_item.take_item() {
                                // Deduct gold
                                player_gold.subtract(price);
                                // Add to inventory
                                let _ = inventory.add_to_inv(purchased_item);
                                transaction_occurred = true;
                            }
                        }
                    }
                }
            }
        } else {
            // SELL: Sell item from player inventory
            let Ok(player_grid) = player_grids.get_single() else {
                continue;
            };
            let selected = player_grid.selected_index;

            let inv_items = inventory.get_inventory_items();
            if let Some(inv_item) = inv_items.get(selected) {
                // Don't sell locked items
                if !inv_item.item.is_locked {
                    let sell_price = inv_item.item.sell_price();
                    let item_id = inv_item.item.item_id;

                    // Add gold
                    player_gold.add(sell_price);
                    // Decrease quantity by 1 (removes item if quantity reaches 0)
                    inventory.decrease_item_quantity(item_id, 1);
                    transaction_occurred = true;
                }
            }
        }

        // Grids will be refreshed reactively by sync_merchant_grids when
        // inventory or stock changes are detected
        let _ = transaction_occurred;
    }
}

