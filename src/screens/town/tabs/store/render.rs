use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

use crate::assets::GameSprites;
use crate::game::{Player, Storage};
use crate::screens::town::shared::spawn_menu;
use crate::screens::town::TabContent;
use crate::ui::widgets::{ItemGrid, ItemGridEntry};
use crate::ui::{selection_colors, selection_prefix};

use super::constants::{STORAGE_MENU_OPTIONS, STORE_MENU_OPTIONS};
use super::state::{StoreModeKind, StoreMode, StoreSelections};

/// Spawn the store UI based on current mode.
pub fn spawn_store_ui(
    commands: &mut Commands,
    content_entity: Entity,
    store_mode: &StoreMode,
    store_selections: &StoreSelections,
    player: &Player,
    storage: &Storage,
    game_sprites: &Res<GameSprites>,
) {
    commands.entity(content_entity).with_children(|parent| {
        parent
            .spawn((
                TabContent,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
            ))
            .with_children(|content| match store_mode.mode {
                StoreModeKind::Menu => spawn_menu_ui(content, store_selections),
                StoreModeKind::Buy => spawn_buy_ui(content, store_selections, player, game_sprites),
                StoreModeKind::Sell => spawn_sell_ui(content, store_selections, player),
                StoreModeKind::StorageMenu => spawn_storage_menu_ui(content, store_selections),
                StoreModeKind::StorageView => {
                    spawn_storage_view_ui(content, store_selections, storage)
                }
                StoreModeKind::StorageDeposit => {
                    spawn_storage_deposit_ui(content, store_selections, player)
                }
            });
    });
}

/// Spawn the main menu UI.
fn spawn_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn((
        Text::new("Welcome to the Store"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

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

/// Spawn the buy screen UI.
fn spawn_buy_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    _player: &Player,
    game_sprites: &Res<GameSprites>,
) {
    // Title
    parent.spawn((
        Text::new("Buy Items"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Info panel above the grid (same width as grid, 3 rows tall)
    let panel_width = 240.0; // 5 columns × 48px
    let panel_height = 144.0; // 3 rows × 48px

    let panel_image = game_sprites.ui_all.as_ref().and_then(|ui_all| {
        ui_all.get("Slice_2").map(|idx| {
            ImageNode::from_atlas_image(
                ui_all.texture.clone(),
                TextureAtlas {
                    layout: ui_all.layout.clone(),
                    index: idx,
                },
            )
            .with_mode(NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(10.0),
                ..default()
            }))
        })
    });

    let mut panel = parent.spawn(Node {
        width: Val::Px(panel_width),
        height: Val::Px(panel_height),
        margin: UiRect::bottom(Val::Px(10.0)),
        ..default()
    });
    if let Some(img) = panel_image {
        panel.insert(img);
    }

    // Item grid with store items
    parent.spawn(ItemGrid {
        items: vec![
            ItemGridEntry { sprite_name: "Slice_337" }, // HP Potion
            ItemGridEntry { sprite_name: "Slice_155" }, // Sword
            ItemGridEntry { sprite_name: "Slice_100" }, // Shield
        ],
        selected_index: store_selections.buy.selected,
    });

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Buy  [Backspace] Back");
}

/// Spawn the sell screen UI.
fn spawn_sell_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections, player: &Player) {
    // Title
    parent.spawn((
        Text::new("Sell Items"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Get player inventory items
    let inventory_items = player.inventory.items.as_slice();

    if inventory_items.is_empty() {
        parent.spawn((
            Text::new("You have no items to sell."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    } else {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|list| {
                for (i, inv_item) in inventory_items.iter().enumerate() {
                    let is_selected = i == store_selections.sell.selected;

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);
                    let sell_price = (inv_item.item.gold_value as f32 * 0.5) as i32;

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, inv_item.item.name)),
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

                        // Sell price
                        item_row.spawn((
                            Text::new(format!("{} gold", sell_price)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.8, 0.3)),
                        ));
                    });
                }
            });
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Sell  [Backspace] Back");
}

/// Spawn the storage menu UI.
fn spawn_storage_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn((
        Text::new("Storage"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

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
fn spawn_storage_view_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    storage: &Storage,
) {
    // Title
    parent.spawn((
        Text::new("Storage - View & Withdraw"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Get storage items
    let storage_items = storage.inventory.items.as_slice();

    if storage_items.is_empty() {
        parent.spawn((
            Text::new("Storage is empty."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    } else {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|list| {
                for (i, inv_item) in storage_items.iter().enumerate() {
                    let is_selected = i == store_selections.storage_view.selected;

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, inv_item.item.name)),
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
                            ));
                        }
                    });
                }
            });
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Withdraw  [Backspace] Back");
}

/// Spawn the storage deposit UI.
fn spawn_storage_deposit_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    player: &Player,
) {
    // Title
    parent.spawn((
        Text::new("Storage - Deposit Items"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Get player inventory items
    let inventory_items = player.inventory.items.as_slice();

    if inventory_items.is_empty() {
        parent.spawn((
            Text::new("You have no items to deposit."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    } else {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|list| {
                for (i, inv_item) in inventory_items.iter().enumerate() {
                    let is_selected = i == store_selections.deposit.selected;

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, inv_item.item.name)),
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
                            ));
                        }
                    });
                }
            });
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Deposit  [Backspace] Back");
}

/// Spawn navigation hint at the bottom.
fn spawn_navigation_hint(parent: &mut ChildBuilder, hint: &str) {
    parent.spawn((
        Text::new(hint),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}
