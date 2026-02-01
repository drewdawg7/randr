use bevy::prelude::*;

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Component)]
pub struct Interpolating;

#[derive(Resource)]
pub struct PendingPlayerSpawn(pub Vec2);
