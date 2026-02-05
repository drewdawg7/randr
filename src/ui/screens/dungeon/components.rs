use bevy::prelude::*;

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct FloorRoot;

#[derive(Resource)]
pub struct PendingPlayerSpawn;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl FacingDirection {
    pub fn to_offset(self, distance: f32) -> Vec2 {
        match self {
            Self::Right => Vec2::X * distance,
            Self::Left => Vec2::NEG_X * distance,
            Self::Up => Vec2::Y * distance,
            Self::Down => Vec2::NEG_Y * distance,
        }
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn hitbox_size(self, width: f32, height: f32) -> Vec2 {
        if self.is_horizontal() {
            Vec2::new(width, height)
        } else {
            Vec2::new(height, width)
        }
    }
}
