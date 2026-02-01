#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySize {
    pub width: f32,
    pub height: f32,
}

impl Default for EntitySize {
    fn default() -> Self {
        Self {
            width: 32.0,
            height: 32.0,
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
