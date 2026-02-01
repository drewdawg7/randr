use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::inventory::{Inventory, ManagesItems};
use crate::player::PlayerGold;
use crate::ui::screens::merchant_modal::MerchantStock;

#[derive(Message, Debug, Clone)]
pub struct BuyItemEvent {
    pub stock_index: usize,
}

#[derive(Message, Debug, Clone)]
pub struct SellItemEvent {
    pub inventory_index: usize,
}

#[derive(Message, Debug, Clone)]
pub enum MerchantTransactionResult {
    BuySuccess { item_name: String, price: i32 },
    BuyFailedNotEnoughGold { need: i32, have: i32 },
    BuyFailedInventoryFull,
    BuyFailedNoItem,
    SellSuccess { item_name: String, price: i32 },
    SellFailedItemLocked,
    SellFailedNoItem,
}

pub struct MerchantPlugin;

impl Plugin for MerchantPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuyItemEvent>()
            .add_event::<SellItemEvent>()
            .add_event::<MerchantTransactionResult>()
            .add_systems(
                Update,
                (
                    handle_buy_item.run_if(on_event::<BuyItemEvent>),
                    handle_sell_item.run_if(on_event::<SellItemEvent>),
                ),
            );
    }
}

fn handle_buy_item(
    mut buy_events: MessageReader<BuyItemEvent>,
    mut result_events: MessageWriter<MerchantTransactionResult>,
    mut player_gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
    mut stock: Option<ResMut<MerchantStock>>,
) {
    let Some(ref mut stock) = stock else {
        return;
    };

    for event in buy_events.read() {
        let Some(store_item) = stock.items.get_mut(event.stock_index) else {
            result_events.write(MerchantTransactionResult::BuyFailedNoItem);
            continue;
        };

        let Some(item) = store_item.display_item() else {
            result_events.write(MerchantTransactionResult::BuyFailedNoItem);
            continue;
        };

        let price = item.purchase_price();
        let item_name = item.name.clone();

        if player_gold.0 < price {
            result_events.write(MerchantTransactionResult::BuyFailedNotEnoughGold {
                need: price,
                have: player_gold.0,
            });
            continue;
        }

        if inventory.get_inventory_items().len() >= inventory.max_slots() {
            result_events.write(MerchantTransactionResult::BuyFailedInventoryFull);
            continue;
        }

        let Some(purchased_item) = store_item.take_item() else {
            result_events.write(MerchantTransactionResult::BuyFailedNoItem);
            continue;
        };

        player_gold.subtract(price);
        let _ = inventory.add_to_inv(purchased_item);
        result_events.write(MerchantTransactionResult::BuySuccess { item_name, price });
    }
}

fn handle_sell_item(
    mut sell_events: MessageReader<SellItemEvent>,
    mut result_events: MessageWriter<MerchantTransactionResult>,
    mut player_gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
) {
    for event in sell_events.read() {
        let inv_items = inventory.get_inventory_items();
        let Some(inv_item) = inv_items.get(event.inventory_index) else {
            result_events.write(MerchantTransactionResult::SellFailedNoItem);
            continue;
        };

        if inv_item.item.is_locked {
            result_events.write(MerchantTransactionResult::SellFailedItemLocked);
            continue;
        }

        let sell_price = inv_item.item.sell_price();
        let item_name = inv_item.item.name.clone();
        let item_id = inv_item.item.item_id;

        player_gold.add(sell_price);
        inventory.decrease_item_quantity(item_id, 1);
        result_events.write(MerchantTransactionResult::SellSuccess {
            item_name,
            price: sell_price,
        });
    }
}
