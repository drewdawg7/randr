//! Plugin for the Alchemist tab.

use bevy::prelude::*;

use super::super::super::TownTab;
use super::state::{AlchemistMode, AlchemistSelections};
use super::systems::{
    refresh_alchemist_on_mode_change, spawn_alchemist_content, update_alchemist_selection,
};

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
                    super::input::handle_alchemist_input,
                    // Only respawn on mode changes (Menu -> Brew, etc.)
                    refresh_alchemist_on_mode_change.run_if(resource_changed::<AlchemistMode>),
                    // Reactive selection updates within each mode
                    update_alchemist_selection.run_if(resource_changed::<AlchemistSelections>),
                )
                    .run_if(in_state(TownTab::Alchemist)),
            );
    }
}
