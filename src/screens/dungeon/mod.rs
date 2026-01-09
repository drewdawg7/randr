mod boss;
mod navigation;
mod rest;
mod room_entry;
mod state;

use bevy::prelude::*;

pub use state::{DungeonMode, DungeonSelectionState};

/// Plugin that manages the dungeon screen and its various view modes.
pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<DungeonMode>()
            .init_resource::<DungeonSelectionState>()
            // OnEnter systems for each mode
            .add_systems(OnEnter(DungeonMode::Navigation), navigation::spawn_navigation_ui)
            .add_systems(OnEnter(DungeonMode::RoomEntry), room_entry::spawn_room_entry_ui)
            .add_systems(OnEnter(DungeonMode::Rest), rest::spawn_rest_ui)
            .add_systems(OnEnter(DungeonMode::Boss), boss::spawn_boss_ui)
            // OnExit systems for each mode (cleanup)
            .add_systems(OnExit(DungeonMode::Navigation), navigation::despawn_navigation_ui)
            .add_systems(OnExit(DungeonMode::RoomEntry), room_entry::despawn_room_entry_ui)
            .add_systems(OnExit(DungeonMode::Rest), rest::despawn_rest_ui)
            .add_systems(OnExit(DungeonMode::Boss), boss::despawn_boss_ui)
            // Input handlers with state-based run conditions
            .add_systems(
                Update,
                navigation::handle_navigation_input.run_if(in_state(DungeonMode::Navigation)),
            )
            .add_systems(
                Update,
                room_entry::handle_room_entry_input.run_if(in_state(DungeonMode::RoomEntry)),
            )
            .add_systems(
                Update,
                rest::handle_rest_input.run_if(in_state(DungeonMode::Rest)),
            )
            .add_systems(
                Update,
                boss::handle_boss_input.run_if(in_state(DungeonMode::Boss)),
            );
    }
}
