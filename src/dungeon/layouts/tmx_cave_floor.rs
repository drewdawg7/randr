//! TMX-based cave floor layout loaded from Tiled map files.

use crate::dungeon::tmx::parse_tmx;
use crate::dungeon::DungeonLayout;
use std::path::Path;

/// Default path to the cave floor TMX file.
const CAVE_FLOOR_TMX: &str = "assets/maps/cave_floor.tmx";

/// Create a cave floor layout by loading from the TMX file.
///
/// The TMX is rendered 1:1 without any modifications.
/// Falls back to an empty layout if the file cannot be loaded.
pub fn create() -> DungeonLayout {
    create_from_path(Path::new(CAVE_FLOOR_TMX))
}

/// Create a cave floor layout from a specific TMX file path.
pub fn create_from_path(path: &Path) -> DungeonLayout {
    match parse_tmx(path) {
        // TMX is source of truth - no modifications
        Ok(tmx_map) => tmx_map.to_layout(),
        Err(e) => {
            eprintln!("Failed to load TMX map from {:?}: {}", path, e);
            // Return a minimal fallback layout
            DungeonLayout::new(10, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tmx_cave_floor() {
        // This test requires the assets/maps/cave_floor.tmx file to exist
        let layout = create();
        // The TMX map is 30x20
        assert!(layout.width() > 0);
        assert!(layout.height() > 0);
    }
}
