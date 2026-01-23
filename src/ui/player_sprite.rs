//! Player sprite animation system.
//!
//! Provides animated sprite support for the player character in dungeons.

use bevy::prelude::*;

use super::animation::{AnimationConfig, SpriteAnimation};
use super::sprite_marker::{SpriteData, SpriteMarker, SpriteMarkerAppExt};

/// Plugin for player sprite animations.
pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        use super::animation::{animate_sprites, tick_animation_clock, AnimationClock};

        app.init_resource::<PlayerSpriteSheet>()
            .init_resource::<AnimationClock>()
            .add_systems(PreStartup, load_player_sprite_sheet)
            .add_systems(
                Update,
                (
                    tick_animation_clock,
                    animate_sprites,
                    revert_player_idle,
                    revert_attack_idle,
                )
                    .chain(),
            )
            .register_sprite_marker::<DungeonPlayerSprite>();
    }
}

/// Resource containing the loaded player sprite sheet.
#[derive(Resource, Default)]
pub struct PlayerSpriteSheet {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub animation: AnimationConfig,
    pub walk_animation: AnimationConfig,
    pub attack_animation: AnimationConfig,
}

impl PlayerSpriteSheet {
    /// Check if the sprite sheet is loaded.
    pub fn is_loaded(&self) -> bool {
        self.texture.is_some() && self.layout.is_some()
    }
}

/// Timer component that tracks how long the walk animation should play.
/// When the timer expires, the player reverts to idle animation.
#[derive(Component)]
pub struct PlayerWalkTimer(pub Timer);

/// Timer component that tracks how long the attack animation should play.
/// When the timer expires, the player reverts to idle animation.
#[derive(Component)]
pub struct PlayerAttackTimer(pub Timer);

/// Marker component for dungeon player sprites that need population.
#[derive(Component)]
pub struct DungeonPlayerSprite;

impl SpriteMarker for DungeonPlayerSprite {
    type Resources = Res<'static, PlayerSpriteSheet>;

    fn resolve(&self, sheet: &Res<PlayerSpriteSheet>) -> Option<SpriteData> {
        if !sheet.is_loaded() {
            return None;
        }
        Some(SpriteData {
            texture: sheet.texture.as_ref()?.clone(),
            layout: sheet.layout.as_ref()?.clone(),
            animation: sheet.animation.clone(),
            flip_x: false,
        })
    }
}

/// System to load player sprite sheet at startup.
fn load_player_sprite_sheet(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_sheet: ResMut<PlayerSpriteSheet>,
) {
    // MiniLightningWarrior: 13x8 grid of 32x32, idle is slices 0-3
    let texture: Handle<Image> = asset_server.load("sprites/player/lightning_warrior.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 13, 8, None, None);
    let layout_handle = layouts.add(layout);

    player_sheet.texture = Some(texture);
    player_sheet.layout = Some(layout_handle);
    player_sheet.animation = AnimationConfig {
        first_frame: 0,
        last_frame: 3,
        frame_duration: 0.15,
        looping: true,
        synchronized: true,
    };
    player_sheet.walk_animation = AnimationConfig {
        first_frame: 13,
        last_frame: 18,
        frame_duration: 0.1,
        looping: true,
        synchronized: false,
    };
    player_sheet.attack_animation = AnimationConfig {
        first_frame: 39,
        last_frame: 47,
        frame_duration: 0.06,
        looping: false,
        synchronized: false,
    };

    info!("Loaded player sprite sheet: MiniLightningWarrior");
}

/// Reverts the player sprite to idle animation when the attack timer expires.
fn revert_attack_idle(
    time: Res<Time>,
    sheet: Res<PlayerSpriteSheet>,
    mut query: Query<(&mut PlayerAttackTimer, &mut SpriteAnimation)>,
) {
    for (mut timer, mut anim) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            anim.first_frame = sheet.animation.first_frame;
            anim.last_frame = sheet.animation.last_frame;
            anim.current_frame = sheet.animation.first_frame;
            anim.frame_duration = sheet.animation.frame_duration;
            anim.looping = true;
            anim.synchronized = true;
        }
    }
}

/// Reverts the player sprite to idle animation when the walk timer expires.
fn revert_player_idle(
    time: Res<Time>,
    sheet: Res<PlayerSpriteSheet>,
    mut query: Query<(&mut PlayerWalkTimer, &mut SpriteAnimation)>,
) {
    for (mut timer, mut anim) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            anim.first_frame = sheet.animation.first_frame;
            anim.last_frame = sheet.animation.last_frame;
            anim.current_frame = sheet.animation.first_frame;
            anim.frame_duration = sheet.animation.frame_duration;
            anim.synchronized = true;
        }
    }
}
