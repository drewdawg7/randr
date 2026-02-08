use bevy::prelude::*;

use crate::game::{BuyItemEvent, SellItemEvent};
use crate::input::GameAction;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::merchant_modal::{MerchantPlayerGrid, MerchantStockGrid};
use crate::ui::widgets::ItemGrid;

pub fn navigate_merchant_grid(
    mut action_reader: MessageReader<GameAction>,
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
                if let Ok(mut grid) = stock_grids.single_mut() {
                    grid.navigate(*direction);
                }
            } else if focus_state.is_focused(FocusPanel::PlayerInventory) {
                if let Ok(mut grid) = player_grids.single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

pub fn process_transaction(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut buy_events: MessageWriter<BuyItemEvent>,
    mut sell_events: MessageWriter<SellItemEvent>,
    stock_grids: Query<&ItemGrid, (With<MerchantStockGrid>, Without<MerchantPlayerGrid>)>,
    player_grids: Query<&ItemGrid, (With<MerchantPlayerGrid>, Without<MerchantStockGrid>)>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        if focus_state.is_focused(FocusPanel::MerchantStock) {
            let Ok(stock_grid) = stock_grids.single() else {
                continue;
            };
            buy_events.write(BuyItemEvent {
                stock_index: stock_grid.selected_index,
            });
        } else if focus_state.is_focused(FocusPanel::PlayerInventory) {
            let Ok(player_grid) = player_grids.single() else {
                continue;
            };
            sell_events.write(SellItemEvent {
                inventory_index: player_grid.selected_index,
            });
        }
    }
}
