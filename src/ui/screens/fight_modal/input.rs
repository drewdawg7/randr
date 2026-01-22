//! Fight modal input handling.

use bevy::prelude::*;

use crate::combat::{apply_victory_rewards, attack, process_defeat, IsKillable};
use crate::dungeon::{GridOccupancy, GridSize};
use crate::entities::Progression;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::Inventory;
use crate::loot::collect_loot_drops;
use crate::player::{PlayerGold, PlayerGuard, PlayerName};
use crate::stats::StatSheet;
use crate::ui::SelectionState;

use super::super::modal::{close_modal, ActiveModal, ModalType};
use super::super::victory_modal::{SpawnVictoryModal, VictoryModalData};
use super::state::{FightModalButton, FightModalButtonSelection, FightModalMob, FightModalRoot};

/// System to handle closing the fight modal.
pub fn handle_fight_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<FightModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal
            && close_modal(
                &mut commands,
                &mut active_modal,
                &modal_query,
                ModalType::FightModal,
            )
        {
            commands.remove_resource::<FightModalMob>();
            commands.remove_resource::<FightModalButtonSelection>();
        }
    }
}

/// System to handle left/right button navigation.
pub fn handle_fight_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    mut selection: ResMut<FightModalButtonSelection>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Left) => selection.up(),
            GameAction::Navigate(NavigationDirection::Right) => selection.down(),
            _ => {}
        }
    }
}

/// System to handle OK/Cancel button activation with Enter.
#[allow(clippy::too_many_arguments)]
pub fn handle_fight_modal_select(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    selection: Res<FightModalButtonSelection>,
    mut fight_mob: ResMut<FightModalMob>,
    mut occupancy: ResMut<GridOccupancy>,
    player_name: Res<PlayerName>,
    mut player_gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
    mut active_modal: ResMut<ActiveModal>,
    modal_query: Query<Entity, With<FightModalRoot>>,
) {
    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        match selection.selected {
            FightModalButton::Ok => {
                // Build player guard for combat (auto-writes changes on drop)
                let mut player = PlayerGuard::from_resources(
                    &player_name,
                    &mut player_gold,
                    &mut progression,
                    &mut inventory,
                    &mut stats,
                );

                // Player attacks mob
                let result = attack(&*player, &mut fight_mob.mob);

                if result.target_died {
                    // Apply victory rewards
                    let death_result = fight_mob.mob.on_death(player.effective_magicfind());
                    let rewards = apply_victory_rewards(
                        &mut player,
                        death_result.gold_dropped,
                        death_result.xp_dropped,
                    );

                    // Collect loot into inventory
                    collect_loot_drops(&mut *player, &death_result.loot_drops);

                    // Despawn mob from dungeon and clear occupancy
                    occupancy.vacate(fight_mob.pos, GridSize::single());
                    commands.entity(fight_mob.entity).despawn_recursive();

                    // Close fight modal
                    close_modal(
                        &mut commands,
                        &mut active_modal,
                        &modal_query,
                        ModalType::FightModal,
                    );

                    // Spawn victory modal with results
                    commands.insert_resource(VictoryModalData {
                        mob_name: fight_mob.mob.name.clone(),
                        mob_id: fight_mob.mob_id,
                        gold_gained: rewards.gold_gained,
                        xp_gained: rewards.xp_gained,
                        loot_drops: death_result.loot_drops,
                    });
                    commands.insert_resource(SpawnVictoryModal);

                    commands.remove_resource::<FightModalMob>();
                    commands.remove_resource::<FightModalButtonSelection>();
                } else {
                    // Enemy counter-attack
                    let enemy_result = attack(&fight_mob.mob, &mut *player);

                    if enemy_result.target_died {
                        // Handle player defeat
                        process_defeat(&mut player);

                        // Close modal
                        close_modal(
                            &mut commands,
                            &mut active_modal,
                            &modal_query,
                            ModalType::FightModal,
                        );
                        commands.remove_resource::<FightModalMob>();
                        commands.remove_resource::<FightModalButtonSelection>();
                    }
                    // Otherwise combat continues - modal stays open for next attack
                }
            }
            FightModalButton::Cancel => {
                // Just close the modal, no combat
                close_modal(
                    &mut commands,
                    &mut active_modal,
                    &modal_query,
                    ModalType::FightModal,
                );
                commands.remove_resource::<FightModalMob>();
                commands.remove_resource::<FightModalButtonSelection>();
            }
        }
    }
}
