//! Fight modal plugin.

use bevy::prelude::*;

use super::input::handle_fight_modal_close;
use super::render::{
    populate_fight_modal_mob_sprite, populate_fight_modal_player_sprite, spawn_fight_modal,
};
use super::state::SpawnFightModal;

/// Plugin for the fight modal that appears when colliding with mobs.
pub struct FightModalPlugin;

impl Plugin for FightModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fight_modal.run_if(resource_exists::<SpawnFightModal>),
                populate_fight_modal_player_sprite,
                populate_fight_modal_mob_sprite,
                handle_fight_modal_close,
            ),
        );
    }
}
