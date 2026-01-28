//! Home floor layout loaded from Tiled map files.

use crate::dungeon::map::parse_map;
use crate::dungeon::DungeonLayout;
use std::path::Path;

/// Default path to the home floor map file.
const HOME_FLOOR_PATH: &str = "assets/maps/home_floor.tmx";

/// Create a home floor layout by loading from the map file.
///
/// The map is rendered 1:1 without any modifications.
/// Falls back to an empty layout if the file cannot be loaded.
pub fn create() -> DungeonLayout {
    create_from_path(Path::new(HOME_FLOOR_PATH))
}

/// Create a home floor layout from a specific map file path.
pub fn create_from_path(path: &Path) -> DungeonLayout {
    match parse_map(path) {
        Ok(map) => map.to_layout(),
        Err(e) => {
            eprintln!("Failed to load map from {:?}: {}", path, e);
            DungeonLayout::new(10, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_home_floor() {
        let layout = create();
        assert_eq!(layout.width(), 10);
        assert_eq!(layout.height(), 10);
    }
}
