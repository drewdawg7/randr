//! Systems for the alchemist tab.

use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::update_list_selection;
use crate::ui::widgets::AlchemistRecipeItem;

use super::super::super::shared::{update_menu_selection, MenuOption, MenuOptionItem, MenuOptionText};
use super::super::super::{ContentArea, TabContent};
use super::render::{spawn_alchemist_ui, AlchemistRecipeItemText};
use super::state::{AlchemistMode, AlchemistModeKind, AlchemistSelections};

/// Menu options for the alchemist.
pub const ALCHEMIST_MENU_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Brew",
    description: Some("Brew potions from recipes"),
}];

/// Spawns alchemist UI content when entering the Alchemist tab.
pub fn spawn_alchemist_content(
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
pub fn refresh_alchemist_on_mode_change(
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
pub fn update_alchemist_selection(
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
