// ─────────────────────────────────────────────────────────────────────────────
// Passive Spell Effects (always active while tome equipped)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum PassiveEffect {
    /// Bonus to attack stat
    BonusAttack(i32),

    /// Bonus to defense stat
    BonusDefense(i32),

    /// Bonus gold find percentage
    BonusGoldFind(i32),

    /// Reveals hidden things (e.g., secret areas, hidden enemies)
    Reveal,

    /// Regenerate HP over time (amount per turn in combat)
    Regeneration(i32),

    /// Bonus to magic find
    BonusMagicFind(i32),
}

impl PassiveEffect {
    /// Generate a description of what this passive does
    pub fn describe(&self) -> String {
        match self {
            PassiveEffect::BonusAttack(amount) => format!("+{} Attack", amount),
            PassiveEffect::BonusDefense(amount) => format!("+{} Defense", amount),
            PassiveEffect::BonusGoldFind(amount) => format!("+{}% Gold Find", amount),
            PassiveEffect::Reveal => "Reveals hidden things".to_string(),
            PassiveEffect::Regeneration(amount) => format!("Regenerate {} HP per turn", amount),
            PassiveEffect::BonusMagicFind(amount) => format!("+{}% Magic Find", amount),
        }
    }
}
