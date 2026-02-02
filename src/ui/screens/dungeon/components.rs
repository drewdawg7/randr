use bevy::prelude::*;

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct FloorRoot;

#[derive(Resource)]
pub struct PendingPlayerSpawn(pub Vec2);
