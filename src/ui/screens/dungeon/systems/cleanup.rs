use bevy::prelude::*;

use super::super::components::DungeonRoot;
use crate::dungeon::{DungeonState, GridOccupancy};

pub fn cleanup_dungeon(
    mut commands: Commands,
    query: Query<Entity, With<DungeonRoot>>,
    mut state: ResMut<DungeonState>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    state.exit_dungeon();
    commands.remove_resource::<GridOccupancy>();
}
