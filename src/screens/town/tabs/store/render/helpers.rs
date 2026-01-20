use bevy::prelude::*;

use crate::inventory::InventoryItem;
use crate::ui::widgets::StoreListItem;
use crate::ui::{selection_colors, selection_prefix};

use super::StoreListItemText;

/// Spawn an inventory item list with optional extra column.
pub fn spawn_inventory_list<F>(
    parent: &mut ChildBuilder,
    items: &[InventoryItem],
    selected_index: usize,
    empty_message: &str,
    extra_column: Option<F>,
) where
    F: Fn(&InventoryItem) -> String,
{
    if items.is_empty() {
        parent.spawn((
            Text::new(empty_message),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
        ));
        return;
    }

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|list| {
            for (i, inv_item) in items.iter().enumerate() {
                let is_selected = i == selected_index;
                let (bg_color, text_color) = selection_colors(is_selected);
                let prefix = selection_prefix(is_selected);
                let item_name = inv_item.item.name.clone();

                list.spawn((
                    StoreListItem::new(i, item_name.clone()),
                    Node {
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                ))
                .with_children(|item_row| {
                    // Item name
                    item_row.spawn((
                        StoreListItemText,
                        Text::new(format!("{}{}", prefix, item_name)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(text_color),
                        Node {
                            width: Val::Px(200.0),
                            ..default()
                        },
                    ));

                    // Quantity (if > 1)
                    if inv_item.quantity > 1 {
                        item_row.spawn((
                            Text::new(format!("x{}", inv_item.quantity)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            Node {
                                width: Val::Px(60.0),
                                ..default()
                            },
                        ));
                    }

                    // Extra column (e.g., sell price)
                    if let Some(ref render_extra) = extra_column {
                        item_row.spawn((
                            Text::new(render_extra(inv_item)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.8, 0.3)),
                        ));
                    }
                });
            }
        });
}
