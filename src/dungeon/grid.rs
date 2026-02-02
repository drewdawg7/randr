use crate::dungeon::constants::DEFAULT_TILE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySize {
    pub width: f32,
    pub height: f32,
}

impl Default for EntitySize {
    fn default() -> Self {
        Self {
            width: DEFAULT_TILE_SIZE,
            height: DEFAULT_TILE_SIZE,
        }
    }
}

impl EntitySize {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn single(tile_size: f32) -> Self {
        Self {
            width: tile_size,
            height: tile_size,
        }
    }
}
