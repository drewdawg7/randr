//! Mob sprite animation system.
//!
//! Provides animated sprite support for mob sprites displayed in combat
//! and the MonsterCompendium.

use bevy::prelude::*;

use crate::mob::MobId;

/// Plugin for mob sprite animations.
pub struct MobAnimationPlugin;

impl Plugin for MobAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobSpriteSheets>()
            .add_systems(PreStartup, load_mob_sprite_sheets)
            .add_systems(Update, (animate_mob_sprites, populate_dungeon_mob_sprites));
    }
}

/// Animation configuration for a mob's idle animation.
#[derive(Debug, Clone)]
pub struct MobAnimationConfig {
    /// First frame index of the idle animation
    pub first_frame: usize,
    /// Last frame index of the idle animation (inclusive)
    pub last_frame: usize,
    /// Duration per frame in seconds
    pub frame_duration: f32,
}

impl Default for MobAnimationConfig {
    fn default() -> Self {
        Self {
            first_frame: 0,
            last_frame: 3,
            frame_duration: 0.1,
        }
    }
}

/// A loaded mob sprite sheet with animation data.
#[derive(Debug)]
pub struct MobSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: MobAnimationConfig,
}

/// Resource containing loaded mob sprite sheets.
#[derive(Resource, Default)]
pub struct MobSpriteSheets {
    sheets: std::collections::HashMap<MobId, MobSpriteSheet>,
}

impl MobSpriteSheets {
    /// Get the sprite sheet for a mob, if available.
    pub fn get(&self, mob_id: MobId) -> Option<&MobSpriteSheet> {
        self.sheets.get(&mob_id)
    }

    /// Insert a sprite sheet for a mob.
    fn insert(&mut self, mob_id: MobId, sheet: MobSpriteSheet) {
        self.sheets.insert(mob_id, sheet);
    }
}

/// Marker component for dungeon mob sprites that need population.
#[derive(Component)]
pub struct DungeonMobSprite {
    pub mob_id: MobId,
}

/// Component for animated mob sprites.
///
/// Add this to an entity with an `ImageNode` to animate it.
#[derive(Component)]
pub struct MobAnimation {
    /// Timer for frame advancement
    pub timer: Timer,
    /// Current frame index within the animation
    pub current_frame: usize,
    /// First frame index
    pub first_frame: usize,
    /// Last frame index (inclusive)
    pub last_frame: usize,
}

impl MobAnimation {
    /// Create a new mob animation from a configuration.
    pub fn new(config: &MobAnimationConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.frame_duration, TimerMode::Repeating),
            current_frame: config.first_frame,
            first_frame: config.first_frame,
            last_frame: config.last_frame,
        }
    }

    /// Get the current atlas index.
    pub fn atlas_index(&self) -> usize {
        self.current_frame
    }
}

/// System to load mob sprite sheets at startup.
fn load_mob_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut mob_sheets: ResMut<MobSpriteSheets>,
) {
    // Goblin: 27 frames total, 32x32 each, idle is frames 0-3
    let goblin_texture: Handle<Image> = asset_server.load("sprites/mobs/goblin.png");
    let goblin_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 27, 1, None, None);
    let goblin_layout_handle = layouts.add(goblin_layout);
    mob_sheets.insert(
        MobId::Goblin,
        MobSpriteSheet {
            texture: goblin_texture,
            layout: goblin_layout_handle,
            animation: MobAnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.2,
            },
        },
    );

    // Slime: 18 frames total, 32x32 each, idle is frames 0-3
    let slime_texture: Handle<Image> = asset_server.load("sprites/mobs/slime.png");
    let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 18, 1, None, None);
    let slime_layout_handle = layouts.add(slime_layout);
    mob_sheets.insert(
        MobId::Slime,
        MobSpriteSheet {
            texture: slime_texture,
            layout: slime_layout_handle,
            animation: MobAnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.25,
            },
        },
    );

    // Dragon: 66 frames total, 64x32 each, idle is frames 0-3
    let dragon_texture: Handle<Image> = asset_server.load("sprites/mobs/dragon.png");
    let dragon_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 66, 1, None, None);
    let dragon_layout_handle = layouts.add(dragon_layout);
    mob_sheets.insert(
        MobId::Dragon,
        MobSpriteSheet {
            texture: dragon_texture,
            layout: dragon_layout_handle,
            animation: MobAnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.35,
            },
        },
    );

    info!("Loaded mob sprite sheets for Goblin, Slime, and Dragon");
}

/// System to animate mob sprites.
fn animate_mob_sprites(time: Res<Time>, mut query: Query<(&mut MobAnimation, &mut ImageNode)>) {
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

/// System to populate dungeon mob sprites with textures and animation.
fn populate_dungeon_mob_sprites(
    mut commands: Commands,
    query: Query<(Entity, &DungeonMobSprite), Added<DungeonMobSprite>>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    for (entity, marker) in &query {
        if let Some(sheet) = mob_sheets.get(marker.mob_id) {
            commands
                .entity(entity)
                .remove::<DungeonMobSprite>()
                .insert((
                    ImageNode::from_atlas_image(
                        sheet.texture.clone(),
                        TextureAtlas {
                            layout: sheet.layout.clone(),
                            index: sheet.animation.first_frame,
                        },
                    ),
                    MobAnimation::new(&sheet.animation),
                ));
        }
    }
}
