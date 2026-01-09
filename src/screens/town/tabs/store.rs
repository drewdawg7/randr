use bevy::prelude::*;

use crate::game::{Player, Storage};
use crate::ui::{selection_colors, selection_prefix};
use crate::input::{GameAction, NavigationDirection};
use crate::item::ItemId;
use crate::states::AppState;
use crate::{FindsItems, ManagesItems};

use super::super::shared::{spawn_menu, MenuOption, SelectionState};
use super::super::{CurrentTab, TabContent, TownTab};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .add_systems(
                Update,
                handle_store_input
                    .run_if(in_state(AppState::Town))
                    .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Store),
            );
    }
}

/// Store mode kind - what submenu the player is in.
/// Flattened to avoid nested state dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreModeKind {
    #[default]
    Menu,
    Buy,
    Sell,
    StorageMenu,
    StorageView,
    StorageDeposit,
}

/// Store mode - tracks navigation state within the tab.
#[derive(Resource, Default)]
pub struct StoreMode {
    pub mode: StoreModeKind,
}

/// Store selections - tracks cursor positions in each mode.
#[derive(Resource)]
pub struct StoreSelections {
    pub menu: SelectionState,
    pub buy: SelectionState,
    pub sell: SelectionState,
    pub storage_menu: SelectionState,
    pub storage_view: SelectionState,
    pub deposit: SelectionState,
}

impl Default for StoreSelections {
    fn default() -> Self {
        Self {
            menu: SelectionState {
                selected: 0,
                count: STORE_MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            buy: SelectionState::new(0),
            sell: SelectionState::new(0),
            storage_menu: SelectionState {
                selected: 0,
                count: STORAGE_MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            storage_view: SelectionState::new(0),
            deposit: SelectionState::new(0),
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
    mut store_mode: ResMut<StoreMode>,
    mut store_selections: ResMut<StoreSelections>,
    mut action_events: EventReader<GameAction>,
    mut player: ResMut<Player>,
    mut storage: ResMut<Storage>,
) {
    for action in action_events.read() {
        match store_mode.mode {
            StoreModeKind::Menu => handle_menu_input(&mut store_mode, &mut store_selections, action),
            StoreModeKind::Buy => handle_buy_input(&mut store_mode, &mut store_selections, &mut player, action),
            StoreModeKind::Sell => handle_sell_input(&mut store_mode, &mut store_selections, &mut player, action),
            StoreModeKind::StorageMenu => handle_storage_menu_input(&mut store_mode, &mut store_selections, action),
            StoreModeKind::StorageView => handle_storage_view_input(&mut store_mode, &mut store_selections, action, &mut player, &mut storage),
            StoreModeKind::StorageDeposit => handle_storage_deposit_input(&mut store_mode, &mut store_selections, action, &mut player, &mut storage),
        }
    }
}

/// Handle input for the main menu.
fn handle_menu_input(store_mode: &mut StoreMode, store_selections: &mut StoreSelections, action: &GameAction) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.menu.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.menu.move_down();
        }
        GameAction::Select => {
            match store_selections.menu.selected {
                0 => {
                    store_mode.mode = StoreModeKind::Buy;
                    store_selections.buy.set_count(BUYABLE_ITEMS.len());
                }
                1 => {
                    store_mode.mode = StoreModeKind::Sell;
                    // sell count will be updated in render
                }
                2 => {
                    store_mode.mode = StoreModeKind::StorageMenu;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// Handle input for the buy screen.
fn handle_buy_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    player: &mut Player,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.buy.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.buy.move_down();
        }
        GameAction::Select => {
            if let Some(buyable) = BUYABLE_ITEMS.get(store_selections.buy.selected) {
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
            store_mode.mode = StoreModeKind::Menu;
            store_selections.buy.reset();
        }
        _ => {}
    }
}

/// Handle input for the sell screen.
fn handle_sell_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Update selection count based on current inventory
    store_selections.sell.set_count(player.inventory.items.len());

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.sell.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.sell.move_down();
        }
        GameAction::Select => {
            if let Some(inv_item) = player.inventory.items.get(store_selections.sell.selected).cloned() {
                let sell_price = (inv_item.item.gold_value as f32 * 0.5) as i32;
                let item_name = inv_item.item.name.clone();

                // Add gold and remove item
                player.gold += sell_price;
                player.decrease_item_quantity(&inv_item, 1);
                info!("Sold {} for {} gold", item_name, sell_price);

                // Update selection if we removed the last item
                let new_count = player.inventory.items.len();
                store_selections.sell.set_count(new_count);
                if store_selections.sell.selected >= new_count && new_count > 0 {
                    store_selections.sell.selected = new_count - 1;
                }
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.sell.reset();
        }
        _ => {}
    }
}

/// Handle input for the storage menu.
fn handle_storage_menu_input(store_mode: &mut StoreMode, store_selections: &mut StoreSelections, action: &GameAction) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.storage_menu.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.storage_menu.move_down();
        }
        GameAction::Select => {
            match store_selections.storage_menu.selected {
                0 => store_mode.mode = StoreModeKind::StorageView,
                1 => store_mode.mode = StoreModeKind::StorageDeposit,
                _ => {}
            }
        }
        GameAction::Back => {
            store_mode.mode = StoreModeKind::Menu;
            store_selections.storage_menu.reset();
        }
        _ => {}
    }
}

