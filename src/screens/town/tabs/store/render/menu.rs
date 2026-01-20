use bevy::prelude::*;

use crate::screens::town::shared::{spawn_menu, MenuOption};
use crate::ui::spawn_navigation_hint;
use crate::ui::UiText;

use super::super::state::StoreSelections;

/// Menu options for the store main menu.
const STORE_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "Buy",
        description: Some("Purchase items"),
    },
    MenuOption {
        label: "Sell",
        description: Some("Sell your items"),
    },
    MenuOption {
        label: "Storage",
        description: Some("Access your storage"),
    },
];

/// Spawn the main menu UI.
pub fn spawn_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn(
        UiText::new("Welcome to the Store")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    // Menu options
    spawn_menu(
        parent,
        STORE_MENU_OPTIONS,
        store_selections.menu.selected,
        None,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab");
}
