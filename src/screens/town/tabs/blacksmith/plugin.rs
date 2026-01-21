use bevy::prelude::*;

use crate::screens::town::TownTab;

use super::input::handle_blacksmith_input;
use super::state::{BlacksmithMode, BlacksmithSelections};
use super::systems::{
    refresh_blacksmith_on_mode_change, spawn_blacksmith_content, update_blacksmith_selection,
};

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
