use crate::dungeon::{DungeonLayout, LayoutBuilder};
use crate::ui::DUNGEON_SCALE;

/// Cave tile scale factor - caves use 32x32 tiles (2x dungeon's 16x16).
const CAVE_TILE_SCALE: f32 = 2.0;

pub fn create_with_stairs() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    // Divide by both DUNGEON_SCALE and CAVE_TILE_SCALE to get half the grid size
    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE / CAVE_TILE_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE / CAVE_TILE_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        // No torches in caves
        .build()
}

pub fn create_final() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE / CAVE_TILE_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE / CAVE_TILE_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        // No torches in caves
        .build()
}
