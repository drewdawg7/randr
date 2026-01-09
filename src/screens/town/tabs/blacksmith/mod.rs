mod constants;
mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::game::Player;
use crate::screens::town::{ContentArea, TabContent, TownTab};

use input::handle_blacksmith_input;

pub use render::spawn_blacksmith_ui;
pub use state::{BlacksmithMode, BlacksmithSelections};

/// Plugin for the Blacksmith tab.
pub struct BlacksmithTabPlugin;

impl Plugin for BlacksmithTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlacksmithMode>()
            .init_resource::<BlacksmithSelections>()
            .add_systems(OnEnter(TownTab::Blacksmith), spawn_blacksmith_content)
            .add_systems(
                Update,
                (handle_blacksmith_input, refresh_blacksmith_on_mode_change)
                    .run_if(in_state(TownTab::Blacksmith)),
            );
    }
}

/// Spawns blacksmith UI content when entering the Blacksmith tab.
fn spawn_blacksmith_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    player: Res<Player>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_blacksmith_ui(
        &mut commands,
        content_entity,
        &blacksmith_mode,
        &blacksmith_selections,
        &player,
    );
}

/// Refreshes blacksmith UI when mode or selections change.
fn refresh_blacksmith_on_mode_change(
    mut commands: Commands,
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    player: Res<Player>,
) {
    if !blacksmith_mode.is_changed() && !blacksmith_selections.is_changed() {
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
    spawn_blacksmith_ui(
        &mut commands,
        content_entity,
        &blacksmith_mode,
        &blacksmith_selections,
        &player,
    );
}
