use crate::magic::word::Element;

// ─────────────────────────────────────────────────────────────────────────────
// Active Spell Effects (cast during combat)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum ActiveEffect {
    /// Direct damage to target
    Damage {
        amount: i32,
        element: Element,
    },

    /// Damage with lifesteal (heal caster for percentage of damage)
    LifeDrain {
        damage: i32,
        heal_percent: i32,
    },

    /// Heal the caster
    Heal {
        amount: i32,
    },

    /// Temporary defense buff
    DefenseBuff {
        amount: i32,
        duration: i32,
    },

    /// Area damage (hits all enemies if multiple)
    AreaDamage {
        amount: i32,
        element: Element,
    },

    /// Slow the target (reduce their speed/attack frequency)
    Slow {
        amount: i32,
        duration: i32,
    },

    /// Combined effect (damage + secondary)
    DamageWithEffect {
        damage: i32,
        element: Element,
        secondary: Box<ActiveEffect>,
    },
}

impl ActiveEffect {
    /// Generate a description of what this effect does
    pub fn describe(&self) -> String {
        match self {
            ActiveEffect::Damage { amount, element } => {
                format!("Deal {} {:?} damage", amount, element)
            }
            ActiveEffect::LifeDrain {
                damage,
                heal_percent,
            } => {
                format!(
                    "Deal {} damage and heal for {}% of damage dealt",
                    damage, heal_percent
                )
            }
            ActiveEffect::Heal { amount } => {
                format!("Heal for {} HP", amount)
            }
            ActiveEffect::DefenseBuff { amount, duration } => {
                format!("+{} defense for {} turns", amount, duration)
            }
            ActiveEffect::AreaDamage { amount, element } => {
                format!("Deal {} {:?} damage to all enemies", amount, element)
            }
            ActiveEffect::Slow { amount, duration } => {
                format!("Slow target by {} for {} turns", amount, duration)
            }
            ActiveEffect::DamageWithEffect {
                damage,
                element,
                secondary,
            } => {
                format!(
                    "Deal {} {:?} damage. {}",
                    damage,
                    element,
                    secondary.describe()
                )
            }
        }
    }
}
