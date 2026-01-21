//! Fight modal plugin.

use bevy::prelude::*;

use crate::ui::SpriteMarkerAppExt;

use super::input::handle_fight_modal_close;
use super::render::spawn_fight_modal;
use super::state::{FightModalMobSprite, FightModalPlayerSprite, SpawnFightModal};

/// Plugin for the fight modal that appears when colliding with mobs.
pub struct FightModalPlugin;

impl Plugin for FightModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fight_modal.run_if(resource_exists::<SpawnFightModal>),
                handle_fight_modal_close,
            ),
        )
        .register_sprite_marker::<FightModalPlayerSprite>()
        .register_sprite_marker::<FightModalMobSprite>();
    }
}
