use bevy::prelude::*;

use crate::combat::hitbox::{AttackHitbox, Attacking, HitboxLifetime};

pub fn cleanup_expired_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &AttackHitbox, &mut HitboxLifetime)>,
) {
    for (entity, hitbox, mut lifetime) in &mut hitboxes {
        lifetime.tick(time.delta());

        if lifetime.just_finished() {
            commands.entity(entity).despawn();
            commands.entity(hitbox.0).remove::<Attacking>();
        }
    }
}
