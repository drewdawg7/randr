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
}

impl FacingDirection {
    pub fn to_offset(self, distance: f32) -> Vec2 {
        match self {
            Self::Right => Vec2::X * distance,
            Self::Left => Vec2::NEG_X * distance,
        }
    }
}
