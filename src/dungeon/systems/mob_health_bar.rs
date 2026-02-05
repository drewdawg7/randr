use bevy::prelude::*;

use crate::assets::{GameSprites, HealthBarSlice, SpriteSheetKey};
use crate::dungeon::MobEntity;
use crate::mob::components::Health;

const MOB_HEALTH_BAR_OFFSET_Y: f32 = 14.0;

#[derive(Component)]
pub struct MobHealthBar(pub Entity);

#[derive(Component)]
pub struct MobHealthBarSprite;

pub fn spawn_mob_health_bars(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mobs: Query<(Entity, &Health), (With<MobEntity>, Without<MobHealthBar>)>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    for (mob_entity, health) in &mobs {
        let percent = health_percent(health);
        let slice = HealthBarSlice::for_percent(percent);

        let Some(sprite) = sheet.sprite(slice.as_str()) else {
            continue;
        };

        let health_bar = commands
            .spawn((
                MobHealthBarSprite,
                sprite,
                Transform::default(),
            ))
            .id();

        commands.entity(mob_entity).insert(MobHealthBar(health_bar));
    }
}

pub fn update_mob_health_bar_positions(
    mobs: Query<(&Transform, &MobHealthBar), With<MobEntity>>,
    mut health_bars: Query<&mut Transform, (With<MobHealthBarSprite>, Without<MobEntity>)>,
) {
    for (mob_transform, health_bar) in &mobs {
        let Ok(mut bar_transform) = health_bars.get_mut(health_bar.0) else {
            continue;
        };

        bar_transform.translation = mob_transform.translation
            + Vec3::new(0.0, MOB_HEALTH_BAR_OFFSET_Y, 0.1);
    }
}

pub fn update_mob_health_bar_values(
    game_sprites: Res<GameSprites>,
    mobs: Query<(&Health, &MobHealthBar), (With<MobEntity>, Changed<Health>)>,
    mut health_bars: Query<&mut Sprite, With<MobHealthBarSprite>>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    for (health, mob_health_bar) in &mobs {
        let Ok(mut sprite) = health_bars.get_mut(mob_health_bar.0) else {
            continue;
        };

        let percent = health_percent(health);
        let slice = HealthBarSlice::for_percent(percent);

        let Some(index) = sheet.get(slice.as_str()) else {
            continue;
        };

        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index != index {
                atlas.index = index;
            }
        }
    }
}

pub fn cleanup_mob_health_bars(
    mut commands: Commands,
    mut removed: RemovedComponents<MobEntity>,
    health_bars: Query<&MobHealthBar>,
) {
    for entity in removed.read() {
        if let Ok(health_bar) = health_bars.get(entity) {
            commands.entity(health_bar.0).despawn();
        }
    }
}

fn health_percent(health: &Health) -> f32 {
    if health.max > 0 {
        (health.current as f32 / health.max as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    }
}
