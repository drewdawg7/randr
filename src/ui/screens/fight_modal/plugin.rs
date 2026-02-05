use bevy::prelude::*;

use crate::combat::PlayerAttackMob;
use crate::stats::StatSheet;
use crate::ui::modal_registry::RegisterModalExt;
use crate::ui::screens::health_bar::{init_sprite_health_bars, update_sprite_health_bar_visuals};
use crate::ui::screens::modal::in_fight_modal;
use crate::ui::SpriteMarkerAppExt;

use super::input::{
    handle_combat_outcome, handle_fight_modal_close, handle_fight_modal_navigation,
    handle_fight_modal_select, trigger_attack_animation,
};
use super::render::{update_button_sprites, update_mob_health_bar, update_player_health_bar};
use super::state::{
    FightModal, FightModalButtonSelection, FightModalMobSprite, FightModalPlayerSprite,
};

pub struct FightModalPlugin;

impl Plugin for FightModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<FightModal>()
            .add_systems(
                Update,
                (
                    handle_fight_modal_close.run_if(in_fight_modal),
                    (
                        handle_fight_modal_navigation,
                        handle_fight_modal_select,
                        update_button_sprites.run_if(
                            resource_exists::<FightModalButtonSelection>
                                .and(resource_changed::<FightModalButtonSelection>),
                        ),
                        update_mob_health_bar,
                        update_player_health_bar.run_if(resource_changed::<StatSheet>),
                        init_sprite_health_bars,
                        update_sprite_health_bar_visuals,
                    )
                        .run_if(in_fight_modal),
                    handle_combat_outcome.run_if(in_fight_modal),
                    trigger_attack_animation.run_if(on_message::<PlayerAttackMob>),
                ),
            )
            .register_sprite_marker::<FightModalPlayerSprite>()
            .register_sprite_marker::<FightModalMobSprite>();
    }
}
