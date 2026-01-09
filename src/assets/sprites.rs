//! Sprite loading and management for the game.
//!
//! This module provides a system for loading sprite sheets exported from Aseprite,
//! with support for both JSON metadata (named sprite access) and simple image loading.
//!
//! # Usage
//!
//! ```ignore
//! // Access sprites by name
//! if let Some(icons) = &game_sprites.ui_icons {
//!     if let Some(index) = icons.get("heart_full") {
//!         commands.spawn((
//!             Sprite::from_atlas_image(
//!                 icons.texture.clone(),
//!                 TextureAtlas { layout: icons.layout.clone(), index },
//!             ),
//!             Transform::from_xyz(0.0, 0.0, 0.0),
//!         ));
//!     }
//! }
//! ```

use bevy::prelude::*;
use std::collections::HashMap;

use crate::assets::aseprite::AsepriteSheet;

/// Plugin that loads and manages game assets.
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .init_resource::<GameSprites>()
            .add_systems(PreStartup, load_assets);
    }
}

/// A loaded sprite sheet with named sprite access.
///
/// This struct holds a texture, its atlas layout, and a mapping from
/// sprite names to their indices in the atlas.
#[derive(Debug)]
pub struct SpriteSheet {
    /// Handle to the sprite sheet texture.
    pub texture: Handle<Image>,
    /// Handle to the texture atlas layout.
    pub layout: Handle<TextureAtlasLayout>,
    /// Map from sprite names to atlas indices.
    pub sprites: HashMap<String, usize>,
}

impl SpriteSheet {
    /// Get a sprite's atlas index by name.
    ///
    /// Returns `None` if no sprite with that name exists.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.sprites.get(name).copied()
    }

    /// Check if a sprite with the given name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.sprites.contains_key(name)
    }

    /// Get all sprite names in this sheet.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.sprites.keys().map(|s| s.as_str())
    }

    /// Create a Sprite component for the named sprite.
    ///
    /// Returns `None` if the sprite name doesn't exist.
    pub fn sprite(&self, name: &str) -> Option<Sprite> {
        let index = self.get(name)?;
        Some(Sprite::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index,
            },
        ))
    }

    /// Create a Sprite component with custom size for the named sprite.
    pub fn sprite_sized(&self, name: &str, size: Vec2) -> Option<Sprite> {
        let index = self.get(name)?;
        let mut sprite = Sprite::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index,
            },
        );
        sprite.custom_size = Some(size);
        Some(sprite)
    }
}

/// Resource containing all loaded sprite sheets.
///
/// Add fields here for each sprite sheet your game uses.
/// Sheets are loaded from `assets/sprites/` with matching `.json` metadata.
#[derive(Resource, Default)]
pub struct GameSprites {
    /// UI icons (hearts, stars, items, arrows, etc.)
    pub ui_icons: Option<SpriteSheet>,
    /// UI buttons (various sizes and colors)
    pub ui_buttons: Option<SpriteSheet>,
    /// Book UI elements (frames, backgrounds)
    pub book_ui: Option<SpriteSheet>,
    /// UI frames and borders
    pub ui_frames: Option<SpriteSheet>,
    /// UI bars (health, mana, progress)
    pub ui_bars: Option<SpriteSheet>,
}

impl GameSprites {
    /// Load a sprite sheet from Aseprite JSON export.
    ///
    /// Expects both `{name}.png` and `{name}.json` in `assets/sprites/`.
    pub fn load_sheet(
        name: &str,
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Option<SpriteSheet> {
        let json_path = format!("assets/sprites/{}.json", name);
        let png_path = format!("sprites/{}.png", name);

        // Try to read the JSON file
        let json = match std::fs::read_to_string(&json_path) {
            Ok(content) => content,
            Err(e) => {
                debug!("Could not load sprite sheet JSON '{}': {}", json_path, e);
                return None;
            }
        };

        // Parse the Aseprite JSON
        let sheet = match AsepriteSheet::load(&json) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to parse Aseprite JSON '{}': {}", json_path, e);
                return None;
            }
        };

        // Convert to Bevy layout
        let (layout, sprites) = sheet.to_layout();
        let layout_handle = texture_atlas_layouts.add(layout);
        let texture = asset_server.load(&png_path);

        info!(
            "Loaded sprite sheet '{}' with {} sprites",
            name,
            sprites.len()
        );

        Some(SpriteSheet {
            texture,
            layout: layout_handle,
            sprites,
        })
    }
}

// ============================================================================
// Legacy support - keeping GameAssets for backward compatibility
// ============================================================================

