mod constants;
mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::screens::town::{CurrentTab, TownTab};
use crate::states::AppState;

use input::handle_store_input;

pub use render::spawn_store_ui;
pub use state::{StoreMode, StoreSelections};

/// Plugin for the Store tab.
pub struct StoreTabPlugin;

impl Plugin for StoreTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoreMode>()
            .init_resource::<StoreSelections>()
            .add_systems(
                Update,
                handle_store_input
                    .run_if(in_state(AppState::Town))
                    .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Store),
            );
    }
}
