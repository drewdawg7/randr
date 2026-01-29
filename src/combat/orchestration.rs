use bevy::prelude::*;

use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::mob::{Mob, MobId};
use crate::player::{PlayerGold, PlayerName};
use super::log::CombatLogEntry;
use crate::stats::StatSheet;

use super::{
    apply_victory_rewards_direct, mob_attacks_player, player_attacks_mob,
    player_effective_magicfind, process_player_defeat, ActiveCombat, ActiveCombatResource,
    CombatPhaseState, CombatSourceResource, IsKillable, Named,
};

#[derive(Resource, Default)]
pub struct CombatLogState {
    pub entries: Vec<CombatLogEntry>,
}

/// Spawns the appropriate mob based on the combat source.
fn spawn_mob_for_source(_combat_source: &CombatSourceResource) -> Mob {
    // For testing: only spawn slimes
    MobId::Slime.spawn()
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
fn get_return_state(_combat_source: &CombatSourceResource) -> crate::states::AppState {
    crate::states::AppState::Town
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
    inventory: Res<Inventory>,
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

        // Player attacks mob using direct resource access
        let player_result = player_attacks_mob(name.0, &stats, &inventory, &mut combat.mob);
        log_state.entries.push(CombatLogEntry::player_attack(
            player_result.damage_to_target,
            &player_result.defender,
        ));

        if player_result.target_died {
            log_state
                .entries
                .push(CombatLogEntry::enemy_defeated(&player_result.defender));

            // Process victory rewards using direct resources
            let death_result = combat.mob.on_death(player_effective_magicfind(&stats, &inventory));
            let rewards = apply_victory_rewards_direct(
                &stats,
                &inventory,
                &mut gold,
                &mut progression,
                death_result.gold_dropped,
                death_result.xp_dropped,
            );
            combat.gold_gained = rewards.gold_gained;
            combat.xp_gained = rewards.xp_gained;
            combat.loot_drops = death_result.loot_drops;

            next_phase.set(CombatPhaseState::Victory);
        } else {
            // Mob counter-attacks player using direct resources
            let enemy_result = mob_attacks_player(&combat.mob, name.0, &mut stats, &inventory);
            log_state.entries.push(CombatLogEntry::enemy_attack(
                enemy_result.damage_to_target,
                &enemy_result.attacker,
            ));

            if enemy_result.target_died {
                log_state.entries.push(CombatLogEntry::player_defeated());
                process_player_defeat(&mut stats, &mut gold);
                next_phase.set(CombatPhaseState::Defeat);
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
