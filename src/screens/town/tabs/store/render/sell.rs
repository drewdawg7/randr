use bevy::prelude::*;

use crate::economy::WorthGold;
use crate::inventory::{Inventory, InventoryItem};
use crate::ui::spawn_navigation_hint;
use crate::ui::UiText;

use super::super::state::StoreSelections;
use super::helpers::spawn_inventory_list;

pub fn spawn_sell_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(
        UiText::new("Sell Items")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    spawn_inventory_list(
        parent,
        inventory.items.as_slice(),
        store_selections.sell.selected,
        "You have no items to sell.",
        Some(|item: &InventoryItem| {
            let sell_price = item.item.sell_price();
            format!("{} gold", sell_price)
        }),
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Sell  [Backspace] Back");
}
