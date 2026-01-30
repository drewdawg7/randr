pub struct BlacksmithBonuses {
    pub quality_bonus: i32,
    pub speed_multiplier: f32,
    pub bonus_item_chance: f32,
}

pub struct MiningBonuses {
    pub speed_multiplier: f32,
    pub gem_chance: f32,
}

pub struct CombatBonuses {
    pub attack_bonus: i32,
    pub defense_bonus: i32,
}

impl BlacksmithBonuses {
    pub fn from_level(level: u32) -> Self {
        Self {
            quality_bonus: (level / 5) as i32,
            speed_multiplier: 1.0 - (level.min(50) as f32 * 0.01),
            bonus_item_chance: level as f32 * 0.005,
        }
    }
}

impl MiningBonuses {
    pub fn from_level(level: u32) -> Self {
        Self {
            speed_multiplier: 1.0 - (level.min(50) as f32 * 0.01),
            gem_chance: level as f32 * 0.002,
        }
    }
}

impl CombatBonuses {
    pub fn from_level(level: u32) -> Self {
        Self {
            attack_bonus: (level / 3) as i32,
            defense_bonus: (level / 5) as i32,
        }
    }
}

pub fn combat_attack_bonus(level: u32) -> i32 {
    CombatBonuses::from_level(level).attack_bonus
}

pub fn combat_defense_bonus(level: u32) -> i32 {
    CombatBonuses::from_level(level).defense_bonus
}

pub fn blacksmith_quality_bonus(level: u32) -> i32 {
    BlacksmithBonuses::from_level(level).quality_bonus
}

pub fn blacksmith_speed_multiplier(level: u32) -> f32 {
    BlacksmithBonuses::from_level(level).speed_multiplier
}

pub fn blacksmith_bonus_item_chance(level: u32) -> f32 {
    BlacksmithBonuses::from_level(level).bonus_item_chance
}

pub fn mining_gem_chance(level: u32) -> f32 {
    MiningBonuses::from_level(level).gem_chance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_bonuses() {
        assert_eq!(combat_attack_bonus(1), 0);
        assert_eq!(combat_attack_bonus(3), 1);
        assert_eq!(combat_attack_bonus(99), 33);

        assert_eq!(combat_defense_bonus(1), 0);
        assert_eq!(combat_defense_bonus(5), 1);
        assert_eq!(combat_defense_bonus(99), 19);
    }

    #[test]
    fn test_blacksmith_bonuses() {
        assert_eq!(blacksmith_quality_bonus(1), 0);
        assert_eq!(blacksmith_quality_bonus(5), 1);
        assert_eq!(blacksmith_quality_bonus(99), 19);

        let speed_1 = blacksmith_speed_multiplier(1);
        assert!((speed_1 - 0.99).abs() < 0.001);

        let speed_50 = blacksmith_speed_multiplier(50);
        assert!((speed_50 - 0.50).abs() < 0.001);

        let speed_99 = blacksmith_speed_multiplier(99);
        assert!((speed_99 - 0.50).abs() < 0.001);

        let chance_1 = blacksmith_bonus_item_chance(1);
        assert!((chance_1 - 0.005).abs() < 0.001);

        let chance_99 = blacksmith_bonus_item_chance(99);
        assert!((chance_99 - 0.495).abs() < 0.001);
    }

    #[test]
    fn test_mining_bonuses() {
        let gem_1 = mining_gem_chance(1);
        assert!((gem_1 - 0.002).abs() < 0.001);

        let gem_99 = mining_gem_chance(99);
        assert!((gem_99 - 0.198).abs() < 0.001);
    }
}
