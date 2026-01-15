use bevy::prelude::*;

use crate::screens::town::shared::{update_menu_selection, MenuOptionItem, MenuOptionText};
use crate::ui::update_list_selection;
use crate::ui::widgets::StoreListItem;

use super::render::StoreListItemText;
use super::state::{StoreMode, StoreModeKind, StoreSelections};

/// Updates store selection highlighting reactively.
pub fn update_store_selection(
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    mut menu_query: Query<(&MenuOptionItem, &mut BackgroundColor, &Children)>,
    mut menu_text_query: Query<(&mut Text, &mut TextColor), With<MenuOptionText>>,
    mut list_query: Query<
        (&StoreListItem, &mut BackgroundColor, &Children),
        Without<MenuOptionItem>,
    >,
    mut list_text_query: Query<
        (&mut Text, &mut TextColor),
        (With<StoreListItemText>, Without<MenuOptionText>),
    >,
) {
    match store_mode.mode {
        StoreModeKind::Menu => {
            update_menu_selection(
                store_selections.menu.selected,
                &mut menu_query,
                &mut menu_text_query,
            );
        }
        StoreModeKind::StorageMenu => {
            update_menu_selection(
                store_selections.storage_menu.selected,
                &mut menu_query,
                &mut menu_text_query,
            );
        }
        StoreModeKind::Sell => {
            update_list_selection(
                store_selections.sell.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        StoreModeKind::StorageView => {
            update_list_selection(
                store_selections.storage_view.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        StoreModeKind::StorageDeposit => {
            update_list_selection(
                store_selections.deposit.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        StoreModeKind::Buy => {
            // Buy mode uses ItemGrid widget - handled by its own system
        }
    }
}
