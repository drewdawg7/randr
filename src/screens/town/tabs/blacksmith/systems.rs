use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::player::PlayerGold;
use crate::screens::town::shared::{update_menu_selection, MenuOptionItem, MenuOptionText};
use crate::screens::town::{ContentArea, TabContent};
use crate::ui::update_list_selection;
use crate::ui::widgets::BlacksmithListItem;

use super::render::{spawn_blacksmith_ui, BlacksmithListItemText};
use super::state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Spawns blacksmith UI content when entering the Blacksmith tab.
pub fn spawn_blacksmith_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    gold: Res<PlayerGold>,
    inventory: Res<Inventory>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_blacksmith_ui(
        &mut commands,
        content_entity,
        &blacksmith_mode,
        &blacksmith_selections,
        gold.0,
        &inventory,
    );
}

/// Refreshes blacksmith UI when mode changes (Menu -> Upgrade, etc.).
pub fn refresh_blacksmith_on_mode_change(
    mut commands: Commands,
    blacksmith_mode: Res<BlacksmithMode>,
    blacksmith_selections: Res<BlacksmithSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    gold: Res<PlayerGold>,
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
    spawn_blacksmith_ui(
        &mut commands,
        content_entity,
        &blacksmith_mode,
        &blacksmith_selections,
        gold.0,
        &inventory,
    );
}

/// Updates blacksmith selection highlighting reactively.
pub fn update_blacksmith_selection(
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
            update_list_selection(
                blacksmith_selections.upgrade.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Quality => {
            update_list_selection(
                blacksmith_selections.quality.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Smelt => {
            update_list_selection(
                blacksmith_selections.smelt.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
        BlacksmithModeKind::Forge => {
            update_list_selection(
                blacksmith_selections.forge.selected,
                &mut list_query,
                &mut list_text_query,
            );
        }
    }
}
