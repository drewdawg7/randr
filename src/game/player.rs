use bevy::prelude::*;

use crate::entities::Progression;
use crate::item::ItemRegistry;
use crate::player::{PlayerBundle, PlayerMarker};
use crate::states::AppState;
use crate::stats::StatSheet;

#[derive(Message, Debug, Clone)]
pub struct PlayerDamaged {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

#[derive(Message, Debug, Clone)]
pub struct PlayerHealed {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

#[derive(Message, Debug, Clone)]
pub struct PlayerLeveledUp {
    pub new_level: u32,
    pub old_level: u32,
}

#[derive(Message, Debug, Clone)]
pub struct GoldChanged {
    pub amount: i32,
    pub new_total: i32,
}

#[derive(Component, Default)]
pub struct PlayerPreviousLevel(pub i32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerDamaged>()
            .add_message::<PlayerHealed>()
            .add_message::<PlayerLeveledUp>()
            .add_message::<GoldChanged>()
            .add_systems(
                OnEnter(AppState::Menu),
                spawn_player_entity.run_if(not(any_with_component::<PlayerMarker>)),
            )
            .add_systems(Update, handle_level_up);
    }
}

fn spawn_player_entity(mut commands: Commands, registry: Res<ItemRegistry>) {
    commands.spawn(PlayerBundle::new(&registry));
}

fn handle_level_up(
    mut player: Query<
        (&Progression, &mut StatSheet, &mut PlayerPreviousLevel),
        (With<PlayerMarker>, Changed<Progression>),
    >,
    mut level_up_events: MessageWriter<PlayerLeveledUp>,
) {
    let Ok((progression, mut stats, mut prev_level)) = player.single_mut() else {
        return;
    };

    let current_level = progression.level;
    if current_level > prev_level.0 {
        for level in (prev_level.0 + 1)..=current_level {
            if level % 10 == 0 {
                stats.increase_stat(crate::stats::StatType::Defense, 1);
            }
            stats.increase_stat(crate::stats::StatType::Health, 5);
            stats.increase_stat_max(crate::stats::StatType::Health, 5);
            stats.increase_stat(crate::stats::StatType::Attack, 1);

            level_up_events.write(PlayerLeveledUp {
                new_level: level as u32,
                old_level: (level - 1) as u32,
            });
        }
        prev_level.0 = current_level;
    }
}
