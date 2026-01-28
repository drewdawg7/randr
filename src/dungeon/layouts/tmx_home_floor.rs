//! TMX-based home floor layout loaded from Tiled map files.

use crate::dungeon::tmx::parse_tmx;
use crate::dungeon::DungeonLayout;
use std::path::Path;

/// Default path to the home floor TMX file.
const HOME_FLOOR_TMX: &str = "assets/maps/home_floor.tmx";

/// Create a home floor layout by loading from the TMX file.
///
/// The TMX is rendered 1:1 without any modifications.
/// Falls back to an empty layout if the file cannot be loaded.
pub fn create() -> DungeonLayout {
    create_from_path(Path::new(HOME_FLOOR_TMX))
}

/// Create a home floor layout from a specific TMX file path.
pub fn create_from_path(path: &Path) -> DungeonLayout {
    match parse_tmx(path) {
        Ok(tmx_map) => tmx_map.to_layout(),
        Err(e) => {
            eprintln!("Failed to load TMX map from {:?}: {}", path, e);
            DungeonLayout::new(10, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tmx_home_floor() {
        let layout = create();
        assert_eq!(layout.width(), 10);
        assert_eq!(layout.height(), 10);
    }
}
