//! Sprite loading and management for the game.
//!
//! This module provides a system for loading sprite sheets with JSON metadata,
//! with support for named sprite access via Bevy's async asset pipeline.
//!
//! # Usage
//!
//! ```ignore
//! use crate::assets::sprites::SpriteSheetKey;
//!
//! // Access sprites by key
//! if let Some(icons) = game_sprites.get(SpriteSheetKey::UiIcons) {
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

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use serde::Deserialize;
use std::collections::HashMap;

// ============================================================================
// Sprite Sheet Key Enum
// ============================================================================

/// Keys for identifying sprite sheets.
///
/// Add new variants here when adding new sprite sheets to the game.
/// Remember to also update `SpriteSheetKey::all()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteSheetKey {
    /// UI icons (hearts, stars, items, arrows, etc.)
    UiIcons,
    /// UI buttons (various sizes and colors)
    UiButtons,
    /// Book UI elements (frames, backgrounds)
    BookUi,
    /// UI frames and borders
    UiFrames,
    /// UI bars (health, mana, progress)
    UiBars,
    /// Combined UI sprite sheet (Cute Fantasy UI pack)
    UiAll,
    /// Item icons (potions, weapons, armor, etc.)
    IconItems,
    /// UI selectors (selection highlights, cursors)
    UiSelectors,
    /// TravelBook UI elements (banners, panels)
    TravelBook,
}

impl SpriteSheetKey {
    /// Get all sprite sheet keys.
    pub const fn all() -> &'static [Self] {
        &[
            Self::UiIcons,
            Self::UiButtons,
            Self::BookUi,
            Self::UiFrames,
            Self::UiBars,
            Self::UiAll,
            Self::IconItems,
            Self::UiSelectors,
            Self::TravelBook,
        ]
    }

    /// Get the asset file name (without extension) for this sprite sheet.
    pub const fn asset_name(&self) -> &'static str {
        match self {
            Self::UiIcons => "ui_icons",
            Self::UiButtons => "ui_buttons",
            Self::BookUi => "book_ui",
            Self::UiFrames => "ui_frames",
            Self::UiBars => "ui_bars",
            Self::UiAll => "ui_all",
            Self::IconItems => "icon_items",
            Self::UiSelectors => "ui_selectors",
            Self::TravelBook => "travel_book",
        }
    }
}

// ============================================================================
// Sprite Sheet Metadata Types (for JSON parsing)
// ============================================================================

/// Sprite sheet JSON metadata (Hash format).
///
/// This represents the structure of sprite sheet JSON files.
#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct SpriteSheetMeta {
    /// Map of frame names to frame data.
    pub frames: HashMap<String, SpriteFrameMeta>,
    /// Metadata about the sprite sheet.
    pub meta: SpriteSheetInfo,
}

/// A single frame/sprite in the sheet.
#[derive(Debug, Deserialize)]
pub struct SpriteFrameMeta {
    /// The rectangle defining this frame's position in the sheet.
    pub frame: SpriteRect,
    /// Whether the frame was rotated during packing.
    #[serde(default)]
    pub rotated: bool,
    /// Whether the frame was trimmed (transparent pixels removed).
    #[serde(default)]
    pub trimmed: bool,
    /// Original sprite size before trimming.
    #[serde(rename = "sourceSize")]
    pub source_size: Option<SpriteSize>,
    /// Sprite position within the original canvas.
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Option<SpriteRect>,
}

/// Rectangle coordinates in the sprite sheet.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SpriteRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Metadata about the sprite sheet.
#[derive(Debug, Deserialize)]
pub struct SpriteSheetInfo {
    /// Total size of the sprite sheet image.
    pub size: SpriteSize,
    /// Original filename (optional).
    #[serde(default)]
    pub image: String,
    /// Application that created this (optional).
    #[serde(default)]
    pub app: String,
    /// Scale factor (optional).
    #[serde(default)]
    pub scale: String,
    /// Slices defined in the sprite sheet (for irregular sprite regions).
    #[serde(default)]
    pub slices: Vec<SpriteSlice>,
}

/// A slice definition for irregular sprite regions.
#[derive(Debug, Deserialize)]
pub struct SpriteSlice {
    /// Name of the slice.
    pub name: String,
    /// Keyframes containing bounds for this slice.
    pub keys: Vec<SpriteSliceKey>,
}

/// A keyframe for a slice, containing its bounds.
#[derive(Debug, Deserialize)]
pub struct SpriteSliceKey {
    /// The bounding rectangle for this slice.
    pub bounds: SpriteRect,
}

