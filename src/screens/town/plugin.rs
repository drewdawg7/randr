use bevy::prelude::*;

use crate::states::AppState;

use super::state::{TownSystemSet, TownTab};
use super::systems::{
    cleanup_tab_content, cleanup_town_ui, handle_back_action, handle_tab_navigation,
    setup_town_ui, update_tab_header_visuals,
};
pub use super::tabs::TabsPlugin;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<TownTab>()
            .add_plugins(TabsPlugin)
            // Configure SystemSet ordering: Input runs before UI
            .configure_sets(
                Update,
                (TownSystemSet::Input, TownSystemSet::Ui)
                    .chain()
                    .run_if(in_state(AppState::Town)),
            )
            .add_systems(OnEnter(AppState::Town), setup_town_ui)
            .add_systems(OnExit(AppState::Town), cleanup_town_ui)
            // Cleanup tab content when exiting any tab
            .add_systems(OnExit(TownTab::Store), cleanup_tab_content)
            // Input systems
            .add_systems(
                Update,
                (handle_tab_navigation, handle_back_action).in_set(TownSystemSet::Input),
            )
            // UI systems
            .add_systems(
                Update,
                update_tab_header_visuals
                    .in_set(TownSystemSet::Ui)
                    .run_if(state_changed::<TownTab>),
            );
    }
}
