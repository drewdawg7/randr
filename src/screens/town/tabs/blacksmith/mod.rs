mod constants;
mod input;
mod render;
mod state;

use bevy::prelude::*;

use crate::screens::town::{CurrentTab, TownTab};
use crate::states::AppState;

use input::handle_blacksmith_input;

pub use render::spawn_blacksmith_ui;
pub use state::{BlacksmithMode, BlacksmithSelections};

/// Plugin for the Blacksmith tab.
pub struct BlacksmithTabPlugin;

impl Plugin for BlacksmithTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlacksmithMode>()
            .init_resource::<BlacksmithSelections>()
            .add_systems(
                Update,
                handle_blacksmith_input
                    .run_if(in_state(AppState::Town))
                    .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Blacksmith),
            );
    }
}