/// Size dimensions.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SpriteSize {
    pub w: u32,
    pub h: u32,
}

impl SpriteSheetMeta {
    /// Convert this metadata to a Bevy TextureAtlasLayout.
    ///
    /// Returns both the layout and a mapping from sprite names to atlas indices.
    pub fn to_layout(&self) -> (TextureAtlasLayout, HashMap<String, usize>) {
        let mut layout =
            TextureAtlasLayout::new_empty(UVec2::new(self.meta.size.w, self.meta.size.h));
        let mut name_to_index = HashMap::new();

        // Sort frames by name for consistent ordering
        let mut frames: Vec<_> = self.frames.iter().collect();
        frames.sort_by_key(|(name, _)| *name);

        for (name, frame) in frames {
            let rect = URect::new(
                frame.frame.x,
                frame.frame.y,
                frame.frame.x + frame.frame.w,
                frame.frame.y + frame.frame.h,
            );
            let index = layout.add_texture(rect);
            name_to_index.insert(name.clone(), index);
        }

        // Also add slices (used for irregular sprite regions)
        for slice in &self.meta.slices {
            if let Some(key) = slice.keys.first() {
                let bounds = &key.bounds;
                let rect = URect::new(
                    bounds.x,
                    bounds.y,
                    bounds.x + bounds.w,
                    bounds.y + bounds.h,
                );
                let index = layout.add_texture(rect);
                name_to_index.insert(slice.name.clone(), index);
            }
        }

        (layout, name_to_index)
    }
}


// ============================================================================
// Asset Loader
// ============================================================================

/// Error type for sprite sheet loading.
#[derive(Debug, thiserror::Error)]
pub enum SpriteSheetLoaderError {
    #[error("Failed to read sprite sheet: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse sprite sheet JSON: {0}")]
    Parse(#[from] serde_json::Error),
}

/// Asset loader for sprite sheet JSON metadata.
#[derive(Default)]
pub struct SpriteSheetMetaLoader;

impl AssetLoader for SpriteSheetMetaLoader {
    type Asset = SpriteSheetMeta;
    type Settings = ();
    type Error = SpriteSheetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

// ============================================================================
// Plugin
// ============================================================================

/// Plugin that loads and manages game assets.
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheetMeta>()
            .init_asset_loader::<SpriteSheetMetaLoader>()
            .init_resource::<GameAssets>()
            .init_resource::<GameSprites>()
            .init_resource::<GameFonts>()
            .init_resource::<PendingSpriteSheets>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Update, finalize_sprite_sheets);
    }
}

// ============================================================================
// SpriteSheet (Runtime representation)
// ============================================================================

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

    /// Create an ImageNode for the named sprite (UI).
    ///
    /// Use this for UI elements that need to display sprites from the atlas.
    /// Returns `None` if the sprite name doesn't exist.
    pub fn image_node(&self, name: &str) -> Option<ImageNode> {
        let index = self.get(name)?;
        Some(ImageNode::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index,
            },
        ))
    }

    /// Create a 9-slice ImageNode for panels/buttons.
    ///
    /// 9-slice images stretch the center while preserving border regions,
    /// perfect for panels, buttons, and frames.
    /// Returns `None` if the sprite name doesn't exist.
    pub fn image_node_sliced(&self, name: &str, border: f32) -> Option<ImageNode> {
        use bevy::ui::widget::NodeImageMode;
        self.image_node(name).map(|img| {
            img.with_mode(NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(border),
                ..default()
            }))
        })
    }
}

// ============================================================================
// GameSprites Resource
// ============================================================================

/// Resource containing loaded fonts.
#[derive(Resource, Default)]
pub struct GameFonts {
    /// CuteFantasy pixel font (5x9)
    pub pixel: Handle<Font>,
}

impl GameFonts {
    /// Create a TextFont for the pixel font with proper settings.
    pub fn pixel_font(&self, font_size: f32) -> TextFont {
        TextFont {
            font: self.pixel.clone(),
            font_size,
            font_smoothing: bevy::text::FontSmoothing::None,
            ..default()
        }
    }
}

/// Resource containing all loaded sprite sheets.
///
/// Sprite sheets are indexed by `SpriteSheetKey` and loaded from
/// `assets/sprites/` with matching `.json` metadata.
#[derive(Resource, Default)]
pub struct GameSprites {
    sheets: HashMap<SpriteSheetKey, SpriteSheet>,
}

