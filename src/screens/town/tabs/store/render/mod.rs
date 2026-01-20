mod buy;
mod helpers;
mod menu;
mod panels;
mod sell;
mod storage;

use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey, UiAllSlice};
use crate::game::Storage;
use crate::inventory::Inventory;
use crate::location::Store;
use crate::screens::town::{ContentArea, TabContent};

use super::state::{StoreMode, StoreModeKind, StoreSelections};

pub use buy::spawn_buy_ui;
pub use helpers::spawn_inventory_list;
pub use menu::spawn_menu_ui;
pub use panels::{populate_central_detail_panel, populate_store_info_panel};
pub use sell::spawn_sell_ui;
pub use storage::{spawn_storage_deposit_ui, spawn_storage_menu_ui, spawn_storage_view_ui};

/// Cached sprites for store UI, populated once when GameSprites loads.
#[derive(Resource, Default)]
pub struct StoreUiCache {
    pub info_panel_bg: Option<ImageNode>,
}

/// System to populate the store UI cache from GameSprites.
pub fn cache_store_ui_sprites(mut cache: ResMut<StoreUiCache>, game_sprites: Res<GameSprites>) {
    if cache.info_panel_bg.is_some() {
        return;
    }

    cache.info_panel_bg = game_sprites
        .get(SpriteSheetKey::UiAll)
        .and_then(|s| s.image_node_sliced(UiAllSlice::InfoPanelBg.as_str(), 10.0));
}

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

/// Internal helper to spawn store UI based on current mode.
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
