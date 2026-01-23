//! Results modal plugin.

use bevy::prelude::*;

use crate::ui::SpriteMarkerAppExt;

use super::input::handle_results_modal_close;
use super::render::spawn_results_modal;
use super::state::{ResultsModalMobSprite, SpawnResultsModal};

/// Plugin for the results modal that displays loot/rewards.
pub struct ResultsModalPlugin;

impl Plugin for ResultsModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_results_modal.run_if(resource_exists::<SpawnResultsModal>),
                handle_results_modal_close,
            ),
        )
        .register_sprite_marker::<ResultsModalMobSprite>();
    }
}
