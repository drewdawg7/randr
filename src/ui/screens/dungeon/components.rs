use bevy::prelude::*;

#[derive(Resource)]
pub struct UiScale(pub u32);

impl UiScale {
    pub fn calculate(window_height: f32) -> u32 {
        match window_height as u32 {
            0..=400 => 2,
            401..=800 => 4,
            801..=1600 => 8,
            _ => 16,
        }
    }
}

#[derive(Resource)]
pub struct TileSizes {
    pub tile_size: f32,
    pub base_tile_size: f32,
}

#[derive(Component)]
pub struct SmoothPosition {
    pub current: Vec2,
    pub target: Vec2,
    pub moving: bool,
}

#[derive(Component)]
pub struct EntityLayer;

#[derive(Component)]
pub struct DungeonCell;

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct DungeonGrid;

#[derive(Component)]
pub struct DungeonContainer;
