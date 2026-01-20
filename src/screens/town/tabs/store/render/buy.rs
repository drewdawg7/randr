use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::location::Store;
use crate::ui::spawn_navigation_hint;
use crate::ui::widgets::{CentralDetailPanel, ItemGrid, ItemGridEntry};
use crate::ui::UiText;

use super::super::state::{BuyFocus, StoreSelections};
use super::{InfoPanelSource, StoreUiCache};

pub fn spawn_buy_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    store: &Store,
    inventory: &Inventory,
    _ui_cache: &StoreUiCache,
) {
    let store_focused = store_selections.buy_focus == BuyFocus::Store;

    parent.spawn(
        UiText::new("Buy Items")
            .heading()
            .yellow()
            .margin_bottom(10.0)
            .build_with_node(),
    );

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            width: Val::Percent(100.0),
            column_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn(ItemGrid {
                items: store
                    .inventory
                    .iter()
                    .map(|store_item| ItemGridEntry {
                        sprite_name: store_item.item_id.sprite_name().to_string(),
                    })
                    .collect(),
                selected_index: store_selections.buy.selected,
                is_focused: store_focused,
            });

            let source = if store_focused {
                InfoPanelSource::Store {
                    selected_index: store_selections.buy.selected,
                }
            } else {
                InfoPanelSource::Inventory {
                    selected_index: store_selections.buy_inventory.selected,
                }
            };
            row.spawn(CentralDetailPanel { source });

            let inventory_entries: Vec<ItemGridEntry> = inventory
                .items
                .iter()
                .map(|inv_item| ItemGridEntry {
                    sprite_name: inv_item.item.item_id.sprite_name().to_string(),
                })
                .collect();

            row.spawn(ItemGrid {
                items: inventory_entries,
                selected_index: store_selections.buy_inventory.selected,
                is_focused: !store_focused,
            });
        });

    spawn_navigation_hint(
        parent,
        "[↑↓←→] Navigate  [Space] Switch Panel  [Enter] Buy/Sell  [Backspace] Back",
    );
}
