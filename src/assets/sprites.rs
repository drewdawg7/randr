use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::ui::{AnimationConfig, SpriteAnimation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteSheetKey {
    UiIcons,
    UiButtons,
    BookUi,
    UiFrames,
    UiBars,
    UiAll,
    IconItems,
    UiSelectors,
    TravelBook,
    BookSlot,
    GridSlot,
    MenuBackground,
    FightPopup,
    FightBackgrounds,
    ShopBgSlices,
    DetailPanelBg,
    ItemDetailIcons,
    HealthIcon,
    DefenseIcon,
    GoldIcon,
    DefaultStatIcon,
    DungeonTileset,
    Chests,
    Rocks,
    FightBannerSlices,
    OkButton,
    OkButtonSelected,
    CancelButton,
    CancelButtonSelected,
    TorchWall,
    TinSword,
    CopperSword,
    BronzeSword,
    Forge,
}

impl SpriteSheetKey {
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
            Self::BookSlot,
            Self::GridSlot,
            Self::MenuBackground,
            Self::FightPopup,
            Self::FightBackgrounds,
            Self::ShopBgSlices,
            Self::DetailPanelBg,
            Self::ItemDetailIcons,
            Self::HealthIcon,
            Self::DefenseIcon,
            Self::GoldIcon,
            Self::DefaultStatIcon,
            Self::DungeonTileset,
            Self::Chests,
            Self::Rocks,
            Self::FightBannerSlices,
            Self::OkButton,
            Self::OkButtonSelected,
            Self::CancelButton,
            Self::CancelButtonSelected,
            Self::TorchWall,
            Self::TinSword,
            Self::CopperSword,
            Self::BronzeSword,
            Self::Forge,
        ]
    }

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
            Self::BookSlot => "book_slot",
            Self::GridSlot => "grid_slot",
            Self::MenuBackground => "menu_background",
            Self::FightPopup => "fight_popup",
            Self::FightBackgrounds => "fight_backgrounds",
            Self::ShopBgSlices => "shop_bg_slices",
            Self::DetailPanelBg => "detail_panel_bg",
            Self::ItemDetailIcons => "item_detail_icons/item_detail_icons",
            Self::HealthIcon => "item_detail_icons/health_icon",
            Self::DefenseIcon => "item_detail_icons/defense_icon",
            Self::GoldIcon => "item_detail_icons/gold_icon",
            Self::DefaultStatIcon => "item_detail_icons/default_stat_icon",
            Self::DungeonTileset => "dungeon_tileset",
            Self::Chests => "dungeon_entities/chests",
            Self::Rocks => "dungeon_entities/rocks",
            Self::FightBannerSlices => "fight_banner_slices",
            Self::OkButton => "ok_button",
            Self::OkButtonSelected => "ok_button_selected",
            Self::CancelButton => "cancel_button",
            Self::CancelButtonSelected => "cancel_button_selected",
            Self::TorchWall => "torch_wall",
            Self::TinSword => "tin_sword",
            Self::CopperSword => "copper_sword",
            Self::BronzeSword => "bronze_sword",
            Self::Forge => "dungeon_entities/forge",
        }
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct SpriteSheetMeta {
    pub frames: HashMap<String, SpriteFrameMeta>,
    pub meta: SpriteSheetInfo,
}

