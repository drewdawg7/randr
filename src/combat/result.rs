use crate::loot::LootDrop;
use crate::magic::word::Element;

#[derive(Debug, Clone)]
pub struct AttackResult {
    pub attacker: String,
    pub defender: String,
    pub damage_to_target: i32,
    pub target_health_before: i32,
    pub target_health_after: i32,
    pub target_died: bool,
}

/// Result of casting a spell
#[derive(Debug, Clone)]
pub enum SpellCastResult {
    /// Spell dealt damage to target
    Damage {
        spell_name: String,
        damage_dealt: i32,
        element: Element,
        target_health_before: i32,
        target_health_after: i32,
        target_died: bool,
    },
    /// Spell healed the caster
    Heal {
        spell_name: String,
        amount_healed: i32,
        caster_health_after: i32,
    },
    /// Spell dealt damage and healed caster (life drain)
    LifeDrain {
        spell_name: String,
        damage_dealt: i32,
        amount_healed: i32,
        target_health_after: i32,
        caster_health_after: i32,
        target_died: bool,
    },
    /// No spell was available to cast
    NoSpell,
    /// Spell fizzled (no effect)
    Fizzle {
        reason: String,
    },
}

/// Result of a mob dying - contains rewards for the killer
#[derive(Debug, Clone)]
pub struct MobDeathResult {
    pub gold_dropped: i32,
    pub xp_dropped: i32,
    pub loot_drops: Vec<LootDrop>,
}

impl Default for MobDeathResult {
    fn default() -> Self {
        Self {
            gold_dropped: 0,
            xp_dropped: 0,
            loot_drops: Vec::new(),
        }
    }
}

/// Result of player dying
#[derive(Debug, Clone, Default)]
pub struct PlayerDeathResult {
    pub gold_lost: i32,
}

/// Result of a rock being destroyed - contains mined items
#[derive(Debug, Clone, Default)]
pub struct RockDeathResult {
    pub drops: Vec<LootDrop>,
}
