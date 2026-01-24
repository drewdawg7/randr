//! Mob sprite animation system.
//!
//! Provides animated sprite support for mob sprites displayed in combat
//! and the MonsterCompendium.

use bevy::prelude::*;

use crate::mob::MobId;

use super::animation::AnimationConfig;
use super::sprite_marker::{SpriteData, SpriteMarker, SpriteMarkerAppExt};

/// Plugin for mob sprite animations.
pub struct MobAnimationPlugin;

impl Plugin for MobAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobSpriteSheets>()
            .add_systems(PreStartup, load_mob_sprite_sheets)
            .register_sprite_marker::<DungeonMobSprite>();
    }
}

/// A loaded mob sprite sheet with animation data.
#[derive(Debug)]
pub struct MobSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: AnimationConfig,
    pub death_animation: Option<AnimationConfig>,
    /// Frame dimensions in pixels (used for aspect ratio in rendering).
    pub frame_size: UVec2,
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

impl SpriteMarker for DungeonMobSprite {
    type Resources = Res<'static, MobSpriteSheets>;

    fn resolve(&self, sheets: &Res<MobSpriteSheets>) -> Option<SpriteData> {
        let sheet = sheets.get(self.mob_id)?;
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone(),
            flip_x: false,
        })
    }
}

/// System to load mob sprite sheets at startup.
fn load_mob_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut mob_sheets: ResMut<MobSpriteSheets>,
) {
    // Goblin: 6x6 grid of 32x32, idle is slices 0-3, death is slices 30-33
    let goblin_texture: Handle<Image> = asset_server.load("sprites/mobs/goblin.png");
    let goblin_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 6, None, None);
    let goblin_layout_handle = layouts.add(goblin_layout);
    mob_sheets.insert(
        MobId::Goblin,
        MobSpriteSheet {
            texture: goblin_texture,
            layout: goblin_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.2,
                looping: true,
                synchronized: true,
            },
            death_animation: Some(AnimationConfig {
                first_frame: 30,
                last_frame: 33,
                frame_duration: 0.15,
                looping: false,
                synchronized: false,
            }),
            frame_size: UVec2::splat(32),
        },
    );

    // Slime: 8x6 grid of 32x32, idle is slices 0-3, death is slices 40-44
    let slime_texture: Handle<Image> = asset_server.load("sprites/mobs/slime.png");
    let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 6, None, None);
    let slime_layout_handle = layouts.add(slime_layout);
    mob_sheets.insert(
        MobId::Slime,
        MobSpriteSheet {
            texture: slime_texture,
            layout: slime_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.25,
                looping: true,
                synchronized: true,
            },
            death_animation: Some(AnimationConfig {
                first_frame: 40,
                last_frame: 44,
                frame_duration: 0.15,
                looping: false,
                synchronized: false,
            }),
            frame_size: UVec2::splat(32),
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
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.35,
                looping: true,
                synchronized: true,
            },
            death_animation: None,
            frame_size: UVec2::new(64, 32),
        },
    );

    // Black Dragon: 16x7 grid of 64x32, idle is frames 2-5, death is frames 98-103
    let black_dragon_texture: Handle<Image> = asset_server.load("sprites/mobs/black_dragon.png");
    let black_dragon_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 16, 7, None, None);
    let black_dragon_layout_handle = layouts.add(black_dragon_layout);
    mob_sheets.insert(
        MobId::BlackDragon,
        MobSpriteSheet {
            texture: black_dragon_texture,
            layout: black_dragon_layout_handle,
            animation: AnimationConfig {
                first_frame: 2,
                last_frame: 5,
                frame_duration: 0.35,
                looping: true,
                synchronized: true,
            },
            death_animation: Some(AnimationConfig {
                first_frame: 98,
                last_frame: 103,
                frame_duration: 0.15,
                looping: false,
                synchronized: false,
            }),
            frame_size: UVec2::new(64, 32),
        },
    );

    info!("Loaded mob sprite sheets for Goblin, Slime, Dragon, and Black Dragon");
}
