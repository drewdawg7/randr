use bevy::prelude::*;

use crate::combat::CombatPhaseState;
use crate::input::clear_game_action_events;
use crate::states::AppState;

use super::input::{handle_player_turn_input, handle_post_combat_input};
use super::state::FightScreenState;
use super::ui::{
    cleanup_fight_screen, despawn_post_combat_overlay, reset_fight_state, spawn_fight_screen,
    spawn_post_combat_overlay, update_combat_visuals,
};

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FightScreenState>()
            .add_systems(
                OnEnter(AppState::Fight),
                (spawn_fight_screen, reset_fight_state).chain(),
            )
            .add_systems(OnExit(AppState::Fight), (cleanup_fight_screen, clear_game_action_events))
            .add_systems(
                Update,
                handle_player_turn_input.run_if(in_state(CombatPhaseState::PlayerTurn)),
            )
            .add_systems(
                Update,
                (spawn_post_combat_overlay, handle_post_combat_input)
                    .chain()
                    .run_if(
                        in_state(CombatPhaseState::Victory).or(in_state(CombatPhaseState::Defeat)),
                    ),
            )
            .add_systems(
                OnEnter(CombatPhaseState::PlayerTurn),
                despawn_post_combat_overlay,
            )
            .add_systems(Update, update_combat_visuals.run_if(in_state(AppState::Fight)));
    }
}
