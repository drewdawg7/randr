use crate::magic::effect::{ActiveEffect, PassiveEffect};

// ─────────────────────────────────────────────────────────────────────────────
// Computed Spell (result of word combination)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum ComputedSpell {
    /// A castable spell with an active effect
    Active {
        name: String,
        description: String,
        effect: ActiveEffect,
    },

    /// A passive effect (always on while tome equipped)
    Passive {
        name: String,
        description: String,
        effect: PassiveEffect,
    },

    /// A spell with both active and passive components
    Hybrid {
        name: String,
        description: String,
        active: ActiveEffect,
        passive: PassiveEffect,
    },

    /// Invalid combination - causes backfire
    Backfire {
        reason: String,
        effect: BackfireEffect,
    },

    /// Word combination has no effect (not enough properties to form a spell)
    Fizzle {
        reason: String,
    },
}

impl ComputedSpell {
    pub fn name(&self) -> &str {
        match self {
            ComputedSpell::Active { name, .. } => name,
            ComputedSpell::Passive { name, .. } => name,
            ComputedSpell::Hybrid { name, .. } => name,
            ComputedSpell::Backfire { .. } => "Unstable Magic",
            ComputedSpell::Fizzle { .. } => "Fizzled Spell",
        }
    }

    pub fn description(&self) -> String {
        match self {
            ComputedSpell::Active { description, .. } => description.clone(),
            ComputedSpell::Passive { description, .. } => description.clone(),
            ComputedSpell::Hybrid { description, .. } => description.clone(),
            ComputedSpell::Backfire { reason, effect } => {
                format!("BACKFIRE: {} - {}", reason, effect.describe())
            }
            ComputedSpell::Fizzle { reason } => format!("The magic fizzles: {}", reason),
        }
    }

    pub fn is_castable(&self) -> bool {
        matches!(
            self,
            ComputedSpell::Active { .. } | ComputedSpell::Hybrid { .. }
        )
    }

    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            ComputedSpell::Passive { .. } | ComputedSpell::Hybrid { .. }
        )
    }

    pub fn is_backfire(&self) -> bool {
        matches!(self, ComputedSpell::Backfire { .. })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Backfire Effects
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum BackfireEffect {
    /// Damage the caster
    DamageSelf(i32),

    /// Random negative effect
    RandomCurse,

    /// Spell explodes, damaging everyone including caster
    Explosion { damage: i32 },

    /// Caster is stunned for N turns
    Stun { turns: i32 },
}

impl BackfireEffect {
    pub fn describe(&self) -> String {
        match self {
            BackfireEffect::DamageSelf(amount) => format!("Take {} damage", amount),
            BackfireEffect::RandomCurse => "Suffer a random curse".to_string(),
            BackfireEffect::Explosion { damage } => {
                format!("Explosion deals {} damage to everyone", damage)
            }
            BackfireEffect::Stun { turns } => format!("Stunned for {} turns", turns),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Spell Result (outcome of casting)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SpellResult {
    /// Spell cast successfully
    Success {
        damage_dealt: Option<i32>,
        healing_done: Option<i32>,
        effects_applied: Vec<String>,
    },

    /// Spell backfired
    Backfired {
        self_damage: i32,
        message: String,
    },

    /// No spell to cast (page empty or no active spell)
    NoSpell,
}
