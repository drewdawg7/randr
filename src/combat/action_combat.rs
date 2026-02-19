use avian2d::prelude::*;
use bevy::prelude::*;

use crate::input::trigger_player_attack;
use crate::states::AppState;
use crate::ui::screens::ActiveModal;

use super::events::{DamageEntity, EntityDied, GoldGained, LootDropped, XpGained};
use super::systems::{damage, death_rewards, hitbox_cleanup, hitbox_collision, rewards};

fn no_modal(modal: Res<ActiveModal>) -> bool {
    modal.modal.is_none()
}

pub struct ActionCombatPlugin;

impl Plugin for ActionCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEntity>()
            .add_message::<GoldGained>()
            .add_message::<XpGained>()
            .add_message::<LootDropped>()
            .add_systems(
                Update,
                (
                    trigger_player_attack.run_if(no_modal),
                    hitbox_collision::handle_hitbox_collisions.run_if(on_message::<CollisionStart>),
                    damage::apply_damage.run_if(on_message::<DamageEntity>),
                    (
                        death_rewards::grant_kill_gold,
                        death_rewards::grant_kill_xp,
                        death_rewards::roll_kill_loot,
                    )
                        .run_if(on_message::<EntityDied>),
                    death_rewards::mark_death_processed.run_if(on_message::<EntityDied>),
                    death_rewards::mark_mob_dying.run_if(on_message::<EntityDied>),
                    (
                        rewards::apply_gold_gain.run_if(on_message::<GoldGained>),
                        rewards::apply_xp_gain.run_if(on_message::<XpGained>),
                    ),
                    hitbox_cleanup::cleanup_expired_hitboxes,
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}
