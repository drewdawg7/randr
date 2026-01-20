mod input;
pub mod render;
mod state;
mod update_selection;

use bevy::prelude::*;

use crate::screens::town::TownTab;

use input::handle_store_input;
use render::{
    cache_store_ui_sprites, populate_central_detail_panel, populate_store_info_panel,
    refresh_store_ui, spawn_store_content, StoreUiCache,
};
use update_selection::update_store_selection;

pub use render::InfoPanelSource;
pub use state::{StoreMode, StoreSelections};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .init_resource::<StoreUiCache>()
            .add_systems(Update, cache_store_ui_sprites)
            .add_systems(OnEnter(TownTab::Store), spawn_store_content)
            .add_systems(
                Update,
                (
                    handle_store_input,
                    refresh_store_ui,
                    populate_store_info_panel,
                    populate_central_detail_panel,
                    update_store_selection.run_if(resource_changed::<StoreSelections>),
                )
                    .run_if(in_state(TownTab::Store)),
            );
    }
}
