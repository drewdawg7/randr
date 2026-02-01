use bevy::prelude::*;

use super::events::{SkillLeveledUp, SkillXpGained};
use super::{xp_for_level, Skills};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Skills::new())
            .add_event::<SkillXpGained>()
            .add_event::<SkillLeveledUp>()
            .add_systems(Update, process_xp_gained.run_if(on_event::<SkillXpGained>));
    }
}

fn process_xp_gained(
    mut events: MessageReader<SkillXpGained>,
    mut skills: ResMut<Skills>,
    mut level_up_events: MessageWriter<SkillLeveledUp>,
) {
    for event in events.read() {
        let Some(skill) = skills.skill_mut(event.skill) else {
            continue;
        };

        let old_level = skill.level;
        skill.xp += event.amount;

        while skill.xp >= xp_for_level(skill.level + 1) {
            skill.level += 1;
        }

        if skill.level > old_level {
            level_up_events.write(SkillLeveledUp {
                skill: event.skill,
                old_level,
                new_level: skill.level,
            });
        }
    }
}
