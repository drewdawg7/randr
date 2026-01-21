//! Player sprite animation system.
//!
//! Provides animated sprite support for the player character in dungeons.

use bevy::prelude::*;

use super::animation::AnimationConfig;
use super::sprite_marker::{SpriteData, SpriteMarker, SpriteMarkerAppExt};

/// Plugin for player sprite animations.
pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        use super::animation::animate_sprites;

        app.init_resource::<PlayerSpriteSheet>()
            .add_systems(PreStartup, load_player_sprite_sheet)
            .add_systems(Update, animate_sprites)
            .register_sprite_marker::<DungeonPlayerSprite>();
    }
}

/// Animation configuration for the player's idle animation.
#[derive(Debug, Clone)]
pub struct PlayerAnimationConfig {
    /// First frame index of the idle animation
    pub first_frame: usize,
    /// Last frame index of the idle animation (inclusive)
    pub last_frame: usize,
    /// Duration per frame in seconds
    pub frame_duration: f32,
}

impl Default for PlayerAnimationConfig {
    fn default() -> Self {
        Self {
            first_frame: 0,
            last_frame: 3,
            frame_duration: 0.15,
        }
    }
}

impl From<PlayerAnimationConfig> for AnimationConfig {
    fn from(config: PlayerAnimationConfig) -> Self {
        Self {
            first_frame: config.first_frame,
            last_frame: config.last_frame,
            frame_duration: config.frame_duration,
        }
    }
}

/// Resource containing the loaded player sprite sheet.
#[derive(Resource, Default)]
pub struct PlayerSpriteSheet {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub animation: PlayerAnimationConfig,
}

impl PlayerSpriteSheet {
    /// Check if the sprite sheet is loaded.
    pub fn is_loaded(&self) -> bool {
        self.texture.is_some() && self.layout.is_some()
    }
}

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
            animation: sheet.animation.clone().into(),
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
    // Viking Swordman: 35 frames total, 32x32 each, idle is frames 0-3
    let texture: Handle<Image> = asset_server.load("sprites/player/viking_swordman.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 35, 1, None, None);
    let layout_handle = layouts.add(layout);

    player_sheet.texture = Some(texture);
    player_sheet.layout = Some(layout_handle);
    player_sheet.animation = PlayerAnimationConfig {
        first_frame: 0,
        last_frame: 3,
        frame_duration: 0.15,
    };

    info!("Loaded player sprite sheet: Viking Swordman");
}
