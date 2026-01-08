use bevy::prelude::*;

use crate::game::{PlayerResource, StorageResource};
use crate::input::{GameAction, NavigationDirection};
use crate::item::ItemId;
use crate::states::AppState;
use crate::{FindsItems, ManagesItems};

use super::super::shared::{spawn_menu, MenuOption, SelectionState};
use super::super::{ContentArea, CurrentTab, TabContent, TownTab};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreTabState>().add_systems(
            Update,
            (
                handle_store_input,
                render_store_content.run_if(resource_changed::<StoreTabState>),
                render_store_on_tab_change.run_if(resource_changed::<CurrentTab>),
            )
                .run_if(in_state(AppState::Town))
                .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Store),
        );
    }
}

/// Store mode - what submenu the player is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreMode {
    #[default]
    Menu,
    Buy,
    Sell,
    Storage,
}

/// Storage submode - what the player is doing in storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageSubmode {
    #[default]
    Menu,
    View,
    Deposit,
}

/// Store tab state - tracks current mode and selections.
#[derive(Resource)]
pub struct StoreTabState {
    pub mode: StoreMode,
    pub menu_selection: SelectionState,
    pub buy_selection: SelectionState,
    pub sell_selection: SelectionState,
    pub storage_submode: StorageSubmode,
    pub storage_menu_selection: SelectionState,
    pub storage_view_selection: SelectionState,
    pub deposit_selection: SelectionState,
}

