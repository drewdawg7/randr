mod constants;
mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::game::Player;
use crate::screens::town::shared::{
    update_menu_selection, MenuOptionItem, MenuOptionText,
};
use crate::screens::town::{ContentArea, TabContent, TownTab};

use input::handle_blacksmith_input;
use render::{update_blacksmith_list_selection, BlacksmithListItem, BlacksmithListItemText};

pub use render::spawn_blacksmith_ui;
pub use state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Plugin for the Blacksmith tab.
pub struct BlacksmithTabPlugin;

impl Plugin for BlacksmithTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlacksmithMode>()
            .init_resource::<BlacksmithSelections>()
            .add_systems(OnEnter(TownTab::Blacksmith), spawn_blacksmith_content)
            .add_systems(
                Update,
                (
                    handle_blacksmith_input,
                    // Only respawn on mode changes (Menu -> Upgrade, etc.)
                    refresh_blacksmith_on_mode_change.run_if(resource_changed::<BlacksmithMode>),
                    // Reactive selection updates within each mode
                    update_blacksmith_selection.run_if(resource_changed::<BlacksmithSelections>),
                )
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

/// Refreshes blacksmith UI when mode changes (Menu -> Upgrade, etc.).
fn refresh_blacksmith_on_mode_change(
    mut commands: Commands,
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    player: Res<Player>,
) {
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

/// Updates blacksmith selection highlighting reactively.
fn update_blacksmith_selection(
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    // Menu mode uses shared menu components
    mut menu_query: Query<(&MenuOptionItem, &mut BackgroundColor, &Children)>,
    mut menu_text_query: Query<(&mut Text, &mut TextColor), With<MenuOptionText>>,
    // Other modes use blacksmith list components
    mut list_query: Query<
        (&BlacksmithListItem, &mut BackgroundColor, &Children),
        Without<MenuOptionItem>,
    >,
    mut list_text_query: Query<
        (&mut Text, &mut TextColor),
        (With<BlacksmithListItemText>, Without<MenuOptionText>),
    >,
) {
    match blacksmith_mode.mode {
        BlacksmithModeKind::Menu => {
            update_menu_selection(
                blacksmith_selections.menu.selected,
                &mut menu_query,
                &mut menu_text_query,
            );
        }
        BlacksmithModeKind::Upgrade => {
            update_blacksmith_list_selection(
                blacksmith_selections.upgrade.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Quality => {
            update_blacksmith_list_selection(
                blacksmith_selections.quality.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Smelt => {
            update_blacksmith_list_selection(
                blacksmith_selections.smelt.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Forge => {
            update_blacksmith_list_selection(
                blacksmith_selections.forge.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
    }
}
