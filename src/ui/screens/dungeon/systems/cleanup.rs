use bevy::prelude::*;

use super::super::components::FloorRoot;
use crate::dungeon::DungeonState;

pub fn cleanup_dungeon(
    mut commands: Commands,
    floor_root_query: Query<Entity, With<FloorRoot>>,
    mut state: ResMut<DungeonState>,
) {
    if let Ok(floor_root) = floor_root_query.single() {
        commands.entity(floor_root).despawn();
    }
    state.exit_dungeon();
}
