use bevy::prelude::*;

use crate::combat::events::{GoldGained, XpGained};
use crate::entities::Progression;
use crate::player::{PlayerGold, PlayerMarker};
use crate::skills::{SkillType, SkillXpGained};

pub fn apply_gold_gain(
    mut events: MessageReader<GoldGained>,
    mut player: Query<&mut PlayerGold, With<PlayerMarker>>,
) {
    let Ok(mut gold) = player.single_mut() else {
        return;
    };
    for event in events.read() {
        gold.add(event.amount);
    }
}

pub fn apply_xp_gain(
    mut events: MessageReader<XpGained>,
    mut player: Query<&mut Progression, With<PlayerMarker>>,
    mut skill_writer: MessageWriter<SkillXpGained>,
) {
    let Ok(mut progression) = player.single_mut() else {
        return;
    };
    for event in events.read() {
        progression.add_xp(event.amount);
        skill_writer.write(SkillXpGained {
            skill: SkillType::Combat,
            amount: event.amount as u64,
        });
    }
}
