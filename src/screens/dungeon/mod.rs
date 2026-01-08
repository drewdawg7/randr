mod boss;
mod navigation;
mod rest;
mod room_entry;
mod state;

use bevy::prelude::*;

use crate::states::AppState;
pub use state::{DungeonScreenState, DungeonViewMode};

/// Plugin that manages the dungeon screen and its various view modes.
pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonScreenState>()
            // On entering Dungeon state, spawn navigation UI
            .add_systems(
                OnEnter(AppState::Dungeon),
                navigation::spawn_navigation_ui,
            )
            // On exiting Dungeon state, despawn all UI
            .add_systems(
                OnExit(AppState::Dungeon),
                (
                    navigation::despawn_navigation_ui,
                    room_entry::despawn_room_entry_ui,
                    rest::despawn_rest_ui,
                    boss::despawn_boss_ui,
                ),
            )
            // Systems that run when in Dungeon state
            .add_systems(
                Update,
                (
                    // Mode transition systems
                    handle_mode_transitions,
                    // Navigation mode systems
                    navigation::handle_navigation_input
                        .run_if(in_state(AppState::Dungeon).and_then(is_navigation_mode)),
                    // Room entry mode systems
                    room_entry::handle_room_entry_input
                        .run_if(in_state(AppState::Dungeon).and_then(is_room_entry_mode)),
                    // Rest mode systems
                    rest::handle_rest_input
                        .run_if(in_state(AppState::Dungeon).and_then(is_rest_mode)),
                    // Boss mode systems
                    boss::handle_boss_input
                        .run_if(in_state(AppState::Dungeon).and_then(is_boss_mode)),
                )
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

/// System that handles transitions between view modes by spawning/despawning UI
fn handle_mode_transitions(
    mut commands: Commands,
    mut state: ResMut<DungeonScreenState>,
    dungeon: Res<crate::game::DungeonResource>,
    player: Res<crate::game::PlayerResource>,
    navigation_root: Query<Entity, With<navigation::NavigationRoot>>,
    room_entry_root: Query<Entity, With<room_entry::RoomEntryRoot>>,
    rest_root: Query<Entity, With<rest::RestRoot>>,
    boss_root: Query<Entity, With<boss::BossRoot>>,
) {
    if !state.is_changed() {
        return;
    }

    // Despawn old UI based on what exists
    if let Ok(entity) = navigation_root.get_single() {
        if !state.is_navigation() {
            commands.entity(entity).despawn_recursive();
        }
    }
    if let Ok(entity) = room_entry_root.get_single() {
        if !state.is_room_entry() {
            commands.entity(entity).despawn_recursive();
        }
    }
    if let Ok(entity) = rest_root.get_single() {
        if !state.is_rest() {
            commands.entity(entity).despawn_recursive();
        }
    }
    if let Ok(entity) = boss_root.get_single() {
        if !state.is_boss() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Spawn new UI based on current mode
    match state.mode {
        DungeonViewMode::Navigation => {
            if navigation_root.get_single().is_err() {
                navigation::spawn_navigation_ui(commands.reborrow(), dungeon);
            }
        }
        DungeonViewMode::RoomEntry => {
            if room_entry_root.get_single().is_err() {
                room_entry::spawn_room_entry_ui(
                    commands.reborrow(),
                    &*dungeon,
                    state.as_mut(),
                );
            }
        }
        DungeonViewMode::Rest => {
            if rest_root.get_single().is_err() {
                rest::spawn_rest_ui(
                    commands.reborrow(),
                    &*player,
                    &*dungeon,
                    state.as_mut(),
                );
            }
        }
        DungeonViewMode::Boss => {
            if boss_root.get_single().is_err() {
                boss::spawn_boss_ui(commands.reborrow(), &*dungeon, state.as_mut());
            }
        }
    }
}

// Run conditions for different modes
fn is_navigation_mode(state: Res<DungeonScreenState>) -> bool {
    state.is_navigation()
}

fn is_room_entry_mode(state: Res<DungeonScreenState>) -> bool {
    state.is_room_entry()
}

fn is_rest_mode(state: Res<DungeonScreenState>) -> bool {
    state.is_rest()
}

fn is_boss_mode(state: Res<DungeonScreenState>) -> bool {
    state.is_boss()
}
