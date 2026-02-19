use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::combat::hitbox::{AttackHitboxBundle, Attacking};
use crate::input::GameAction;
use crate::ui::player_sprite::{PLAYER_ATTACK_TAG, PLAYER_IDLE_TAG};
use crate::ui::{DungeonPlayer, FacingDirection, PlayerSpriteSheet};

pub fn trigger_player_attack(
    mut commands: Commands,
    mut actions: MessageReader<GameAction>,
    mut player: Query<
        (Entity, &Transform, &FacingDirection, &mut AseAnimation),
        (With<DungeonPlayer>, Without<Attacking>),
    >,
    sprite_sheet: Res<PlayerSpriteSheet>,
) {
    if !actions.read().any(|a| matches!(a, GameAction::Attack)) {
        return;
    }

    let Ok((entity, transform, facing, mut ase_anim)) = player.single_mut() else {
        return;
    };

    commands.entity(entity).insert(Attacking);
    commands.spawn(AttackHitboxBundle::new(
        entity,
        transform.translation.truncate(),
        *facing,
        sprite_sheet.frame_size.as_vec2(),
    ));

    ase_anim.animation = Animation::tag(PLAYER_ATTACK_TAG)
        .with_repeat(AnimationRepeat::Count(1))
        .with_then(PLAYER_IDLE_TAG, AnimationRepeat::Loop);
}
