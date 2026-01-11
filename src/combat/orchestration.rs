use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::mob::{Mob, MobId};
use crate::player::{Player, PlayerGold, PlayerName};
use super::log::CombatLogEntry;
use crate::stats::StatSheet;

use super::{
    enemy_attack_step, player_attack_step, process_defeat, process_victory, ActiveCombat,
    ActiveCombatResource, CombatPhaseState, CombatSourceResource, Named,
};

#[derive(Resource, Default)]
pub struct CombatLogState {
    pub entries: Vec<CombatLogEntry>,
}

/// Spawns the appropriate mob based on the combat source.
fn spawn_mob_for_source(combat_source: &CombatSourceResource) -> Mob {
    match *combat_source {
        CombatSourceResource::Field => {
            let field_mobs = [MobId::Slime, MobId::Cow, MobId::Goblin];
            let mob_id = field_mobs.choose(&mut rand::thread_rng()).unwrap();
            mob_id.spawn()
        }
        CombatSourceResource::Dungeon => MobId::Goblin.spawn(),
        CombatSourceResource::DungeonBoss => MobId::Dragon.spawn(),
    }
}

/// Sets up a new combat encounter: spawns mob, clears log, and initializes combat state.
fn setup_new_combat(
    combat_res: &mut ActiveCombatResource,
    log_state: &mut CombatLogState,
    combat_source: &CombatSourceResource,
) {
    let mob = spawn_mob_for_source(combat_source);

    log_state.entries.clear();
    let enemy_name = mob.name().to_string();
    log_state
        .entries
        .push(CombatLogEntry::info(format!("A wild {} appears!", enemy_name)));

    combat_res.0 = Some(ActiveCombat::new(mob));
}

/// Returns the app state to transition to when leaving combat.
fn get_return_state(combat_source: &CombatSourceResource) -> crate::states::AppState {
    match *combat_source {
        CombatSourceResource::Field => crate::states::AppState::Town,
        CombatSourceResource::Dungeon | CombatSourceResource::DungeonBoss => {
            crate::states::AppState::Dungeon
        }
    }
}

#[derive(Event, Debug, Clone)]
pub enum PlayerCombatAction {
    Attack,
    Run,
}

#[derive(Event, Debug, Clone)]
pub enum PostCombatAction {
    FightAgain,
    Continue,
}

pub fn initialize_combat(
    mut combat_res: ResMut<ActiveCombatResource>,
    mut log_state: ResMut<CombatLogState>,
    combat_source: Res<CombatSourceResource>,
    mut next_phase: ResMut<NextState<CombatPhaseState>>,
) {
    setup_new_combat(&mut combat_res, &mut log_state, &combat_source);
    next_phase.set(CombatPhaseState::PlayerTurn);
}

pub fn execute_player_attack(
    mut action_events: EventReader<PlayerCombatAction>,
    mut combat_res: ResMut<ActiveCombatResource>,
    name: Res<PlayerName>,
    mut gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
    mut log_state: ResMut<CombatLogState>,
    mut next_phase: ResMut<NextState<CombatPhaseState>>,
) {
    for action in action_events.read() {
        if !matches!(action, PlayerCombatAction::Attack) {
            continue;
        }

        let Some(combat) = combat_res.get_mut() else {
            continue;
        };

        // Build Player view for combat
        let mut player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);

        let player_result = player_attack_step(&player, combat);
        log_state.entries.push(CombatLogEntry::player_attack(
            player_result.damage_to_target,
            &player_result.defender,
        ));

        if player_result.target_died {
            log_state
                .entries
                .push(CombatLogEntry::enemy_defeated(&player_result.defender));
            process_victory(&mut player, combat);
            // Write changes back to resources
            player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
            next_phase.set(CombatPhaseState::Victory);
        } else {
            let enemy_result = enemy_attack_step(combat, &mut player);
            log_state.entries.push(CombatLogEntry::enemy_attack(
                enemy_result.damage_to_target,
                &enemy_result.attacker,
            ));

            if enemy_result.target_died {
                log_state.entries.push(CombatLogEntry::player_defeated());
                process_defeat(&mut player);
                // Write changes back to resources
                player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
                next_phase.set(CombatPhaseState::Defeat);
            } else {
                // Write stat changes back (HP damage)
                player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
            }
        }
    }
}

pub fn handle_run_action(
    mut action_events: EventReader<PlayerCombatAction>,
    combat_source: Res<CombatSourceResource>,
    mut next_app_state: ResMut<NextState<crate::states::AppState>>,
) {
    for action in action_events.read() {
        if !matches!(action, PlayerCombatAction::Run) {
            continue;
        }

        next_app_state.set(get_return_state(&combat_source));
    }
}

pub fn handle_fight_again(
    mut action_events: EventReader<PostCombatAction>,
    mut combat_res: ResMut<ActiveCombatResource>,
    mut log_state: ResMut<CombatLogState>,
    combat_source: Res<CombatSourceResource>,
    mut next_phase: ResMut<NextState<CombatPhaseState>>,
) {
    for action in action_events.read() {
        if !matches!(action, PostCombatAction::FightAgain) {
            continue;
        }

        setup_new_combat(&mut combat_res, &mut log_state, &combat_source);
        next_phase.set(CombatPhaseState::PlayerTurn);
    }
}

pub fn handle_continue_action(
    mut action_events: EventReader<PostCombatAction>,
    combat_source: Res<CombatSourceResource>,
    mut next_app_state: ResMut<NextState<crate::states::AppState>>,
) {
    for action in action_events.read() {
        if !matches!(action, PostCombatAction::Continue) {
            continue;
        }

        next_app_state.set(get_return_state(&combat_source));
    }
}

pub fn cleanup_combat(mut combat_res: ResMut<ActiveCombatResource>) {
    combat_res.clear();
}
