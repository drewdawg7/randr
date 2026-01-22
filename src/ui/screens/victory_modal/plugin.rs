//! Victory modal plugin.

use bevy::prelude::*;

use crate::ui::SpriteMarkerAppExt;

use super::input::handle_victory_modal_close;
use super::render::spawn_victory_modal;
use super::state::{SpawnVictoryModal, VictoryModalMobSprite};

/// Plugin for the victory modal that appears after defeating a mob.
pub struct VictoryModalPlugin;

impl Plugin for VictoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_victory_modal.run_if(resource_exists::<SpawnVictoryModal>),
                handle_victory_modal_close,
            ),
        )
        .register_sprite_marker::<VictoryModalMobSprite>();
    }
}
