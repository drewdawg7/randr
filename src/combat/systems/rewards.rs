use bevy::prelude::*;

use crate::combat::events::{GoldGained, XpGained};
use crate::entities::Progression;
use crate::player::PlayerGold;
use crate::skills::{SkillType, SkillXpGained};

pub fn apply_gold_gain(mut events: MessageReader<GoldGained>, mut gold: ResMut<PlayerGold>) {
    for event in events.read() {
        gold.add(event.amount);
    }
}

pub fn apply_xp_gain(
    mut events: MessageReader<XpGained>,
    mut progression: ResMut<Progression>,
    mut skill_writer: MessageWriter<SkillXpGained>,
) {
    for event in events.read() {
        progression.add_xp(event.amount);
        skill_writer.write(SkillXpGained {
            skill: SkillType::Combat,
            amount: event.amount as u64,
        });
    }
}
