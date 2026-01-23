use bevy::prelude::*;

use crate::combat::{ActiveCombatResource, CombatPhaseState};
use crate::input::clear_game_action_events;
use crate::inventory::Inventory;
use crate::stats::StatSheet;
use crate::states::AppState;

use super::actions::{despawn_post_combat_overlay, reset_fight_state, spawn_post_combat_overlay};
use super::input::{handle_player_turn_input, handle_post_combat_input};
use super::spawn::spawn_fight_screen;
use super::state::FightScreenState;
use super::systems::{
    cleanup_fight_screen, populate_fight_background, populate_fight_popup, populate_mob_sprite,
    update_combat_visuals, update_enemy_name, SelectedFightBackground,
};
use crate::ui::init_sprite_health_bars;

/// SystemSets for organizing Fight screen systems by function.
/// Configured to run in order: Input -> UI
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FightSystemSet {
    /// Handle player input (combat actions, post-combat choices)
    Input,
    /// Update combat visuals and overlays
    Ui,
}

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FightScreenState>()
            .init_resource::<SelectedFightBackground>()
            // Configure SystemSet ordering: Input runs before UI
            .configure_sets(
                Update,
                (FightSystemSet::Input, FightSystemSet::Ui)
                    .chain()
                    .run_if(in_state(AppState::Fight)),
            )
            .add_systems(
                OnEnter(AppState::Fight),
                (spawn_fight_screen, reset_fight_state).chain(),
            )
            .add_systems(OnExit(AppState::Fight), (cleanup_fight_screen, clear_game_action_events))
            // Input systems
            .add_systems(
                Update,
                handle_player_turn_input
                    .in_set(FightSystemSet::Input)
                    .run_if(in_state(CombatPhaseState::PlayerTurn)),
            )
            .add_systems(
                Update,
                handle_post_combat_input
                    .in_set(FightSystemSet::Input)
                    .run_if(
                        in_state(CombatPhaseState::Victory).or(in_state(CombatPhaseState::Defeat)),
                    ),
            )
            // UI systems
            .add_systems(
                Update,
                (
                    init_sprite_health_bars,
                    populate_fight_background,
                    populate_fight_popup,
                    populate_mob_sprite,
                )
                    .in_set(FightSystemSet::Ui),
            )
            .add_systems(
                Update,
                spawn_post_combat_overlay
                    .in_set(FightSystemSet::Ui)
                    .run_if(
                        in_state(CombatPhaseState::Victory).or(in_state(CombatPhaseState::Defeat)),
                    ),
            )
            .add_systems(
                Update,
                update_combat_visuals
                    .in_set(FightSystemSet::Ui)
                    .run_if(
                        resource_changed::<StatSheet>
                            .or(resource_changed::<Inventory>)
                            .or(resource_changed::<ActiveCombatResource>),
                    ),
            )
            .add_systems(
                Update,
                update_enemy_name
                    .in_set(FightSystemSet::Ui)
                    .run_if(resource_changed::<ActiveCombatResource>),
            )
            .add_systems(
                OnEnter(CombatPhaseState::PlayerTurn),
                despawn_post_combat_overlay,
            );
    }
}