/// Handle input for viewing/withdrawing storage items.
fn handle_storage_view_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    player: &mut Player,
    storage: &mut Storage,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.storage_view.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.storage_view.move_down();
        }
        GameAction::Select => {
            // Withdraw item from storage
            let storage_items = storage.inventory.items.as_slice();
            if let Some(inv_item) = storage_items.get(store_selections.storage_view.selected) {
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
            store_mode.mode = StoreModeKind::StorageMenu;
            store_selections.storage_view.reset();
        }
        _ => {}
    }
}

/// Handle input for depositing items into storage.
fn handle_storage_deposit_input(
    store_mode: &mut StoreMode,
    store_selections: &mut StoreSelections,
    action: &GameAction,
    player: &mut Player,
    storage: &mut Storage,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            store_selections.deposit.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            store_selections.deposit.move_down();
        }
        GameAction::Select => {
            // Deposit item into storage
            let inventory_items = player.inventory.items.as_slice();
            if let Some(inv_item) = inventory_items.get(store_selections.deposit.selected) {
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
            store_mode.mode = StoreModeKind::StorageMenu;
            store_selections.deposit.reset();
        }
        _ => {}
    }
}

/// Spawn the store UI based on current mode.
pub fn spawn_store_ui(
    commands: &mut Commands,
    content_entity: Entity,
    store_mode: &StoreMode,
    store_selections: &StoreSelections,
    player: &Player,
    storage: &Storage,
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
                match store_mode.mode {
                    StoreModeKind::Menu => spawn_menu_ui(content, store_selections, player),
                    StoreModeKind::Buy => spawn_buy_ui(content, store_selections, player),
                    StoreModeKind::Sell => spawn_sell_ui(content, store_selections, player),
                    StoreModeKind::StorageMenu => spawn_storage_menu_ui(content, store_selections, player),
                    StoreModeKind::StorageView => spawn_storage_view_ui(content, store_selections, player, storage),
                    StoreModeKind::StorageDeposit => spawn_storage_deposit_ui(content, store_selections, player),
                }
            });
    });
}

/// Spawn the main menu UI.
fn spawn_menu_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections, player: &Player) {
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
        store_selections.menu.selected,
        None,
    );

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab");
}

/// Spawn the buy screen UI.
fn spawn_buy_ui(parent: &mut ChildBuilder, store_selections: &StoreSelections, player: &Player) {
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
                let is_selected = i == store_selections.buy.selected;
                let can_afford = player.gold >= item.price;

                let (bg_color, text_color) = selection_colors(is_selected);

                let prefix = selection_prefix(is_selected);

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
fn spawn_storage_menu_ui(
    parent: &mut ChildBuilder,
    store_selections: &StoreSelections,
    player: &Player,
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
    player: &Player,
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

/// Spawn gold display widget.
fn spawn_gold_display(parent: &mut ChildBuilder, player: &Player) {
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
