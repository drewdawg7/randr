use bevy::prelude::*;

use crate::combat::{EntityDied, PendingVictory, PlayerAttackMob};
use crate::input::{GameAction, NavigationDirection};
use crate::mob::Health;
use crate::ui::modal_registry::ModalCommands;
use crate::ui::{PlayerAttackTimer, PlayerSpriteSheet, SelectionState, SpriteAnimation};

use super::super::modal::{ActiveModal, ModalType, OpenModal};
use super::super::results_modal::{ResultsModalData, ResultsSprite};
use super::state::{FightModal, FightModalButton, FightModalButtonSelection, FightModalMob};

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

pub fn handle_fight_modal_select(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut attack_events: EventWriter<PlayerAttackMob>,
    selection: Res<FightModalButtonSelection>,
    fight_mob: Option<Res<FightModalMob>>,
    mob_query: Query<&Health>,
) {
    let Some(fight_mob) = fight_mob else { return };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        match selection.selected {
            FightModalButton::Ok => {
                if mob_query.get(fight_mob.entity).is_err() {
                    warn!(
                        "Fight modal entity {:?} is invalid, closing modal",
                        fight_mob.entity
                    );
                    commands.close_modal::<FightModal>();
                    continue;
                }

                attack_events.send(PlayerAttackMob {
                    target: fight_mob.entity,
                });
            }
            FightModalButton::Cancel => {
                commands.close_modal::<FightModal>();
            }
        }
    }
}

pub fn trigger_attack_animation(
    mut events: EventReader<PlayerAttackMob>,
    sheet: Res<PlayerSpriteSheet>,
    mut sprite_query: Query<(&mut SpriteAnimation, &mut PlayerAttackTimer)>,
) {
    for _ in events.read() {
        if let Ok((mut anim, mut attack_timer)) = sprite_query.get_single_mut() {
            anim.first_frame = sheet.attack_animation.first_frame;
            anim.last_frame = sheet.attack_animation.last_frame;
            anim.current_frame = sheet.attack_animation.first_frame;
            anim.frame_duration = sheet.attack_animation.frame_duration;
            anim.looping = false;
            anim.synchronized = false;
            anim.timer = Timer::from_seconds(
                sheet.attack_animation.frame_duration,
                TimerMode::Repeating,
            );
            attack_timer.0.reset();
        }
    }
}

pub fn handle_combat_outcome(
    mut commands: Commands,
    mut events: EventReader<EntityDied>,
    pending_victory: Option<Res<PendingVictory>>,
    fight_mob: Option<Res<FightModalMob>>,
) {
    for event in events.read() {
        if fight_mob.is_none() {
            continue;
        }

        if event.is_player {
            commands.close_modal::<FightModal>();
        } else if let Some(ref victory) = pending_victory {
            let fight_mob = fight_mob.as_ref().expect("checked above");
            commands.close_modal::<FightModal>();

            commands.insert_resource(ResultsModalData {
                title: "Victory!".to_string(),
                subtitle: Some(victory.mob_name.clone()),
                sprite: Some(ResultsSprite::Mob(fight_mob.mob_id)),
                gold_gained: Some(victory.gold_gained),
                xp_gained: Some(victory.xp_gained),
                loot_drops: victory.loot_drops.clone(),
            });
            commands.trigger(OpenModal(ModalType::ResultsModal));
            commands.remove_resource::<PendingVictory>();
        }
    }
}