impl Default for StoreTabState {
    fn default() -> Self {
        Self {
            mode: StoreMode::Menu,
            menu_selection: SelectionState {
                selected: 0,
                count: STORE_MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            buy_selection: SelectionState::new(0), // Will be updated when buy screen is shown
            sell_selection: SelectionState::new(0), // Will be updated based on player inventory
            storage_submode: StorageSubmode::Menu,
            storage_menu_selection: SelectionState {
                selected: 0,
                count: STORAGE_MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            storage_view_selection: SelectionState::new(0), // Will be updated based on storage contents
            deposit_selection: SelectionState::new(0),      // Will be updated based on player inventory
        }
    }
}

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

/// Item available for purchase in the store.
#[derive(Clone, Copy)]
struct BuyableItem {
    item_id: ItemId,
    name: &'static str,
    price: i32,
    description: &'static str,
}

const BUYABLE_ITEMS: &[BuyableItem] = &[
    BuyableItem {
        item_id: ItemId::BasicHPPotion,
        name: "Health Potion",
        price: 50,
        description: "Restores 50 HP",
    },
    BuyableItem {
        item_id: ItemId::Sword,
        name: "Sword",
        price: 100,
        description: "A basic sword (+10 ATK)",
    },
    BuyableItem {
        item_id: ItemId::BasicShield,
        name: "Basic Shield",
        price: 80,
        description: "Basic protection (+4 DEF)",
    },
    BuyableItem {
        item_id: ItemId::CopperHelmet,
        name: "Copper Helmet",
        price: 200,
        description: "Copper armor (+36 DEF)",
    },
];

/// Handle input for the Store tab.
fn handle_store_input(
    mut store_state: ResMut<StoreTabState>,
    mut action_events: EventReader<GameAction>,
    mut player: ResMut<PlayerResource>,
    mut storage: ResMut<StorageResource>,
) {
    for action in action_events.read() {
        match store_state.mode {
            StoreMode::Menu => handle_menu_input(&mut store_state, action),
            StoreMode::Buy => handle_buy_input(&mut store_state, &mut player, action),
            StoreMode::Sell => handle_sell_input(&mut store_state, &mut player, action),
            StoreMode::Storage => handle_storage_input(&mut store_state, action, &mut player, &mut storage),
        }
    }
}

/// Handle input for the main menu.
fn handle_menu_input(store_state: &mut StoreTabState, action: &GameAction) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.menu_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.menu_selection.move_down();
        }
        GameAction::Select => {
            match store_state.menu_selection.selected {
                0 => {
                    store_state.mode = StoreMode::Buy;
                    store_state.buy_selection.set_count(BUYABLE_ITEMS.len());
                }
                1 => {
                    store_state.mode = StoreMode::Sell;
                    // sell_selection count will be updated in render
                }
                2 => {
                    store_state.mode = StoreMode::Storage;
                    // storage_selection count will be updated in render
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// Handle input for the buy screen.
fn handle_buy_input(
    store_state: &mut StoreTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.buy_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.buy_selection.move_down();
        }
        GameAction::Select => {
            if let Some(buyable) = BUYABLE_ITEMS.get(store_state.buy_selection.selected) {
                if player.gold >= buyable.price {
                    let new_item = buyable.item_id.spawn();
                    if player.add_to_inv(new_item).is_ok() {
                        player.gold -= buyable.price;
                        info!("Purchased {} for {} gold", buyable.name, buyable.price);
                    } else {
                        info!("Inventory full!");
                    }
                } else {
                    info!("Not enough gold! Need {} but have {}", buyable.price, player.gold);
                }
            }
        }
        GameAction::Back => {
            store_state.mode = StoreMode::Menu;
            store_state.buy_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for the sell screen.
fn handle_sell_input(
    store_state: &mut StoreTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    // Update selection count based on current inventory
    store_state.sell_selection.set_count(player.inventory.items.len());

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.sell_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.sell_selection.move_down();
        }
        GameAction::Select => {
            if let Some(inv_item) = player.inventory.items.get(store_state.sell_selection.selected).cloned() {
                let sell_price = (inv_item.item.gold_value as f32 * 0.5) as i32;
                let item_name = inv_item.item.name.clone();

                // Add gold and remove item
                player.gold += sell_price;
                player.decrease_item_quantity(&inv_item, 1);
                info!("Sold {} for {} gold", item_name, sell_price);

                // Update selection if we removed the last item
                let new_count = player.inventory.items.len();
                store_state.sell_selection.set_count(new_count);
                if store_state.sell_selection.selected >= new_count && new_count > 0 {
                    store_state.sell_selection.selected = new_count - 1;
                }
            }
        }
        GameAction::Back => {
            store_state.mode = StoreMode::Menu;
            store_state.sell_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for the storage screen.
fn handle_storage_input(
    store_state: &mut StoreTabState,
    action: &GameAction,
    player: &mut PlayerResource,
    storage: &mut StorageResource,
) {
    match store_state.storage_submode {
        StorageSubmode::Menu => handle_storage_menu_input(store_state, action),
        StorageSubmode::View => handle_storage_view_input(store_state, action, player, storage),
        StorageSubmode::Deposit => handle_storage_deposit_input(store_state, action, player, storage),
    }
}

/// Handle input for the storage menu.
fn handle_storage_menu_input(store_state: &mut StoreTabState, action: &GameAction) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.storage_menu_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.storage_menu_selection.move_down();
        }
        GameAction::Select => {
            match store_state.storage_menu_selection.selected {
                0 => {
                    // View Storage
                    store_state.storage_submode = StorageSubmode::View;
                }
                1 => {
                    // Deposit Items
                    store_state.storage_submode = StorageSubmode::Deposit;
                }
                _ => {}
            }
        }
        GameAction::Back => {
            store_state.mode = StoreMode::Menu;
            store_state.storage_submode = StorageSubmode::Menu;
            store_state.storage_menu_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for viewing/withdrawing storage items.
fn handle_storage_view_input(
    store_state: &mut StoreTabState,
    action: &GameAction,
    player: &mut PlayerResource,
    storage: &mut StorageResource,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.storage_view_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.storage_view_selection.move_down();
        }
        GameAction::Select => {
            // Withdraw item from storage
            let storage_items = storage.inventory.items.as_slice();
            if let Some(inv_item) = storage_items.get(store_state.storage_view_selection.selected) {
                let item = inv_item.item.clone();

                // Try to add to player inventory
                match player.add_to_inv(item.clone()) {
                    Ok(_) => {
                        // Remove from storage
                        let item_uuid = inv_item.uuid();
                        storage.remove_item(item_uuid);
                        info!("Withdrew {} from storage", item.name);
                    }
                    Err(_) => {
                        info!("Inventory is full! Cannot withdraw item.");
                    }
                }
            }
        }
        GameAction::Back => {
            store_state.storage_submode = StorageSubmode::Menu;
            store_state.storage_view_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for depositing items into storage.
fn handle_storage_deposit_input(
    store_state: &mut StoreTabState,
    action: &GameAction,
    player: &mut PlayerResource,
    storage: &mut StorageResource,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_state.deposit_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_state.deposit_selection.move_down();
        }
        GameAction::Select => {
            // Deposit item into storage
            let inventory_items = player.inventory.items.as_slice();
            if let Some(inv_item) = inventory_items.get(store_state.deposit_selection.selected) {
                let item = inv_item.item.clone();

                // Add to storage (storage has unlimited capacity)
                match storage.add_to_inv(item.clone()) {
                    Ok(_) => {
                        // Remove from player inventory
                        let item_uuid = inv_item.uuid();
                        player.remove_item(item_uuid);
                        info!("Deposited {} into storage", item.name);
                    }
                    Err(e) => {
                        info!("Failed to deposit item: {:?}", e);
                    }
                }
            }
        }
        GameAction::Back => {
            store_state.storage_submode = StorageSubmode::Menu;
            store_state.deposit_selection.reset();
        }
        _ => {}
    }
}

/// Render store content when tab is changed to Store.
fn render_store_on_tab_change(
    mut commands: Commands,
    current_tab: Res<CurrentTab>,
    store_state: Res<StoreTabState>,
    player: Res<PlayerResource>,
    storage: Res<StorageResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    if current_tab.tab != TownTab::Store {
        return;
    }

    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_store_ui(&mut commands, content_entity, &store_state, &player, &storage);
}

/// Render store content when state changes.
fn render_store_content(
    mut commands: Commands,
    store_state: Res<StoreTabState>,
    player: Res<PlayerResource>,
    storage: Res<StorageResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_store_ui(&mut commands, content_entity, &store_state, &player, &storage);
}

/// Spawn the store UI based on current mode.
fn spawn_store_ui(
    commands: &mut Commands,
    content_entity: Entity,
    store_state: &StoreTabState,
    player: &PlayerResource,
    storage: &StorageResource,
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
            .with_children(|content| {
                match store_state.mode {
                    StoreMode::Menu => spawn_menu_ui(content, store_state, player),
                    StoreMode::Buy => spawn_buy_ui(content, store_state, player),
                    StoreMode::Sell => spawn_sell_ui(content, store_state, player),
                    StoreMode::Storage => spawn_storage_ui(content, store_state, player, storage),
                }
            });
    });
}

/// Spawn the main menu UI.
fn spawn_menu_ui(parent: &mut ChildBuilder, store_state: &StoreTabState, player: &PlayerResource) {
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

    // Gold display
    spawn_gold_display(parent, player);

    // Menu options
    spawn_menu(
        parent,
        STORE_MENU_OPTIONS,
        store_state.menu_selection.selected,
        None,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab");
}

/// Spawn the buy screen UI.
fn spawn_buy_ui(parent: &mut ChildBuilder, store_state: &StoreTabState, player: &PlayerResource) {
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

    // Gold display
    spawn_gold_display(parent, player);

    // Items for sale
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
        ))
        .with_children(|list| {
            for (i, item) in BUYABLE_ITEMS.iter().enumerate() {
                let is_selected = i == store_state.buy_selection.selected;
                let can_afford = player.gold >= item.price;

                let (bg_color, text_color) = if is_selected {
                    (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                } else {
                    (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                };

                let prefix = if is_selected { "> " } else { "  " };

                let price_color = if can_afford {
                    Color::srgb(0.9, 0.8, 0.3)
                } else {
                    Color::srgb(0.8, 0.3, 0.3)
                };

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
                        Text::new(format!("{}{}", prefix, item.name)),
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

                    // Price
                    item_row.spawn((
                        Text::new(format!("{} gold", item.price)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(price_color),
                        Node {
                            width: Val::Px(100.0),
                            ..default()
                        },
                    ));

                    // Description
                    item_row.spawn((
                        Text::new(item.description),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
            }
        });

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Buy  [Backspace] Back");
}

/// Spawn the sell screen UI.
fn spawn_sell_ui(parent: &mut ChildBuilder, store_state: &StoreTabState, player: &PlayerResource) {
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

    // Gold display
    spawn_gold_display(parent, player);

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
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, inv_item) in inventory_items.iter().enumerate() {
                    let is_selected = i == store_state.sell_selection.selected;

                    let (bg_color, text_color) = if is_selected {
                        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                    } else {
                        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                    };

                    let prefix = if is_selected { "> " } else { "  " };
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

/// Spawn the storage screen UI.
fn spawn_storage_ui(
    parent: &mut ChildBuilder,
    store_state: &StoreTabState,
    player: &PlayerResource,
    storage: &StorageResource,
) {
    match store_state.storage_submode {
        StorageSubmode::Menu => spawn_storage_menu_ui(parent, store_state, player),
        StorageSubmode::View => spawn_storage_view_ui(parent, store_state, player, storage),
        StorageSubmode::Deposit => spawn_storage_deposit_ui(parent, store_state, player),
    }
}

/// Spawn the storage menu UI.
fn spawn_storage_menu_ui(
    parent: &mut ChildBuilder,
    store_state: &StoreTabState,
    player: &PlayerResource,
) {
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

    // Gold display
    spawn_gold_display(parent, player);

    // Menu options
    spawn_menu(
        parent,
        STORAGE_MENU_OPTIONS,
        store_state.storage_menu_selection.selected,
        None,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Select  [Backspace] Back");
}

/// Spawn the storage view/withdraw UI.
fn spawn_storage_view_ui(
    parent: &mut ChildBuilder,
    store_state: &StoreTabState,
    player: &PlayerResource,
    storage: &StorageResource,
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

    // Gold display
    spawn_gold_display(parent, player);

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
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, inv_item) in storage_items.iter().enumerate() {
                    let is_selected = i == store_state.storage_view_selection.selected;

                    let (bg_color, text_color) = if is_selected {
                        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                    } else {
                        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                    };

                    let prefix = if is_selected { "> " } else { "  " };

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
    store_state: &StoreTabState,
    player: &PlayerResource,
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

    // Gold display
    spawn_gold_display(parent, player);

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
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, inv_item) in inventory_items.iter().enumerate() {
                    let is_selected = i == store_state.deposit_selection.selected;

                    let (bg_color, text_color) = if is_selected {
                        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                    } else {
                        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                    };

                    let prefix = if is_selected { "> " } else { "  " };

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

/// Spawn gold display widget.
fn spawn_gold_display(parent: &mut ChildBuilder, player: &PlayerResource) {
    parent
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_children(|gold| {
            gold.spawn((
                Text::new(format!("Gold: {}", player.gold)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.3)),
            ));
        });
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
