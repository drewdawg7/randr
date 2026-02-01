use bevy_ecs_tiled::prelude::tiled;

use crate::dungeon::map::map_to_layout;
use crate::dungeon::DungeonLayout;

const HOME_FLOOR_PATH: &str = "assets/maps/home_floor.tmx";

pub fn create() -> DungeonLayout {
    let mut loader = tiled::Loader::new();
    match loader.load_tmx_map(HOME_FLOOR_PATH) {
        Ok(map) => map_to_layout(&map),
        Err(e) => {
            eprintln!("Failed to load map from {}: {}", HOME_FLOOR_PATH, e);
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
