//! Grid-based tileset for direct TMX tile ID rendering.
//!
//! This module provides a way to render tiles directly by their TMX tile ID,
//! bypassing the named-slice system used for procedural dungeon generation.

use bevy::prelude::*;

/// Grid-based tileset for TMX direct tile rendering.
///
/// Stores a texture and grid layout where tile IDs map directly to indices.
/// Tile ID 0 = empty (not rendered), tile ID 1+ = grid index (id - firstgid).
#[derive(Resource, Default)]
pub struct TmxTilesetGrid {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
    /// Number of columns in the tileset grid.
    pub columns: u32,
    /// First global ID (tiles in TMX are offset by this).
    pub first_gid: u32,
    /// Tile dimensions in pixels.
    pub tile_width: u32,
    pub tile_height: u32,
}

impl TmxTilesetGrid {
    /// Create an ImageNode for a specific tile ID.
    ///
    /// Returns None for tile ID 0 (empty) or if tileset not initialized.
    pub fn image_node_for_tile(&self, tile_id: u32) -> Option<ImageNode> {
        if tile_id == 0 {
            return None;
        }

        let texture = self.texture.clone()?;
        let layout = self.layout.clone()?;

        // Convert global ID to local index
        let local_id = tile_id.saturating_sub(self.first_gid);

        Some(ImageNode::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: local_id as usize,
            },
        ))
    }

    /// Check if the tileset is initialized and ready to use.
    pub fn is_ready(&self) -> bool {
        self.texture.is_some() && self.layout.is_some()
    }
}

/// Initialize the TMX tileset grid for cave tiles.
///
/// Creates a grid-based TextureAtlasLayout covering all tiles in the tileset.
pub fn init_tmx_tileset_grid(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> TmxTilesetGrid {
    // Cave tileset parameters (from cave.tsx)
    const COLUMNS: u32 = 14;
    const TILE_WIDTH: u32 = 32;
    const TILE_HEIGHT: u32 = 32;
    const TILE_COUNT: u32 = 280;
    const IMAGE_WIDTH: u32 = 448;
    const IMAGE_HEIGHT: u32 = 640;
    const FIRST_GID: u32 = 1;

    // Load the tileset texture
    let texture = asset_server.load("maps/Cave Tileset.png");

    // Create grid-based layout with all tiles
    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(IMAGE_WIDTH, IMAGE_HEIGHT));

    for tile_id in 0..TILE_COUNT {
        let col = tile_id % COLUMNS;
        let row = tile_id / COLUMNS;
        let x = col * TILE_WIDTH;
        let y = row * TILE_HEIGHT;

        layout.add_texture(URect::new(x, y, x + TILE_WIDTH, y + TILE_HEIGHT));
    }

    let layout_handle = layouts.add(layout);

    TmxTilesetGrid {
        texture: Some(texture),
        layout: Some(layout_handle),
        columns: COLUMNS,
        first_gid: FIRST_GID,
        tile_width: TILE_WIDTH,
        tile_height: TILE_HEIGHT,
    }
}
