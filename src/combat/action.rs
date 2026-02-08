use std::collections::HashSet;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::dungeon::{attack_hitbox_layers, PLAYER_COLLIDER};
use crate::ui::FacingDirection;

const HITBOX_WIDTH: f32 = 12.0;
const HITBOX_HEIGHT: f32 = 36.0;
const HITBOX_DURATION_SECS: f32 = 9.0 / 16.0;

#[derive(Component)]
pub struct Attacking;

#[derive(Component)]
pub struct AttackHitbox(pub Entity);

#[derive(Component, Deref, DerefMut)]
pub struct HitboxLifetime(pub Timer);

impl Default for HitboxLifetime {
    fn default() -> Self {
        Self(Timer::from_seconds(HITBOX_DURATION_SECS, TimerMode::Once))
    }
}

#[derive(Component, Default)]
pub struct HitEntities(pub HashSet<Entity>);

#[derive(Bundle)]
pub struct AttackHitboxBundle {
    hitbox: AttackHitbox,
    lifetime: HitboxLifetime,
    hit_entities: HitEntities,
    collider: Collider,
    sensor: Sensor,
    events: CollisionEventsEnabled,
    layers: CollisionLayers,
    transform: Transform,
}

impl AttackHitboxBundle {
    pub fn new(owner: Entity, position: Vec2, facing: FacingDirection, player_sprite_size: Vec2) -> Self {
        let size = Vec2::new(HITBOX_WIDTH, HITBOX_HEIGHT);
        let player_half_width = player_sprite_size.x * PLAYER_COLLIDER.scale_x / 2.0;
        let hitbox_half_width = HITBOX_WIDTH / 2.0;
        let offset = facing.to_offset(player_half_width + hitbox_half_width)
            + Vec2::Y * PLAYER_COLLIDER.offset_y;

        Self {
            hitbox: AttackHitbox(owner),
            lifetime: HitboxLifetime::default(),
            hit_entities: HitEntities::default(),
            collider: Collider::rectangle(size.x, size.y),
            sensor: Sensor,
            events: CollisionEventsEnabled,
            layers: attack_hitbox_layers(),
            transform: Transform::from_translation((position + offset).extend(0.5)),
        }
    }
}
