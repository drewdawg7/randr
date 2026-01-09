use bevy::{ecs::system::SystemParam, prelude::*};

use crate::game::{Player, Storage};

use super::tabs::{
    spawn_alchemist_ui, spawn_blacksmith_ui, spawn_dungeon_ui, spawn_field_ui, spawn_store_ui,
    AlchemistMode, AlchemistSelections, BlacksmithMode, BlacksmithSelections, DungeonTabState,
    FieldTabState, StoreMode, StoreSelections,
};
use super::{ContentArea, CurrentTab, TabContent, TabHeaderItem, TownTab};

#[derive(Resource, Default)]
pub struct ForceTabRefresh(pub bool);

#[derive(SystemParam)]
pub struct TabStates<'w> {
    field: Res<'w, FieldTabState>,
    dungeon: Res<'w, DungeonTabState>,
    store_mode: Res<'w, StoreMode>,
    store_selections: Res<'w, StoreSelections>,
    blacksmith_mode: Res<'w, BlacksmithMode>,
    blacksmith_selections: Res<'w, BlacksmithSelections>,
    alchemist_mode: Res<'w, AlchemistMode>,
    alchemist_selections: Res<'w, AlchemistSelections>,
}

pub fn trigger_tab_refresh(mut refresh: ResMut<ForceTabRefresh>) {
    refresh.0 = true;
}

pub fn update_tab_header_visuals(
    current_tab: Res<CurrentTab>,
    mut tab_query: Query<(&TabHeaderItem, &mut BackgroundColor)>,
) {
    if !current_tab.is_changed() {
        return;
    }

    for (tab_item, mut bg_color) in tab_query.iter_mut() {
        if tab_item.tab == current_tab.tab {
            *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
        }
    }
}

pub fn render_tab_content(
    mut commands: Commands,
    current_tab: Res<CurrentTab>,
    mut force_refresh: ResMut<ForceTabRefresh>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    tab_states: TabStates,
    player: Res<Player>,
    storage: Res<Storage>,
) {
    let should_render = force_refresh.0
        || current_tab.is_changed()
        || (current_tab.tab == TownTab::Store
            && (tab_states.store_mode.is_changed() || tab_states.store_selections.is_changed()))
        || (current_tab.tab == TownTab::Blacksmith
            && (tab_states.blacksmith_mode.is_changed()
                || tab_states.blacksmith_selections.is_changed()))
        || (current_tab.tab == TownTab::Alchemist
            && (tab_states.alchemist_mode.is_changed()
                || tab_states.alchemist_selections.is_changed()))
        || (current_tab.tab == TownTab::Field && tab_states.field.is_changed())
        || (current_tab.tab == TownTab::Dungeon && tab_states.dungeon.is_changed());

    if !should_render {
        return;
    }

    force_refresh.0 = false;

    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    match current_tab.tab {
        TownTab::Store => {
            spawn_store_ui(
                &mut commands,
                content_entity,
                &tab_states.store_mode,
                &tab_states.store_selections,
                &player,
                &storage,
            );
        }
        TownTab::Blacksmith => {
            spawn_blacksmith_ui(
                &mut commands,
                content_entity,
                &tab_states.blacksmith_mode,
                &tab_states.blacksmith_selections,
                &player,
            );
        }
        TownTab::Alchemist => {
            spawn_alchemist_ui(
                &mut commands,
                content_entity,
                &tab_states.alchemist_mode,
                &tab_states.alchemist_selections,
                &player,
            );
        }
        TownTab::Field => {
            spawn_field_ui(&mut commands, content_entity, &tab_states.field, &player);
        }
        TownTab::Dungeon => {
            spawn_dungeon_ui(
                &mut commands,
                content_entity,
                &tab_states.dungeon,
                &player,
            );
        }
    }
}
