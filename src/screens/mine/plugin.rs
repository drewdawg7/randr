use bevy::prelude::*;

use crate::states::AppState;

use super::grid::{update_grid_tiles, update_player_sprite};
use super::state::MineScreenState;
use super::systems::{
    cleanup_mine_screen, handle_ladder_exit, handle_mine_input, handle_mining_action,
    reset_mine_state, spawn_mine_screen, update_message_display,
};

/// Plugin that manages the mine screen.
pub struct MinePlugin;

impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MineScreenState>()
            .add_systems(OnEnter(AppState::Mine), (spawn_mine_screen, reset_mine_state).chain())
            .add_systems(OnExit(AppState::Mine), cleanup_mine_screen)
            .add_systems(
                Update,
                (
                    handle_mine_input,
                    handle_mining_action,
                    handle_ladder_exit,
                    update_player_sprite,
                    update_grid_tiles,
                    update_message_display,
                )
                    .run_if(in_state(AppState::Mine)),
            );
    }
}
