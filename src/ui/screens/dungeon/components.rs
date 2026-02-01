use bevy::prelude::*;

#[derive(Resource)]
pub struct TileSizes {
    pub tile_size: f32,
    pub base_tile_size: f32,
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

#[derive(Component)]
pub struct TargetPosition(pub Vec2);
