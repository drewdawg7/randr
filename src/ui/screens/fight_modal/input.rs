//! Fight modal input handling.

use bevy::prelude::*;

use crate::combat::{
    apply_victory_rewards_direct, entity_attacks_player, player_attacks_entity,
    player_effective_magicfind, process_player_defeat,
};
use crate::dungeon::{GridOccupancy, GridSize};
use crate::entities::Progression;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::Inventory;
use crate::loot::collect_loot_drops;
use crate::mob::{CombatStats, DeathProcessed, GoldReward, Health, MobLootTable, MobMarker, XpReward};
use crate::player::PlayerGold;
use crate::stats::StatSheet;
use crate::ui::{PlayerAttackTimer, PlayerSpriteSheet, SelectionState, SpriteAnimation};

use crate::plugins::MobDefeated;

use crate::ui::modal_registry::ModalCommands;

use super::super::modal::{ActiveModal, ModalType, OpenModal};
use super::super::results_modal::{ResultsModalData, ResultsSprite};
use super::state::{FightModal, FightModalButton, FightModalButtonSelection, FightModalMob};

/// System to handle closing the fight modal.
pub fn handle_fight_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal != Some(ModalType::FightModal) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            commands.close_modal::<FightModal>();
        }
    }
}

/// System to handle left/right button navigation.
pub fn handle_fight_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    selection: Option<ResMut<FightModalButtonSelection>>,
) {
    let Some(mut selection) = selection else { return };
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Left) => selection.up(),
            GameAction::Navigate(NavigationDirection::Right) => selection.down(),
            _ => {}
        }
    }
}

/// System to handle OK/Cancel button activation with Enter.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_fight_modal_select(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut mob_defeated_events: EventWriter<MobDefeated>,
    selection: Res<FightModalButtonSelection>,
    fight_mob: Res<FightModalMob>,
    mut occupancy: ResMut<GridOccupancy>,
    mut player_gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
    sheet: Res<PlayerSpriteSheet>,
    mut sprite_query: Query<(&mut SpriteAnimation, &mut PlayerAttackTimer)>,
    mut mob_query: Query<(
        &MobMarker,
        &mut Health,
        &CombatStats,
        &GoldReward,
        &XpReward,
        &MobLootTable,
        &mut DeathProcessed,
    )>,
) {
    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        match selection.selected {
            FightModalButton::Ok => {
                // Query mob entity's combat components
                let Ok((
                    mob_marker,
                    mut health,
                    combat_stats,
                    gold_reward,
                    xp_reward,
                    loot_table,
                    mut death_processed,
                )) = mob_query.get_mut(fight_mob.entity)
                else {
                    // Entity is invalid (despawned during floor transition) - close modal gracefully
                    warn!(
                        "Fight modal entity {:?} is invalid, closing modal",
                        fight_mob.entity
                    );
                    commands.close_modal::<FightModal>();
                    continue;
                };

                let mob_name = fight_mob.mob_id.spec().name.clone();

                // Player attacks mob using ECS components
                let result = player_attacks_entity(
                    &stats,
                    &inventory,
                    &mut health,
                    combat_stats,
                );

                // Switch to attack animation
                if let Ok((mut anim, mut attack_timer)) = sprite_query.get_single_mut() {
                    anim.first_frame = sheet.attack_animation.first_frame;
                    anim.last_frame = sheet.attack_animation.last_frame;
                    anim.current_frame = sheet.attack_animation.first_frame;
                    anim.frame_duration = sheet.attack_animation.frame_duration;
                    anim.looping = false;
                    anim.synchronized = false;
                    anim.timer = Timer::from_seconds(sheet.attack_animation.frame_duration, TimerMode::Repeating);
                    attack_timer.0.reset();
                }

                if result.target_died {
                    // Guard against double death processing
                    if death_processed.0 {
                        continue;
                    }
                    death_processed.0 = true;

                    // Roll loot drops
                    let magic_find = player_effective_magicfind(&stats, &inventory);
                    let loot_drops = loot_table.0.roll_drops(magic_find);

                    // Apply victory rewards using direct resources
                    let rewards = apply_victory_rewards_direct(
                        &mut stats,
                        &inventory,
                        &mut player_gold,
                        &mut progression,
                        gold_reward.0,
                        xp_reward.0,
                    );

                    // Collect loot into inventory
                    collect_loot_drops(&mut *inventory, &loot_drops);

                    // Send MobDefeated event for monster tracking
                    mob_defeated_events.send(MobDefeated {
                        mob_id: mob_marker.0,
                    });

                    // Despawn mob from dungeon and clear occupancy
                    occupancy.vacate(fight_mob.pos, GridSize::single());
                    commands.entity(fight_mob.entity).despawn_recursive();

                    // Close fight modal (cleanup removes FightModalMob and FightModalButtonSelection)
                    commands.close_modal::<FightModal>();

                    // Spawn results modal with victory data
                    commands.insert_resource(ResultsModalData {
                        title: "Victory!".to_string(),
                        subtitle: Some(mob_name),
                        sprite: Some(ResultsSprite::Mob(fight_mob.mob_id)),
                        gold_gained: Some(rewards.gold_gained),
                        xp_gained: Some(rewards.xp_gained),
                        loot_drops,
                    });
                    commands.trigger(OpenModal(ModalType::ResultsModal));
                } else {
                    // Enemy counter-attack using ECS components
                    let enemy_result = entity_attacks_player(
                        combat_stats,
                        &mut stats,
                        &inventory,
                    );

                    if enemy_result.target_died {
                        // Handle player defeat
                        process_player_defeat(&mut stats, &mut player_gold);

                        // Close modal (cleanup removes FightModalMob and FightModalButtonSelection)
                        commands.close_modal::<FightModal>();
                    }
                    // Otherwise combat continues - modal stays open for next attack
                }
            }
            FightModalButton::Cancel => {
                // Just close the modal, no combat (cleanup removes resources)
                commands.close_modal::<FightModal>();
            }
        }
    }
}
