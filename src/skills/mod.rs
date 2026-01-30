pub mod bonuses;
pub mod events;
pub mod plugin;

use std::collections::HashMap;

use bevy::prelude::*;

pub use events::{SkillLeveledUp, SkillXpGained};
pub use plugin::SkillsPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillType {
    Blacksmith,
    Mining,
    Combat,
}

impl SkillType {
    pub fn all() -> &'static [SkillType] {
        &[SkillType::Blacksmith, SkillType::Mining, SkillType::Combat]
    }

    pub fn display_name(self) -> &'static str {
        match self {
            SkillType::Blacksmith => "Blacksmith",
            SkillType::Mining => "Mining",
            SkillType::Combat => "Combat",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub skill_type: SkillType,
    pub level: u32,
    pub xp: u64,
}

impl Skill {
    pub fn new(skill_type: SkillType) -> Self {
        Self {
            skill_type,
            level: 1,
            xp: 0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Skills {
    skills: HashMap<SkillType, Skill>,
}

impl Default for Skills {
    fn default() -> Self {
        Self::new()
    }
}

impl Skills {
    pub fn new() -> Self {
        let mut skills = HashMap::new();
        for &skill_type in SkillType::all() {
            skills.insert(skill_type, Skill::new(skill_type));
        }
        Self { skills }
    }

    pub fn skill(&self, t: SkillType) -> Option<&Skill> {
        self.skills.get(&t)
    }

    pub fn skill_mut(&mut self, t: SkillType) -> Option<&mut Skill> {
        self.skills.get_mut(&t)
    }

    pub fn player_level(&self) -> u32 {
        self.skills.values().map(|s| s.level).sum()
    }
}

pub fn xp_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }
    let mut total = 0.0;
    for n in 1..(level as i32) {
        total += (n as f64 + 300.0 * 2.0_f64.powf(n as f64 / 7.0)).floor() / 4.0;
    }
    total.floor() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_for_level() {
        assert_eq!(xp_for_level(1), 0);
        assert_eq!(xp_for_level(2), 83);
        assert_eq!(xp_for_level(10), 1154);
        assert_eq!(xp_for_level(50), 101333);
        assert_eq!(xp_for_level(99), 13034431);
    }

    #[test]
    fn test_skills_new() {
        let skills = Skills::new();
        assert_eq!(skills.skills.len(), 3);
        for &skill_type in SkillType::all() {
            let skill = skills.skill(skill_type).expect("skill should exist");
            assert_eq!(skill.level, 1);
            assert_eq!(skill.xp, 0);
        }
    }

    #[test]
    fn test_player_level() {
        let mut skills = Skills::new();
        assert_eq!(skills.player_level(), 3);

        if let Some(skill) = skills.skill_mut(SkillType::Mining) {
            skill.level = 10;
        }
        assert_eq!(skills.player_level(), 12);
    }
}
