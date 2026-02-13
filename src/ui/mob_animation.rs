//! Mob sprite animation system.
//!
//! Provides animated sprite support for mob sprites displayed in combat
//! and the MonsterCompendium.

use bevy::prelude::*;

use crate::combat::events::DamageEntity;
use crate::mob::components::MobMarker;
use crate::mob::MobId;
use crate::states::AppState;

use super::animation::{AnimationConfig, SpriteAnimation};
use super::sprite_marker::{SpriteData, SpriteMarker, SpriteMarkerAppExt};

/// Plugin for mob sprite animations.
pub struct MobAnimationPlugin;

impl Plugin for MobAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobSpriteSheets>()
            .add_systems(PreStartup, load_mob_sprite_sheets)
            .add_systems(
                Update,
                (
                    trigger_hurt_animation.run_if(on_message::<DamageEntity>),
                    revert_hurt_animation,
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            )
            .register_sprite_marker::<DungeonMobSprite>();
    }
}

/// A loaded mob sprite sheet with animation data.
#[derive(Debug)]
pub struct MobSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: AnimationConfig,
    pub hurt_animation: Option<AnimationConfig>,
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
            hurt_animation: Some(AnimationConfig {
                first_frame: 18,
                last_frame: 21,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
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
            hurt_animation: Some(AnimationConfig {
                first_frame: 24,
                last_frame: 27,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
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
            hurt_animation: None,
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
            hurt_animation: None,
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

    // Merchant: 23x1 grid of 32x32, idle is frames 0-3
    let merchant_texture: Handle<Image> = asset_server.load("sprites/mobs/merchant.png");
    let merchant_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 23, 1, None, None);
    let merchant_layout_handle = layouts.add(merchant_layout);
    mob_sheets.insert(
        MobId::Merchant,
        MobSpriteSheet {
            texture: merchant_texture,
            layout: merchant_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.15,
                looping: true,
                synchronized: true,
            },
            hurt_animation: None,
            death_animation: None,
            frame_size: UVec2::splat(32),
        },
    );

    // Dwarf Defender: 6x7 grid of 32x32, idle is frames 0-3, death is frames 36-41
    let dwarf_defender_texture: Handle<Image> =
        asset_server.load("sprites/mobs/dwarf_defender.png");
    let dwarf_defender_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 7, None, None);
    let dwarf_defender_layout_handle = layouts.add(dwarf_defender_layout);
    mob_sheets.insert(
        MobId::DwarfDefender,
        MobSpriteSheet {
            texture: dwarf_defender_texture,
            layout: dwarf_defender_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.2,
                looping: true,
                synchronized: true,
            },
            hurt_animation: Some(AnimationConfig {
                first_frame: 18,
                last_frame: 21,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
            death_animation: Some(AnimationConfig {
                first_frame: 36,
                last_frame: 41,
                frame_duration: 0.15,
                looping: false,
                synchronized: false,
            }),
            frame_size: UVec2::splat(32),
        },
    );

    // Dwarf Warrior: 6x6 grid of 32x32, idle is frames 0-3, death is frames 30-33
    let dwarf_warrior_texture: Handle<Image> =
        asset_server.load("sprites/mobs/dwarf_warrior.png");
    let dwarf_warrior_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 6, None, None);
    let dwarf_warrior_layout_handle = layouts.add(dwarf_warrior_layout);
    mob_sheets.insert(
        MobId::DwarfWarrior,
        MobSpriteSheet {
            texture: dwarf_warrior_texture,
            layout: dwarf_warrior_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.2,
                looping: true,
                synchronized: true,
            },
            hurt_animation: Some(AnimationConfig {
                first_frame: 18,
                last_frame: 21,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
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

    // Dwarf Miner: 6x6 grid of 32x32, idle is frames 0-3, death is frames 30-33
    let dwarf_miner_texture: Handle<Image> = asset_server.load("sprites/mobs/dwarf_miner.png");
    let dwarf_miner_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 6, None, None);
    let dwarf_miner_layout_handle = layouts.add(dwarf_miner_layout);
    mob_sheets.insert(
        MobId::DwarfMiner,
        MobSpriteSheet {
            texture: dwarf_miner_texture,
            layout: dwarf_miner_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.2,
                looping: true,
                synchronized: true,
            },
            hurt_animation: Some(AnimationConfig {
                first_frame: 18,
                last_frame: 21,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
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

    // Dwarf King: 7x7 grid of 32x32, idle is frames 0-3, death is frames 42-48
    let dwarf_king_texture: Handle<Image> = asset_server.load("sprites/mobs/dwarf_king.png");
    let dwarf_king_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 7, 7, None, None);
    let dwarf_king_layout_handle = layouts.add(dwarf_king_layout);
    mob_sheets.insert(
        MobId::DwarfKing,
        MobSpriteSheet {
            texture: dwarf_king_texture,
            layout: dwarf_king_layout_handle,
            animation: AnimationConfig {
                first_frame: 0,
                last_frame: 3,
                frame_duration: 0.25,
                looping: true,
                synchronized: true,
            },
            hurt_animation: Some(AnimationConfig {
                first_frame: 21,
                last_frame: 24,
                frame_duration: 0.1,
                looping: false,
                synchronized: false,
            }),
            death_animation: Some(AnimationConfig {
                first_frame: 42,
                last_frame: 48,
                frame_duration: 0.15,
                looping: false,
                synchronized: false,
            }),
            frame_size: UVec2::splat(32),
        },
    );

    info!("Loaded mob sprite sheets for all mobs");
}

/// Marker for entities currently playing a hurt animation.
#[derive(Component)]
pub struct PlayingHurtAnimation;

/// Triggers hurt animation when a mob takes damage.
/// `Without<PlayingHurtAnimation>` prevents restarting mid-animation.
fn trigger_hurt_animation(
    mut commands: Commands,
    mut events: MessageReader<DamageEntity>,
    mut mobs: Query<(&MobMarker, &mut SpriteAnimation), Without<PlayingHurtAnimation>>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    for event in events.read() {
        let Ok((marker, mut animation)) = mobs.get_mut(event.target) else {
            continue;
        };
        let Some(sheet) = mob_sheets.get(marker.0) else {
            continue;
        };
        let Some(hurt_config) = &sheet.hurt_animation else {
            continue;
        };

        animation.apply_config(hurt_config);
        commands.entity(event.target).insert(PlayingHurtAnimation);
    }
}

/// Reverts mob to idle animation after hurt animation completes.
fn revert_hurt_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpriteAnimation, &PlayingHurtAnimation, &MobMarker)>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    for (entity, mut animation, _, mob_marker) in &mut query {
        let animation_finished = !animation.looping && animation.current_frame >= animation.last_frame;
        if !animation_finished {
            continue;
        }

        if let Some(sheet) = mob_sheets.get(mob_marker.0) {
            animation.apply_config(&sheet.animation);
        }
        commands.entity(entity).remove::<PlayingHurtAnimation>();
    }
}