impl GameSprites {
    /// Get a sprite sheet by key.
    ///
    /// Returns `None` if the sprite sheet hasn't been loaded yet.
    pub fn get(&self, key: SpriteSheetKey) -> Option<&SpriteSheet> {
        self.sheets.get(&key)
    }

    /// Insert a sprite sheet.
    fn insert(&mut self, key: SpriteSheetKey, sheet: SpriteSheet) {
        self.sheets.insert(key, sheet);
    }

    /// Check if a sprite sheet is loaded.
    fn contains(&self, key: SpriteSheetKey) -> bool {
        self.sheets.contains_key(&key)
    }
}

// ============================================================================
// Async Loading Infrastructure
// ============================================================================

/// Tracks pending sprite sheet loads.
#[derive(Resource, Default)]
struct PendingSpriteSheets {
    handles: HashMap<SpriteSheetKey, Handle<SpriteSheetMeta>>,
}

/// Build a SpriteSheet from loaded metadata.
fn build_sprite_sheet(
    meta: &SpriteSheetMeta,
    name: &str,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> SpriteSheet {
    let png_path = format!("sprites/{}.png", name);
    let (layout, sprites) = meta.to_layout();
    let layout_handle = texture_atlas_layouts.add(layout);
    let texture = asset_server.load(&png_path);

    info!(
        "Loaded sprite sheet '{}' with {} sprites",
        name,
        sprites.len()
    );

    SpriteSheet {
        texture,
        layout: layout_handle,
        sprites,
    }
}

/// System to finalize sprite sheets when their metadata loads.
fn finalize_sprite_sheets(
    mut events: EventReader<AssetEvent<SpriteSheetMeta>>,
    meta_assets: Res<Assets<SpriteSheetMeta>>,
    pending: Res<PendingSpriteSheets>,
    mut game_sprites: ResMut<GameSprites>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        for (key, handle) in &pending.handles {
            if handle.id() == *id && !game_sprites.contains(*key) {
                if let Some(meta) = meta_assets.get(*id) {
                    let sheet = build_sprite_sheet(
                        meta,
                        key.asset_name(),
                        &asset_server,
                        &mut texture_atlas_layouts,
                    );
                    game_sprites.insert(*key, sheet);
                }
            }
        }
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

    // Background images
    pub menu_background: Option<Handle<Image>>,
    pub fight_backgrounds: Vec<Handle<Image>>,
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
    mut game_assets: ResMut<GameAssets>,
    mut game_fonts: ResMut<GameFonts>,
    mut pending: ResMut<PendingSpriteSheets>,
) {
    // Load fonts
    game_fonts.pixel = asset_server.load("fonts/CuteFantasy-5x9.ttf");

    // Kick off async sprite sheet loads for all registered sheets
    for key in SpriteSheetKey::all() {
        let path = format!("sprites/{}.json", key.asset_name());
        pending.handles.insert(*key, asset_server.load(&path));
    }

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

    game_assets.sprites.menu_background = try_load(&asset_server, "backgrounds/sunrise.png");

    // Load fight backgrounds (80 images)
    for i in 1..=80 {
        if let Some(handle) = try_load(&asset_server, &format!("backgrounds/fight/{}.png", i)) {
            game_assets.sprites.fight_backgrounds.push(handle);
        }
    }

    info!("Asset loading initiated");
}

/// Try to load an asset, returning None if file doesn't exist.
fn try_load(asset_server: &AssetServer, path: &str) -> Option<Handle<Image>> {
    Some(asset_server.load(path))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sprite_sheet_json() {
        let json = r#"{
            "frames": {
                "heart_full": {
                    "frame": {"x": 0, "y": 0, "w": 16, "h": 16},
                    "rotated": false,
                    "trimmed": false
                },
                "heart_empty": {
                    "frame": {"x": 16, "y": 0, "w": 16, "h": 16},
                    "rotated": false,
                    "trimmed": false
                }
            },
            "meta": {
                "size": {"w": 32, "h": 16}
            }
        }"#;

        let sheet: SpriteSheetMeta = serde_json::from_str(json).unwrap();
        assert_eq!(sheet.frames.len(), 2);
        assert_eq!(sheet.meta.size.w, 32);
        assert_eq!(sheet.meta.size.h, 16);

        let (layout, name_to_index) = sheet.to_layout();
        assert_eq!(layout.len(), 2);
        assert!(name_to_index.contains_key("heart_full"));
        assert!(name_to_index.contains_key("heart_empty"));
    }
}
