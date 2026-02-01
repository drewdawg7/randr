use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use super::super::components::DungeonRoot;
use crate::dungeon::{DungeonState, Occupancy};

pub fn cleanup_dungeon(
    mut commands: Commands,
    root_query: Query<Entity, With<DungeonRoot>>,
    tilemap_query: Query<Entity, With<TiledMap>>,
    mut state: ResMut<DungeonState>,
) {
    for entity in &root_query {
        commands.entity(entity).despawn();
    }
    for entity in &tilemap_query {
        commands.entity(entity).despawn();
    }
    state.exit_dungeon();
    commands.remove_resource::<Occupancy>();
}
