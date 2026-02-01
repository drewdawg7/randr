use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::combat::{EntityDied, PlayerAttackMob, VictoryAchieved};
use crate::input::{GameAction, NavigationDirection};
use crate::mob::Health;
use crate::ui::modal_registry::ModalCommands;
use crate::ui::{PlayerAttackTimer, PlayerSpriteSheet, SelectionState, SpriteAnimation};

use super::super::modal::{ActiveModal, ModalType, OpenModal};
use super::super::results_modal::{ResultsModalData, ResultsSprite};
use super::state::{FightModal, FightModalButton, FightModalButtonSelection, FightModalMob};

struct OpenResultsModalCommand(ResultsModalData);

impl Command for OpenResultsModalCommand {
    fn apply(self, world: &mut World) {
        world.insert_resource(self.0);
        world.trigger(OpenModal(ModalType::ResultsModal));
    }
}

pub fn handle_fight_modal_close(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
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
    mut action_reader: MessageReader<GameAction>,
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
    mut action_reader: MessageReader<GameAction>,
    mut attack_events: MessageWriter<PlayerAttackMob>,
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

                attack_events.write(PlayerAttackMob {
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
    mut events: MessageReader<PlayerAttackMob>,
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
    mut death_events: MessageReader<EntityDied>,
    mut victory_events: MessageReader<VictoryAchieved>,
    fight_mob: Option<Res<FightModalMob>>,
) {
    for event in death_events.read() {
        if event.is_player && fight_mob.is_some() {
            commands.close_modal::<FightModal>();
        }
    }

    for victory in victory_events.read() {
        if let Some(ref fight_mob) = fight_mob {
            commands.close_modal::<FightModal>();

            commands.queue(OpenResultsModalCommand(ResultsModalData {
                title: "Victory!".to_string(),
                subtitle: Some(victory.mob_name.clone()),
                sprite: Some(ResultsSprite::Mob(fight_mob.mob_id)),
                gold_gained: Some(victory.gold_gained),
                xp_gained: Some(victory.xp_gained),
                loot_drops: victory.loot_drops.clone(),
            }));
        }
    }
}
