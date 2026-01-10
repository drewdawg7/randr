use bevy::prelude::*;

use super::{TabContent, TabHeaderItem, TownTab};

/// Updates the visual appearance of tab headers based on the current tab state.
pub fn update_tab_header_visuals(
    current_tab: Res<State<TownTab>>,
    mut tab_query: Query<(&TabHeaderItem, &mut BackgroundColor)>,
) {
    for (tab_item, mut bg_color) in tab_query.iter_mut() {
        if tab_item.tab == *current_tab.get() {
            *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
        }
    }
}

/// Cleans up tab content when exiting a tab. Used as OnExit system for each TownTab.
pub fn cleanup_tab_content(mut commands: Commands, tab_content_query: Query<Entity, With<TabContent>>) {
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }
}
