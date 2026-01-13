use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, SpriteSheetKey, UiAllSlice};
use crate::game::Storage;
use crate::inventory::{Inventory, InventoryItem};
use crate::screens::town::shared::spawn_menu;
use crate::ui::spawn_navigation_hint;
use crate::screens::town::TabContent;
use crate::ui::UiText;
use crate::stats::StatType;
use crate::ui::widgets::{GoldDisplay, ItemGrid, ItemGridEntry, StoreListItem};
use crate::ui::{selection_colors, selection_prefix};

use super::constants::{BUYABLE_ITEMS, STORAGE_MENU_OPTIONS, STORE_MENU_OPTIONS};
use super::state::{BuyFocus, StoreModeKind, StoreMode, StoreSelections};

/// Marker for the text of a store list item.
#[derive(Component)]
pub struct StoreListItemText;

/// Source of items for the info panel.
#[derive(Clone, Copy)]
pub enum InfoPanelSource {
    /// Display item from store's buyable items
    Store { selected_index: usize },
    /// Display item from player's inventory
    Inventory { selected_index: usize },
}

/// Marker component for the store info panel that displays selected item details.
#[derive(Component)]
pub struct StoreInfoPanel {
    pub source: InfoPanelSource,
}

/// Spawn the store UI based on current mode.
pub fn spawn_store_ui(
    commands: &mut Commands,
    content_entity: Entity,
    store_mode: &StoreMode,
    store_selections: &StoreSelections,
    inventory: &Inventory,
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
                StoreModeKind::Buy => spawn_buy_ui(content, store_selections, inventory, game_sprites),
                StoreModeKind::Sell => spawn_sell_ui(content, store_selections, inventory),
                StoreModeKind::StorageMenu => spawn_storage_menu_ui(content, store_selections),
                StoreModeKind::StorageView => {
                    spawn_storage_view_ui(content, store_selections, storage)
                }
                StoreModeKind::StorageDeposit => {
                    spawn_storage_deposit_ui(content, store_selections, inventory)
                }
            });
    });
}

/// Spawn the main menu UI.
fn spawn_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn(UiText::new("Welcome to the Store").heading().yellow().margin_bottom(10.0).build_with_node());

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
    inventory: &Inventory,
    game_sprites: &Res<GameSprites>,
) {
    let store_focused = store_selections.buy_focus == BuyFocus::Store;

    // Title
    parent.spawn(UiText::new("Buy Items").heading().yellow().margin_bottom(10.0).build_with_node());

    // Main row container with store on far left, inventory on far right
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            // LEFT COLUMN: Store info panel and grid
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|col| {
                spawn_info_panel(
                    col,
                    InfoPanelSource::Store { selected_index: store_selections.buy.selected },
                    game_sprites,
                );

                // Store grid
                col.spawn(ItemGrid {
                    items: BUYABLE_ITEMS
                        .iter()
                        .map(|item| ItemGridEntry {
                            sprite_name: item.item_id.sprite_name().to_string(),
                        })
                        .collect(),
                    selected_index: store_selections.buy.selected,
                    is_focused: store_focused,
                });
            });

            // RIGHT COLUMN: Inventory info panel and grid
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|col| {
                spawn_info_panel(
                    col,
                    InfoPanelSource::Inventory { selected_index: store_selections.buy_inventory.selected },
                    game_sprites,
                );

                // Inventory grid
                let inventory_entries: Vec<ItemGridEntry> = inventory
                    .items
                    .iter()
                    .map(|inv_item| ItemGridEntry {
                        sprite_name: inv_item.item.item_id.sprite_name().to_string(),
                    })
                    .collect();

                col.spawn(ItemGrid {
                    items: inventory_entries,
                    selected_index: store_selections.buy_inventory.selected,
                    is_focused: !store_focused,
                });
            });
        });

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓←→] Navigate  [Space] Switch Panel  [Enter] Buy  [Backspace] Back");
}

