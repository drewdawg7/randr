use crate::entities::mob::MobId;
use crate::location::mine::rock::RockId;

// ─────────────────────────────────────────────────────────────────────────────
// Passive Spell Effects (always active while tome equipped)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum PassiveEffect {
    // ─────────────────────────────────────────────────────────────────────────
    // Combat Bonuses
    // ─────────────────────────────────────────────────────────────────────────
    /// Bonus to attack stat
    BonusAttack(i32),

    /// Bonus to defense stat
    BonusDefense(i32),

    /// Regenerate HP over time (amount per turn in combat)
    Regeneration(i32),

    // ─────────────────────────────────────────────────────────────────────────
    // Loot Bonuses
    // ─────────────────────────────────────────────────────────────────────────
    /// Bonus gold find percentage
    BonusGoldFind(i32),

    /// Bonus to magic find
    BonusMagicFind(i32),

    /// Bonus XP multiplier percentage (e.g., 10 = +10% XP)
    XPMultiplier(i32),

    // ─────────────────────────────────────────────────────────────────────────
    // World/Exploration Effects
    // ─────────────────────────────────────────────────────────────────────────
    /// Reveals hidden things (e.g., secret areas, hidden enemies)
    Reveal,

    // ─────────────────────────────────────────────────────────────────────────
    // Blacksmith/Furnace System
    // ─────────────────────────────────────────────────────────────────────────
    /// Adds fuel to furnace per minute (amount per minute)
    FurnaceFuelRegen(i32),

    // ─────────────────────────────────────────────────────────────────────────
    // Field/Mob System
    // ─────────────────────────────────────────────────────────────────────────
    /// Modifies spawn weight for a specific mob type
    /// Positive values increase spawn chance, negative decrease
    MobSpawnWeight(MobId, i32),

    // ─────────────────────────────────────────────────────────────────────────
    // Mine/Rock System
    // ─────────────────────────────────────────────────────────────────────────
    /// Modifies spawn weight for a specific rock type
    RockSpawnWeight(RockId, i32),

    /// Bonus mining efficiency
    BonusMining(i32),

    // ─────────────────────────────────────────────────────────────────────────
    // Store System
    // ─────────────────────────────────────────────────────────────────────────
    /// Discount percentage on store purchases
    StoreDiscount(i32),

    // ─────────────────────────────────────────────────────────────────────────
    // Dungeon System
    // ─────────────────────────────────────────────────────────────────────────
    /// Allows bypassing dungeon rooms without combat
    DungeonBypass,

    /// Reveals all rooms in a dungeon
    DungeonReveal,
}

impl PassiveEffect {
    /// Generate a description of what this passive does
    pub fn describe(&self) -> String {
        match self {
            // Combat
            PassiveEffect::BonusAttack(amount) => format!("+{} Attack", amount),
            PassiveEffect::BonusDefense(amount) => format!("+{} Defense", amount),
            PassiveEffect::Regeneration(amount) => format!("Regenerate {} HP per turn", amount),

            // Loot
            PassiveEffect::BonusGoldFind(amount) => format!("+{}% Gold Find", amount),
            PassiveEffect::BonusMagicFind(amount) => format!("+{}% Magic Find", amount),
            PassiveEffect::XPMultiplier(amount) => format!("+{}% XP Gain", amount),

            // World
            PassiveEffect::Reveal => "Reveals hidden things".to_string(),

            // Blacksmith
            PassiveEffect::FurnaceFuelRegen(amount) => {
                format!("+{} furnace fuel per minute", amount)
            }

            // Field/Mob
            PassiveEffect::MobSpawnWeight(mob_id, weight) => {
                let sign = if *weight >= 0 { "+" } else { "" };
                format!("{}{} {:?} spawn weight", sign, weight, mob_id)
            }

            // Mine/Rock
            PassiveEffect::RockSpawnWeight(rock_id, weight) => {
                let sign = if *weight >= 0 { "+" } else { "" };
                format!("{}{} {:?} spawn weight", sign, weight, rock_id)
            }
            PassiveEffect::BonusMining(amount) => format!("+{} Mining", amount),

            // Store
            PassiveEffect::StoreDiscount(amount) => format!("{}% store discount", amount),

            // Dungeon
            PassiveEffect::DungeonBypass => "Can bypass dungeon rooms".to_string(),
            PassiveEffect::DungeonReveal => "Reveals all dungeon rooms".to_string(),
        }
    }
}
