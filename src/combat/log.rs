use bevy::prelude::*;

/// A single entry in the combat log.
#[derive(Debug, Clone)]
pub struct CombatLogEntry {
    pub text: String,
    pub color: Color,
}

impl CombatLogEntry {
    pub fn player_attack(damage: i32, target: &str) -> Self {
        Self {
            text: format!("You attack {} for {} damage!", target, damage),
            color: Color::srgb(0.5, 0.8, 0.5),
        }
    }

    pub fn enemy_attack(damage: i32, attacker: &str) -> Self {
        Self {
            text: format!("{} attacks you for {} damage!", attacker, damage),
            color: Color::srgb(0.8, 0.5, 0.5),
        }
    }

    pub fn enemy_defeated(target: &str) -> Self {
        Self {
            text: format!("{} has been defeated!", target),
            color: Color::srgb(0.9, 0.9, 0.3),
        }
    }

    pub fn player_defeated() -> Self {
        Self {
            text: "You have been defeated...".to_string(),
            color: Color::srgb(0.8, 0.3, 0.3),
        }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::srgb(0.7, 0.7, 0.7),
        }
    }
}
