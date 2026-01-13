mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::assets::GameSprites;
use crate::game::Storage;
use crate::inventory::Inventory;
use crate::location::Store;
use crate::screens::town::shared::{update_menu_selection, MenuOptionItem, MenuOptionText};
use crate::screens::town::{ContentArea, TabContent, TownTab};
use crate::ui::update_list_selection;

use crate::ui::widgets::StoreListItem;
use input::handle_store_input;
use render::{populate_store_info_panel, StoreListItemText};

pub use render::spawn_store_ui;
pub use state::{StoreMode, StoreModeKind, StoreSelections};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .add_systems(OnEnter(TownTab::Store), spawn_store_content)
            .add_systems(
                Update,
                (
                    handle_store_input,
                    // Only respawn on mode changes
                    refresh_store_on_mode_change.run_if(resource_changed::<StoreMode>),
                    // Reactive selection updates within each mode
                    update_store_selection.run_if(resource_changed::<StoreSelections>),
                    populate_store_info_panel,
                )
                    .run_if(in_state(TownTab::Store)),
            );
    }
}

/// Spawns store UI content when entering the Store tab.
fn spawn_store_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    store: Res<Store>,
    game_sprites: Res<GameSprites>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_store_ui(
        &mut commands,
        content_entity,
        &store_mode,
        &store_selections,
        &inventory,
        &storage,
        &store,
        &game_sprites,
    );
}

/// Refreshes store UI when mode changes.
fn refresh_store_on_mode_change(
    mut commands: Commands,
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    inventory: Res<Inventory>,
    storage: Res<Storage>,
    store: Res<Store>,
    game_sprites: Res<GameSprites>,
) {
    // Despawn existing content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Respawn with new state
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_store_ui(
        &mut commands,
        content_entity,
        &store_mode,
        &store_selections,
        &inventory,
        &storage,
        &store,
        &game_sprites,
    );
}

/// Updates store selection highlighting reactively.
fn update_store_selection(
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    // Menu modes use shared menu components
    mut menu_query: Query<(&MenuOptionItem, &mut BackgroundColor, &Children)>,
    mut menu_text_query: Query<(&mut Text, &mut TextColor), With<MenuOptionText>>,
    // Inventory modes use store list components
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
            // No reactive selection update needed here
        }
    }
}
