//! Results modal plugin.

use bevy::prelude::*;

use crate::ui::modal_registry::RegisterModalExt;
use crate::ui::screens::modal::in_results_modal;
use crate::ui::SpriteMarkerAppExt;

use super::input::handle_results_modal_close;
use super::state::{ResultsModal, ResultsModalMobSprite};

/// Plugin for the results modal that displays loot/rewards.
pub struct ResultsModalPlugin;

impl Plugin for ResultsModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<ResultsModal>()
            .add_systems(Update, handle_results_modal_close.run_if(in_results_modal))
            .register_sprite_marker::<ResultsModalMobSprite>();
    }
}
