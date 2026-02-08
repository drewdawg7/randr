use bevy::prelude::*;

use crate::combat::hitbox::{AttackHitboxBundle, Attacking};
use crate::input::GameAction;
use crate::ui::animation::SpriteAnimation;
use crate::ui::{DungeonPlayer, FacingDirection, PlayerSpriteSheet};

pub fn trigger_player_attack(
    mut commands: Commands,
    mut actions: MessageReader<GameAction>,
    mut player: Query<
        (Entity, &Transform, &FacingDirection, &mut SpriteAnimation),
        (With<DungeonPlayer>, Without<Attacking>),
    >,
    sprite_sheet: Res<PlayerSpriteSheet>,
) {
    if !actions.read().any(|a| matches!(a, GameAction::Attack)) {
        return;
    }

    let Ok((entity, transform, facing, mut anim)) = player.single_mut() else {
        return;
    };

    commands.entity(entity).insert(Attacking);
    commands.spawn(AttackHitboxBundle::new(
        entity,
        transform.translation.truncate(),
        *facing,
        sprite_sheet.frame_size.as_vec2(),
    ));

    anim.apply_config(&sprite_sheet.attack_animation);
    anim.timer = Timer::from_seconds(sprite_sheet.attack_animation.frame_duration, TimerMode::Repeating);
}
