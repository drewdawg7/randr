use bevy::prelude::*;

use crate::game::Storage;
use crate::inventory::{Inventory, InventoryItem};
use crate::screens::town::shared::{spawn_menu, MenuOption};
use crate::ui::spawn_navigation_hint;
use crate::ui::UiText;

use super::super::state::StoreSelections;
use super::helpers::spawn_inventory_list;

/// Menu options for the storage submenu.
const STORAGE_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "View Storage",
        description: Some("View and withdraw stored items"),
    },
    MenuOption {
        label: "Deposit Items",
        description: Some("Store items from your inventory"),
    },
];

/// Spawn the storage menu UI.
pub fn spawn_storage_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn(
        UiText::new("Storage")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    // Menu options
    spawn_menu(
        parent,
        STORAGE_MENU_OPTIONS,
        store_selections.storage_menu.selected,
        None,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Select  [Backspace] Back");
}

/// Spawn the storage view/withdraw UI.
pub fn spawn_storage_view_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    storage: &Storage,
) {
    // Title
    parent.spawn(
        UiText::new("Storage - View & Withdraw")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    spawn_inventory_list(
        parent,
        storage.inventory.items.as_slice(),
        store_selections.storage_view.selected,
        "Storage is empty.",
        None::<fn(&InventoryItem) -> String>,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Withdraw  [Backspace] Back");
}

/// Spawn the storage deposit UI.
pub fn spawn_storage_deposit_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(
        UiText::new("Storage - Deposit Items")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    spawn_inventory_list(
        parent,
        inventory.items.as_slice(),
        store_selections.deposit.selected,
        "You have no items to deposit.",
        None::<fn(&InventoryItem) -> String>,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Deposit  [Backspace] Back");
}