#[derive(Debug, Deserialize)]
pub struct SpriteFrameMeta {
    pub frame: SpriteRect,
    #[serde(default)]
    pub rotated: bool,
    #[serde(default)]
    pub trimmed: bool,
    #[serde(rename = "sourceSize")]
    pub source_size: Option<SpriteSize>,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Option<SpriteRect>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SpriteRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteSheetInfo {
    pub size: SpriteSize,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub app: String,
    #[serde(default)]
    pub scale: String,
    #[serde(default)]
    pub slices: Vec<SpriteSlice>,
}

#[derive(Debug, Deserialize)]
pub struct SpriteSlice {
    pub name: String,
    pub keys: Vec<SpriteSliceKey>,
}

#[derive(Debug, Deserialize)]
pub struct SpriteSliceKey {
    pub bounds: SpriteRect,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SpriteSize {
    pub w: u32,
    pub h: u32,
}

impl SpriteSheetMeta {
    pub fn to_layout(&self) -> (TextureAtlasLayout, HashMap<String, usize>) {
        let mut layout =
            TextureAtlasLayout::new_empty(UVec2::new(self.meta.size.w, self.meta.size.h));
        let mut name_to_index = HashMap::new();

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

#[derive(Debug, thiserror::Error)]
pub enum SpriteSheetLoaderError {
    #[error("Failed to read sprite sheet: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse sprite sheet JSON: {0}")]
    Parse(#[from] serde_json::Error),
}

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

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheetMeta>()
            .init_asset_loader::<SpriteSheetMetaLoader>()
            .init_resource::<GameSprites>()
            .init_resource::<GameFonts>()
            .init_resource::<PendingSpriteSheets>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Update, finalize_sprite_sheets);
    }
}

#[derive(Debug)]
pub struct SpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub sprites: HashMap<String, usize>,
}

impl SpriteSheet {
    pub fn get(&self, name: &str) -> Option<usize> {
        self.sprites.get(name).copied()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.sprites.contains_key(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.sprites.keys().map(|s| s.as_str())
    }

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

    pub fn image_node_sliced(&self, name: &str, border: f32) -> Option<ImageNode> {
        use bevy::ui::widget::NodeImageMode;
        self.image_node(name).map(|img| {
            img.with_mode(NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(border),
                ..default()
            }))
        })
    }

    /// Returns (ImageNode, Node) bundle with sizing for UI sprites.
    ///
    /// # Example
    /// ```ignore
    /// cell.spawn(sheet.image_bundle("heart", 32.0, 32.0)?);
    /// ```
    pub fn image_bundle(&self, name: &str, width: f32, height: f32) -> Option<impl Bundle> {
        let image_node = self.image_node(name)?;
        Some((
            image_node,
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                ..default()
            },
        ))
    }

    /// Returns (ImageNode, Node, SpriteAnimation) bundle with animation.
    ///
    /// # Example
    /// ```ignore
    /// cell.spawn(sheet.image_bundle_animated("chest", 64.0, 64.0, AnimationConfig::default())?);
    /// ```
    pub fn image_bundle_animated(
        &self,
        name: &str,
        width: f32,
        height: f32,
        config: AnimationConfig,
    ) -> Option<impl Bundle> {
        let image_node = self.image_node(name)?;
        Some((
            image_node,
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                ..default()
            },
            SpriteAnimation::new(&config),
        ))
    }
}

#[derive(Resource, Default)]
pub struct GameFonts {
    pub pixel: Handle<Font>,
}

impl GameFonts {
    pub fn pixel_font(&self, font_size: f32) -> TextFont {
        TextFont {
            font: self.pixel.clone(),
            font_size,
            font_smoothing: bevy::text::FontSmoothing::None,
            ..default()
        }
    }
}

#[derive(Resource, Default)]
pub struct GameSprites {
    sheets: HashMap<SpriteSheetKey, SpriteSheet>,
}

impl GameSprites {
    pub fn get(&self, key: SpriteSheetKey) -> Option<&SpriteSheet> {
        self.sheets.get(&key)
    }

    fn insert(&mut self, key: SpriteSheetKey, sheet: SpriteSheet) {
        self.sheets.insert(key, sheet);
    }

    fn contains(&self, key: SpriteSheetKey) -> bool {
        self.sheets.contains_key(&key)
    }
}

#[derive(Resource, Default)]
struct PendingSpriteSheets {
    handles: HashMap<SpriteSheetKey, Handle<SpriteSheetMeta>>,
}

fn build_sprite_sheet(
    meta: &SpriteSheetMeta,
    name: &str,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> SpriteSheet {
    let png_path = if meta.meta.image.is_empty() {
        format!("sprites/{}.png", name)
    } else {
        meta.meta.image.clone()
    };
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

fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_fonts: ResMut<GameFonts>,
    mut pending: ResMut<PendingSpriteSheets>,
) {
    game_fonts.pixel = asset_server.load("fonts/FantasyRPGtitle.ttf");

    for key in SpriteSheetKey::all() {
        let path = format!("sprites/{}.json", key.asset_name());
        pending.handles.insert(*key, asset_server.load(&path));
    }

    info!("Asset loading initiated");
}

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
