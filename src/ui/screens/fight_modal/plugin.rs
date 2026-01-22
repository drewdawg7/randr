//! Fight modal plugin.

use bevy::prelude::*;

use crate::ui::screens::health_bar::{init_sprite_health_bars, update_sprite_health_bar_visuals};
use crate::ui::SpriteMarkerAppExt;

use super::input::{handle_fight_modal_close, handle_fight_modal_navigation, handle_fight_modal_select};
use super::render::{spawn_fight_modal, update_button_sprites, update_mob_health_bar, update_player_health_bar};
use super::state::{
    FightModalButtonSelection, FightModalMobSprite, FightModalPlayerSprite, SpawnFightModal,
};

/// Plugin for the fight modal that appears when colliding with mobs.
pub struct FightModalPlugin;

impl Plugin for FightModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fight_modal.run_if(resource_exists::<SpawnFightModal>),
                handle_fight_modal_close,
                (
                    handle_fight_modal_navigation,
                    handle_fight_modal_select,
                    update_button_sprites,
                    update_mob_health_bar,
                    update_player_health_bar,
                    init_sprite_health_bars,
                    update_sprite_health_bar_visuals,
                )
                    .run_if(resource_exists::<FightModalButtonSelection>),
            ),
        )
        .register_sprite_marker::<FightModalPlayerSprite>()
        .register_sprite_marker::<FightModalMobSprite>();
    }
}
