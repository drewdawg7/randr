use bevy::prelude::*;

use crate::screens::town::TownTab;

use super::input::handle_store_input;
use super::render::{populate_central_detail_panel, refresh_store_ui, spawn_store_content};
use super::state::{StoreMode, StoreSelections};
use super::update_selection::update_store_selection;

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .add_systems(OnEnter(TownTab::Store), spawn_store_content)
            .add_systems(
                Update,
                (
                    handle_store_input,
                    refresh_store_ui,
                    populate_central_detail_panel,
                    update_store_selection.run_if(resource_changed::<StoreSelections>),
                )
                    .run_if(in_state(TownTab::Store)),
            );
    }
}
