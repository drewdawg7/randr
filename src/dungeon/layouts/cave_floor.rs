use bevy_ecs_tiled::prelude::tiled;

use crate::dungeon::map::map_to_layout;
use crate::dungeon::DungeonLayout;

const CAVE_FLOOR_PATH: &str = "assets/maps/cave_floor.tmx";

pub fn create() -> DungeonLayout {
    let mut loader = tiled::Loader::new();
    match loader.load_tmx_map(CAVE_FLOOR_PATH) {
        Ok(map) => map_to_layout(&map),
        Err(e) => {
            eprintln!("Failed to load map from {}: {}", CAVE_FLOOR_PATH, e);
            DungeonLayout::new(10, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cave_floor() {
        let layout = create();
        assert_eq!(layout.width(), 15);
        assert_eq!(layout.height(), 11);
    }
}
