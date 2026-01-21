use bevy::prelude::*;

use crate::states::AppState;

use super::state::TownSystemSet;
use super::systems::{cleanup_town_ui, handle_back_action, setup_town_ui};
pub use super::tabs::TabsPlugin;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TabsPlugin)
            // Configure SystemSet ordering: Input runs before UI
            .configure_sets(
                Update,
                (TownSystemSet::Input, TownSystemSet::Ui)
                    .chain()
                    .run_if(in_state(AppState::Town)),
            )
            .add_systems(OnEnter(AppState::Town), setup_town_ui)
            .add_systems(OnExit(AppState::Town), cleanup_town_ui)
            // Input systems
            .add_systems(Update, handle_back_action.in_set(TownSystemSet::Input));
    }
}