/// Container for all loaded game assets (legacy).
///
/// This is kept for backward compatibility with existing code.
/// New code should use `GameSprites` instead.
#[derive(Resource, Default)]
pub struct GameAssets {
    pub sprites: SpriteAssets,
}

/// Sprite asset handles for functional game elements (legacy).
#[derive(Default)]
pub struct SpriteAssets {
    // Mine screen sprites
    pub mine_wall: Option<Handle<Image>>,
    pub mine_floor: Option<Handle<Image>>,
    pub mine_rock: Option<Handle<Image>>,
    pub mine_ore: Option<Handle<Image>>,
    pub mine_player: Option<Handle<Image>>,
    pub mine_pickaxe: Option<Handle<Image>>,
    pub mine_ladder: Option<Handle<Image>>,

    // Fight screen sprites
    pub fight_player: Option<Handle<Image>>,
    pub fight_health_bar: Option<Handle<Image>>,

    // Dungeon minimap sprites
    pub dungeon_unexplored: Option<Handle<Image>>,
    pub dungeon_current: Option<Handle<Image>>,
    pub dungeon_cleared: Option<Handle<Image>>,
    pub dungeon_boss: Option<Handle<Image>>,
}

impl SpriteAssets {
    /// Check if mine sprites are loaded.
    pub fn mine_ready(&self) -> bool {
        self.mine_wall.is_some()
            && self.mine_floor.is_some()
            && self.mine_rock.is_some()
            && self.mine_player.is_some()
    }

    /// Check if fight sprites are loaded.
    pub fn fight_ready(&self) -> bool {
        self.fight_player.is_some() && self.fight_health_bar.is_some()
    }

    /// Check if dungeon minimap sprites are loaded.
    pub fn dungeon_ready(&self) -> bool {
        self.dungeon_unexplored.is_some()
            && self.dungeon_current.is_some()
            && self.dungeon_cleared.is_some()
            && self.dungeon_boss.is_some()
    }
}

/// System to load assets at startup.
fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_assets: ResMut<GameAssets>,
    mut game_sprites: ResMut<GameSprites>,
) {
    // Load new sprite sheet system
    game_sprites.ui_icons =
        GameSprites::load_sheet("ui_icons", &asset_server, &mut texture_atlas_layouts);
    game_sprites.ui_buttons =
        GameSprites::load_sheet("ui_buttons", &asset_server, &mut texture_atlas_layouts);
    game_sprites.book_ui =
        GameSprites::load_sheet("book_ui", &asset_server, &mut texture_atlas_layouts);
    game_sprites.ui_frames =
        GameSprites::load_sheet("ui_frames", &asset_server, &mut texture_atlas_layouts);
    game_sprites.ui_bars =
        GameSprites::load_sheet("ui_bars", &asset_server, &mut texture_atlas_layouts);

    // Legacy sprite loading (individual files)
    game_assets.sprites.mine_wall = try_load(&asset_server, "sprites/mine/wall.png");
    game_assets.sprites.mine_floor = try_load(&asset_server, "sprites/mine/floor.png");
    game_assets.sprites.mine_rock = try_load(&asset_server, "sprites/mine/rock.png");
    game_assets.sprites.mine_ore = try_load(&asset_server, "sprites/mine/ore.png");
    game_assets.sprites.mine_player = try_load(&asset_server, "sprites/mine/player.png");
    game_assets.sprites.mine_pickaxe = try_load(&asset_server, "sprites/mine/pickaxe.png");
    game_assets.sprites.mine_ladder = try_load(&asset_server, "sprites/mine/ladder.png");

    game_assets.sprites.fight_player = try_load(&asset_server, "sprites/fight/player.png");
    game_assets.sprites.fight_health_bar = try_load(&asset_server, "sprites/fight/health_bar.png");

    game_assets.sprites.dungeon_unexplored =
        try_load(&asset_server, "sprites/dungeon/unexplored.png");
    game_assets.sprites.dungeon_current = try_load(&asset_server, "sprites/dungeon/current.png");
    game_assets.sprites.dungeon_cleared = try_load(&asset_server, "sprites/dungeon/cleared.png");
    game_assets.sprites.dungeon_boss = try_load(&asset_server, "sprites/dungeon/boss.png");

    info!("Asset loading initiated");
}

/// Try to load an asset, returning None if file doesn't exist.
fn try_load(asset_server: &AssetServer, path: &str) -> Option<Handle<Image>> {
    Some(asset_server.load(path))
}
