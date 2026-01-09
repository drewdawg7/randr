mod constants;
mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::assets::GameSprites;
use crate::game::{Player, Storage};
use crate::screens::town::{ContentArea, TabContent, TownTab};

use input::handle_store_input;

pub use render::spawn_store_ui;
pub use state::{StoreMode, StoreSelections};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .add_systems(OnEnter(TownTab::Store), spawn_store_content)
            .add_systems(
                Update,
                (handle_store_input, refresh_store_on_mode_change)
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
    player: Res<Player>,
    storage: Res<Storage>,
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
        &player,
        &storage,
        &game_sprites,
    );
}

/// Refreshes store UI when mode or selections change.
fn refresh_store_on_mode_change(
    mut commands: Commands,
    store_mode: Res<StoreMode>,
    store_selections: Res<StoreSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    player: Res<Player>,
    storage: Res<Storage>,
    game_sprites: Res<GameSprites>,
) {
    if !store_mode.is_changed() && !store_selections.is_changed() {
        return;
    }

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
        &player,
        &storage,
        &game_sprites,
    );
}
