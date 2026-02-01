use bevy::prelude::*;

#[derive(Resource)]
pub struct TileSizes {
    pub tile_size: f32,
    pub base_tile_size: f32,
    pub map_height: usize,
}


#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Component)]
pub struct Interpolating;
