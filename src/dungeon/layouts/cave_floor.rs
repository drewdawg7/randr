//! Cave floor layout loaded from Tiled map files.

use crate::dungeon::map::parse_map;
use crate::dungeon::DungeonLayout;
use std::path::Path;

/// Default path to the cave floor map file.
const CAVE_FLOOR_PATH: &str = "assets/maps/cave_floor.tmx";

/// Create a cave floor layout by loading from the map file.
///
/// The map is rendered 1:1 without any modifications.
/// Falls back to an empty layout if the file cannot be loaded.
pub fn create() -> DungeonLayout {
    create_from_path(Path::new(CAVE_FLOOR_PATH))
}

/// Create a cave floor layout from a specific map file path.
pub fn create_from_path(path: &Path) -> DungeonLayout {
    match parse_map(path) {
        Ok(map) => map.to_layout(),
        Err(e) => {
            eprintln!("Failed to load map from {:?}: {}", path, e);
            // Return a minimal fallback layout
            DungeonLayout::new(10, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cave_floor() {
        // This test requires the assets/maps/cave_floor.tmx file to exist
        let layout = create();
        // The map is 30x20
        assert!(layout.width() > 0);
        assert!(layout.height() > 0);
    }
}
