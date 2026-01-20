use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, SpriteSheetKey, UiAllSlice};
use crate::economy::WorthGold;
use crate::game::Storage;
use crate::inventory::{Inventory, InventoryItem};
use crate::location::Store;
use crate::screens::town::shared::{spawn_menu, MenuOption};
use crate::screens::town::{ContentArea, TabContent};
use crate::stats::StatType;
use crate::ui::spawn_navigation_hint;
use crate::ui::widgets::{CentralDetailPanel, GoldDisplay, ItemGrid, ItemGridEntry, StoreListItem};
use crate::ui::{selection_colors, selection_prefix, UiText};

use super::state::{BuyFocus, StoreMode, StoreModeKind, StoreSelections};

/// Cached sprites for store UI, populated once when GameSprites loads.
#[derive(Resource, Default)]
pub struct StoreUiCache {
    pub info_panel_bg: Option<ImageNode>,
}

/// System to populate the store UI cache from GameSprites.
pub fn cache_store_ui_sprites(
    mut cache: ResMut<StoreUiCache>,
    game_sprites: Res<GameSprites>,
) {
    if cache.info_panel_bg.is_some() {
        return;
    }

    cache.info_panel_bg = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node_sliced(UiAllSlice::InfoPanelBg.as_str(), 10.0));
}

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

/// Marker for the text of a store list item.
#[derive(Component)]
pub struct StoreListItemText;

/// Source of items for the info panel.
#[derive(Clone, Copy)]
pub enum InfoPanelSource {
    /// Display item from store's inventory
    Store { selected_index: usize },
    /// Display item from player's inventory
    Inventory { selected_index: usize },
}

/// Marker component for the store info panel that displays selected item details.
#[derive(Component)]
pub struct StoreInfoPanel {
    pub source: InfoPanelSource,
}

/// System to spawn store UI content when entering the Store tab.
pub fn spawn_store_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    store: Res<Store>,
    ui_cache: Res<StoreUiCache>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_store_ui_inner(
        &mut commands,
        content_entity,
        &store_mode,
        &store_selections,
        &inventory,
        &storage,
        &store,
        &ui_cache,
    );
}

pub fn refresh_store_ui(
    mut commands: Commands,
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    store: Res<Store>,
    ui_cache: Res<StoreUiCache>,
) {
    let mode_changed = store_mode.is_changed();
    let inventory_changed_in_buy =
        inventory.is_changed() && store_mode.mode == StoreModeKind::Buy;
    let selections_changed_in_buy =
        store_selections.is_changed() && store_mode.mode == StoreModeKind::Buy;

    if !mode_changed && !inventory_changed_in_buy && !selections_changed_in_buy {
        return;
    }

    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_store_ui_inner(
        &mut commands,
        content_entity,
        &store_mode,
        &store_selections,
        &inventory,
        &storage,
        &store,
        &ui_cache,
    );
}

/// Internal helper to spawn store UI.
fn spawn_store_ui_inner(
    commands: &mut Commands,
    content_entity: Entity,
    store_mode: &StoreMode,
    store_selections: &StoreSelections,
    inventory: &Inventory,
    storage: &Storage,
    store: &Store,
    ui_cache: &StoreUiCache,
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
                StoreModeKind::Buy => {
                    spawn_buy_ui(content, store_selections, store, inventory, ui_cache)
                }
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

fn spawn_buy_ui(
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

fn spawn_sell_ui(
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

/// Spawn the storage menu UI.
fn spawn_storage_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections) {
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
fn spawn_storage_view_ui(
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
fn spawn_storage_deposit_ui(
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
    store: Res<Store>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, panel) in &query {
        // Dark brown text color
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));

        match panel.source {
            InfoPanelSource::Store { selected_index } => {
                // Get the selected store item
                let Some(store_item) = store.inventory.get(selected_index) else {
                    continue;
                };

                // Get display item for stats/price
                let display_item = store_item.display_item();

                // Remove the marker and add children with item details
                commands
                    .entity(entity)
                    .remove::<StoreInfoPanel>()
                    .with_children(|parent| {
                        if let Some(item) = display_item {
                            // Item name
                            parent.spawn((
                                Text::new(&item.name),
                                game_fonts.pixel_font(24.0),
                                text_color,
                            ));

                            // Stats (only show non-zero stats)
                            for stat_type in StatType::all() {
                                let value = item.stats.value(*stat_type);
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
                                GoldDisplay::new(item.purchase_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                            );
                        } else {
                            // Out of stock
                            parent.spawn((
                                Text::new("Out of Stock"),
                                game_fonts.pixel_font(18.0),
                                text_color,
                            ));
                        }
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

                            // Sell price
                            parent.spawn(
                                GoldDisplay::new(inv_item.item.sell_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                            );
                        } else {
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

pub fn populate_central_detail_panel(
    mut commands: Commands,
    query: Query<(Entity, &CentralDetailPanel)>,
    inventory: Res<Inventory>,
    store: Res<Store>,
) {
    for (entity, panel) in &query {
        let text_color = TextColor(Color::srgb(0.4, 0.25, 0.15));

        match panel.source {
            InfoPanelSource::Store { selected_index } => {
                let Some(store_item) = store.inventory.get(selected_index) else {
                    continue;
                };

                let display_item = store_item.display_item();

                commands
                    .entity(entity)
                    .remove::<CentralDetailPanel>()
                    .with_children(|parent| {
                        if let Some(item) = display_item {
                            parent.spawn((
                                Text::new(&item.name),
                                TextFont { font_size: 24.0, ..default() },
                                text_color,
                                Node { position_type: PositionType::Relative, ..default() },
                            ));

                            for stat_type in StatType::all() {
                                let value = item.stats.value(*stat_type);
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
                                        TextFont { font_size: 18.0, ..default() },
                                        text_color,
                                        Node { position_type: PositionType::Relative, ..default() },
                                    ));
                                }
                            }

                            parent.spawn((
                                GoldDisplay::new(item.purchase_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                                Node { position_type: PositionType::Relative, ..default() },
                            ));
                        } else {
                            parent.spawn((
                                Text::new("Out of Stock"),
                                TextFont { font_size: 18.0, ..default() },
                                text_color,
                                Node { position_type: PositionType::Relative, ..default() },
                            ));
                        }
                    });
            }
            InfoPanelSource::Inventory { selected_index } => {
                let inv_item = inventory.items.get(selected_index);

                commands
                    .entity(entity)
                    .remove::<CentralDetailPanel>()
                    .with_children(|parent| {
                        if let Some(inv_item) = inv_item {
                            parent.spawn((
                                Text::new(&inv_item.item.name),
                                TextFont { font_size: 24.0, ..default() },
                                text_color,
                                Node { position_type: PositionType::Relative, ..default() },
                            ));

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
                                        TextFont { font_size: 18.0, ..default() },
                                        text_color,
                                        Node { position_type: PositionType::Relative, ..default() },
                                    ));
                                }
                            }

                            parent.spawn((
                                GoldDisplay::new(inv_item.item.sell_price())
                                    .with_font_size(18.0)
                                    .with_color(text_color.0),
                                Node { position_type: PositionType::Relative, ..default() },
                            ));
                        } else {
                            parent.spawn((
                                Text::new("Empty"),
                                TextFont { font_size: 18.0, ..default() },
                                text_color,
                                Node { position_type: PositionType::Relative, ..default() },
                            ));
                        }
                    });
            }
        }
    }
}
