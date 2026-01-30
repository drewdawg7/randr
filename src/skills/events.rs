use bevy::prelude::*;

use super::SkillType;

#[derive(Event, Debug, Clone)]
pub struct SkillXpGained {
    pub skill: SkillType,
    pub amount: u64,
}

#[derive(Event, Debug, Clone)]
pub struct SkillLeveledUp {
    pub skill: SkillType,
    pub old_level: u32,
    pub new_level: u32,
}
