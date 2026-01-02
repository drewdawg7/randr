use rand::Rng;

/// Represents an attack with variable damage range.
/// Damage is rolled randomly between min_damage and max_damage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attack {
    /// Minimum damage this attack can deal (before defense)
    pub min_damage: i32,
    /// Maximum damage this attack can deal (before defense)
    pub max_damage: i32,
}

impl Attack {
    /// Create a new Attack with explicit min/max damage values
    pub fn new(min_damage: i32, max_damage: i32) -> Self {
        Self { min_damage, max_damage }
    }

    /// Roll a random damage value within the attack's range
    pub fn roll_damage(&self) -> i32 {
        if self.min_damage >= self.max_damage {
            return self.min_damage;
        }
        let mut rng = rand::thread_rng();
        rng.gen_range(self.min_damage..=self.max_damage)
    }

    /// Get the average damage (useful for UI display)
    pub fn average(&self) -> i32 {
        (self.min_damage + self.max_damage) / 2
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            min_damage: 1,
            max_damage: 1,
        }
    }
}
