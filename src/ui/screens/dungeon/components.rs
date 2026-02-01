use bevy::prelude::*;

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Resource)]
pub struct PendingPlayerSpawn(pub Vec2);