/// Spawn an info panel with the given source.
fn spawn_info_panel(
    parent: &mut ChildBuilder,
    source: InfoPanelSource,
    game_sprites: &Res<GameSprites>,
) {
    let panel_width = 240.0; // 5 columns × 48px
    let panel_height = 144.0; // 3 rows × 48px

    let panel_image = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node_sliced(UiAllSlice::InfoPanelBg.as_str(), 10.0));

    let mut panel = parent.spawn((
        StoreInfoPanel { source },
        Node {
            width: Val::Px(panel_width),
            height: Val::Px(panel_height),
            margin: UiRect::bottom(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(12.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
    ));
    if let Some(img) = panel_image {
        panel.insert(img);
    }
}

/// Spawn the sell screen UI.
fn spawn_sell_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections, inventory: &Inventory) {
    // Title
    parent.spawn(UiText::new("Sell Items").heading().yellow().margin_bottom(10.0).build_with_node());

    spawn_inventory_list(
        parent,
        inventory.items.as_slice(),
        store_selections.sell.selected,
        "You have no items to sell.",
        Some(|item: &InventoryItem| {
            let sell_price = (item.item.gold_value as f32 * 0.5) as i32;
            format!("{} gold", sell_price)
        }),
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Sell  [Backspace] Back");
}

/// Spawn the storage menu UI.
fn spawn_storage_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
    // Title
    parent.spawn(UiText::new("Storage").heading().yellow().margin_bottom(10.0).build_with_node());

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
    parent.spawn(UiText::new("Storage - View & Withdraw").heading().yellow().margin_bottom(10.0).build_with_node());

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
fn spawn_storage_deposit_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(UiText::new("Storage - Deposit Items").heading().yellow().margin_bottom(10.0).build_with_node());

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

/// Spawn an inventory item list with optional extra column.
fn spawn_inventory_list<F>(
    parent: &mut ChildBuilder,
    items: &[InventoryItem],
    selected_index: usize,
    empty_message: &str,
    extra_column: Option<F>,
) where
    F: Fn(&InventoryItem) -> String,
{
    if items.is_empty() {
        parent.spawn(UiText::new(empty_message).dark_gray().build());
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

/// System to populate the store info panel with item details.
pub fn populate_store_info_panel(
    mut commands: Commands,
    query: Query<(Entity, &StoreInfoPanel)>,
    inventory: Res<Inventory>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, panel) in &query {
        // Dark brown text color
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));

        match panel.source {
            InfoPanelSource::Store { selected_index } => {
                // Get the selected item from BUYABLE_ITEMS
                let Some(item) = BUYABLE_ITEMS.get(selected_index) else {
                    continue;
                };

                // Get item spec for stats
                let spec = item.item_id.spec();

                // Remove the marker and add children with item details
                commands
                    .entity(entity)
                    .remove::<StoreInfoPanel>()
                    .with_children(|parent| {
                        // Item name
                        parent.spawn((
                            Text::new(item.name),
                            game_fonts.pixel_font(24.0),
                            text_color,
                        ));

                        // Stats (only show non-zero stats)
                        for stat_type in StatType::all() {
                            let value = spec.stats.value(*stat_type);
                            if value > 0 {
                                let stat_name = match stat_type {
                                    StatType::Health => "HP",
                                    StatType::Attack => "ATK",
                                    StatType::Defense => "DEF",
                                    StatType::GoldFind => "Gold Find",
                                    StatType::Mining => "Mining",
                                    StatType::MagicFind => "Magic Find",
                                };
                                parent.spawn((
                                    Text::new(format!("{}: +{}", stat_name, value)),
                                    game_fonts.pixel_font(18.0),
                                    text_color,
                                ));
                            }
                        }

                        // Cost with gold icon
                        parent.spawn(
                            GoldDisplay::new(item.price)
                                .with_font_size(18.0)
                                .with_color(text_color.0),
                        );
                    });
            }
            InfoPanelSource::Inventory { selected_index } => {
                // Get the selected item from inventory
                let inv_item = inventory.items.get(selected_index);

                // Remove the marker and add children with item details
                commands
                    .entity(entity)
                    .remove::<StoreInfoPanel>()
                    .with_children(|parent| {
                        if let Some(inv_item) = inv_item {
                            // Item name
                            parent.spawn((
                                Text::new(&inv_item.item.name),
                                game_fonts.pixel_font(24.0),
                                text_color,
                            ));

                            // Stats (only show non-zero stats)
                            for stat_type in StatType::all() {
                                let value = inv_item.item.stats.value(*stat_type);
                                if value > 0 {
                                    let stat_name = match stat_type {
                                        StatType::Health => "HP",
                                        StatType::Attack => "ATK",
                                        StatType::Defense => "DEF",
                                        StatType::GoldFind => "Gold Find",
                                        StatType::Mining => "Mining",
                                        StatType::MagicFind => "Magic Find",
                                    };
                                    parent.spawn((
                                        Text::new(format!("{}: +{}", stat_name, value)),
                                        game_fonts.pixel_font(18.0),
                                        text_color,
                                    ));
                                }
                            }

                            // Quantity (if stackable)
                            if inv_item.quantity > 1 {
                                parent.spawn((
                                    Text::new(format!("Qty: {}", inv_item.quantity)),
                                    game_fonts.pixel_font(18.0),
                                    text_color,
                                ));
                            }
                        } else {
                            // Empty inventory slot
                            parent.spawn((
                                Text::new("Empty"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                            ));
                        }
                    });
            }
        }
    }
}
