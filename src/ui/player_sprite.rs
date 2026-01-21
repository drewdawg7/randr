//! Player sprite animation system.
//!
//! Provides animated sprite support for the player character in dungeons.

use bevy::prelude::*;

/// Plugin for player sprite animations.
pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpriteSheet>()
            .add_systems(PreStartup, load_player_sprite_sheet)
            .add_systems(Update, (animate_player_sprite, populate_dungeon_player_sprite));
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

/// Component for animated player sprites.
#[derive(Component)]
pub struct PlayerAnimation {
    /// Timer for frame advancement
    pub timer: Timer,
    /// Current frame index within the animation
    pub current_frame: usize,
    /// First frame index
    pub first_frame: usize,
    /// Last frame index (inclusive)
    pub last_frame: usize,
}

impl PlayerAnimation {
    /// Create a new player animation from a configuration.
    pub fn new(config: &PlayerAnimationConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.frame_duration, TimerMode::Repeating),
            current_frame: config.first_frame,
            first_frame: config.first_frame,
            last_frame: config.last_frame,
        }
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

/// System to animate player sprites.
fn animate_player_sprite(time: Res<Time>, mut query: Query<(&mut PlayerAnimation, &mut ImageNode)>) {
    for (mut animation, mut image) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // Advance to next frame, wrapping back to first
            animation.current_frame += 1;
            if animation.current_frame > animation.last_frame {
                animation.current_frame = animation.first_frame;
            }

            // Update the atlas index
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = animation.current_frame;
            }
        }
    }
}

/// System to populate dungeon player sprites with textures and animation.
fn populate_dungeon_player_sprite(
    mut commands: Commands,
    query: Query<Entity, Added<DungeonPlayerSprite>>,
    player_sheet: Res<PlayerSpriteSheet>,
) {
    if !player_sheet.is_loaded() {
        return;
    }

    let texture = player_sheet.texture.as_ref().unwrap().clone();
    let layout = player_sheet.layout.as_ref().unwrap().clone();

    for entity in &query {
        commands
            .entity(entity)
            .remove::<DungeonPlayerSprite>()
            .insert((
                ImageNode::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: layout.clone(),
                        index: player_sheet.animation.first_frame,
                    },
                ),
                PlayerAnimation::new(&player_sheet.animation),
            ));
    }
}
