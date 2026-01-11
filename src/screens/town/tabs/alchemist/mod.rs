//! Alchemist tab for brewing potions.

mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::inventory::Inventory;

use super::super::shared::{update_menu_selection, MenuOption, MenuOptionItem, MenuOptionText};
use super::super::{ContentArea, TabContent, TownTab};
use crate::ui::update_list_selection;

pub use state::AlchemistMode;

use crate::ui::widgets::AlchemistRecipeItem;
use input::handle_alchemist_input;
use render::{spawn_alchemist_ui, AlchemistRecipeItemText};
use state::{AlchemistModeKind, AlchemistSelections};

/// Menu options for the alchemist.
const ALCHEMIST_MENU_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Brew",
    description: Some("Brew potions from recipes"),
}];

/// Plugin for the Alchemist tab.
pub struct AlchemistTabPlugin;

impl Plugin for AlchemistTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlchemistMode>()
            .init_resource::<AlchemistSelections>()
            .add_systems(OnEnter(TownTab::Alchemist), spawn_alchemist_content)
            .add_systems(
                Update,
                (
                    handle_alchemist_input,
                    // Only respawn on mode changes (Menu -> Brew, etc.)
                    refresh_alchemist_on_mode_change.run_if(resource_changed::<AlchemistMode>),
                    // Reactive selection updates within each mode
                    update_alchemist_selection.run_if(resource_changed::<AlchemistSelections>),
                )
                    .run_if(in_state(TownTab::Alchemist)),
            );
    }
}

/// Spawns alchemist UI content when entering the Alchemist tab.
fn spawn_alchemist_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    alchemist_mode: Res<AlchemistMode>,
    alchemist_selections: Res<AlchemistSelections>,
    inventory: Res<Inventory>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_alchemist_ui(
        &mut commands,
        content_entity,
        &alchemist_mode,
        &alchemist_selections,
        &inventory,
    );
}

/// Refreshes alchemist UI when mode changes (Menu -> Brew, etc.).
fn refresh_alchemist_on_mode_change(
    mut commands: Commands,
    alchemist_mode: Res<AlchemistMode>,
    alchemist_selections: Res<AlchemistSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    inventory: Res<Inventory>,
) {
    // Despawn existing content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Respawn with new state
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_alchemist_ui(
        &mut commands,
        content_entity,
        &alchemist_mode,
        &alchemist_selections,
        &inventory,
    );
}

/// Updates alchemist selection highlighting reactively.
fn update_alchemist_selection(
    alchemist_mode: Res<AlchemistMode>,
    alchemist_selections: Res<AlchemistSelections>,
    // Menu mode uses shared menu components
    mut menu_query: Query<(&MenuOptionItem, &mut BackgroundColor, &Children)>,
    mut menu_text_query: Query<(&mut Text, &mut TextColor), With<MenuOptionText>>,
    // Brew mode uses alchemist recipe components
    mut recipe_query: Query<
        (&AlchemistRecipeItem, &mut BackgroundColor, &Children),
        Without<MenuOptionItem>,
    >,
    mut recipe_text_query: Query<
        (&mut Text, &mut TextColor),
        (With<AlchemistRecipeItemText>, Without<MenuOptionText>),
    >,
) {
    match alchemist_mode.mode {
        AlchemistModeKind::Menu => {
            update_menu_selection(
                alchemist_selections.menu.selected,
                &mut menu_query,
                &mut menu_text_query,
            );
        }
        AlchemistModeKind::Brew => {
            update_list_selection(
                alchemist_selections.recipe.selected,
                &mut recipe_query,
                &mut recipe_text_query,
            );
        }
    }
}
