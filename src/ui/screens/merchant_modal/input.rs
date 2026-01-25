use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::player::PlayerGold;
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::ItemGrid;

use super::render::{get_merchant_stock_entries, get_player_inventory_entries};
use super::state::{MerchantPlayerGrid, MerchantStock, MerchantStockGrid};

/// System to handle Tab key toggling focus between merchant stock and player inventory grids.
pub fn handle_merchant_modal_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    if active_modal.modal != Some(ModalType::MerchantModal) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            let Ok(mut stock_grid) = stock_grids.get_single_mut() else {
                return;
            };
            let stock_was_focused = stock_grid.is_focused;
            stock_grid.is_focused = !stock_was_focused;

            let Ok(mut player_grid) = player_grids.get_single_mut() else {
                return;
            };
            player_grid.is_focused = stock_was_focused;
        }
    }
}

/// System to handle arrow key navigation within the focused merchant modal grid.
pub fn handle_merchant_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    if active_modal.modal != Some(ModalType::MerchantModal) {
        return;
    }

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if let Ok(mut grid) = stock_grids.get_single_mut() {
                if grid.is_focused {
                    navigate_grid(&mut grid, *direction);
                    continue;
                }
            }
            if let Ok(mut grid) = player_grids.get_single_mut() {
                if grid.is_focused {
                    navigate_grid(&mut grid, *direction);
                }
            }
        }
    }
}

/// System to handle Enter key for buying/selling items.
pub fn handle_merchant_modal_select(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut player_gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
    mut stock: Option<ResMut<MerchantStock>>,
    mut stock_grids: Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    if active_modal.modal != Some(ModalType::MerchantModal) {
        return;
    }

    let Some(ref mut stock) = stock else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        let stock_focused = stock_grids
            .get_single()
            .map(|g| g.is_focused)
            .unwrap_or(false);

        if stock_focused {
            // BUY: Purchase item from merchant
            let selected = stock_grids.get_single().unwrap().selected_index;

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
                            }
                        }
                    }
                }
            }
        } else {
            // SELL: Sell item from player inventory
            let selected = player_grids.get_single().unwrap().selected_index;

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
                }
            }
        }

        // Refresh both grids with updated data
        refresh_grids(stock, &inventory, &mut stock_grids, &mut player_grids);
    }
}

fn refresh_grids(
    stock: &MerchantStock,
    inventory: &Inventory,
    stock_grids: &mut Query<&mut ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    player_grids: &mut Query<&mut ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    if let Ok(mut grid) = stock_grids.get_single_mut() {
        grid.items = get_merchant_stock_entries(stock);
        if !grid.items.is_empty() {
            grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
        } else {
            grid.selected_index = 0;
        }
    }

    if let Ok(mut grid) = player_grids.get_single_mut() {
        grid.items = get_player_inventory_entries(inventory);
        if !grid.items.is_empty() {
            grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
        } else {
            grid.selected_index = 0;
        }
    }
}

fn navigate_grid(grid: &mut ItemGrid, direction: NavigationDirection) {
    let gs = grid.grid_size;
    let item_count = grid.items.len();
    if item_count == 0 {
        return;
    }

    let current = grid.selected_index;
    let row = current / gs;
    let col = current % gs;

    let new_index = match direction {
        NavigationDirection::Left if col > 0 => current - 1,
        NavigationDirection::Right if col < gs - 1 => current + 1,
        NavigationDirection::Up if row > 0 => current - gs,
        NavigationDirection::Down if row < gs - 1 => current + gs,
        _ => current,
    };

    if new_index < item_count {
        grid.selected_index = new_index;
    }
}
