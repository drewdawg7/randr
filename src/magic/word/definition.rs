use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// Word Identification
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WordId {
    // Elements
    Fire,
    Ice,
    Lightning,

    // Actions
    Bolt,
    Shield,
    Burst,
    Drain,

    // Modifiers
    Power,
    Swift,
    Stable,
    Chaos,

    // Utility
    Sight,
    Gold,
    Mend,
}

impl WordId {
    /// Returns the canonical string name for this word (used for text input matching)
    pub fn name(&self) -> &'static str {
        match self {
            WordId::Fire => "fire",
            WordId::Ice => "ice",
            WordId::Lightning => "lightning",
            WordId::Bolt => "bolt",
            WordId::Shield => "shield",
            WordId::Burst => "burst",
            WordId::Drain => "drain",
            WordId::Power => "power",
            WordId::Swift => "swift",
            WordId::Stable => "stable",
            WordId::Chaos => "chaos",
            WordId::Sight => "sight",
            WordId::Gold => "gold",
            WordId::Mend => "mend",
        }
    }

    /// Parse a string into a WordId (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fire" => Some(WordId::Fire),
            "ice" => Some(WordId::Ice),
            "lightning" => Some(WordId::Lightning),
            "bolt" => Some(WordId::Bolt),
            "shield" => Some(WordId::Shield),
            "burst" => Some(WordId::Burst),
            "drain" => Some(WordId::Drain),
            "power" => Some(WordId::Power),
            "swift" => Some(WordId::Swift),
            "stable" => Some(WordId::Stable),
            "chaos" => Some(WordId::Chaos),
            "sight" => Some(WordId::Sight),
            "gold" => Some(WordId::Gold),
            "mend" => Some(WordId::Mend),
            _ => None,
        }
    }

    /// Returns all known word IDs
    pub fn all() -> &'static [WordId] {
        &[
            WordId::Fire,
            WordId::Ice,
            WordId::Lightning,
            WordId::Bolt,
            WordId::Shield,
            WordId::Burst,
            WordId::Drain,
            WordId::Power,
            WordId::Swift,
            WordId::Stable,
            WordId::Chaos,
            WordId::Sight,
            WordId::Gold,
            WordId::Mend,
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Element Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Physical,
}

// ─────────────────────────────────────────────────────────────────────────────
// Word Properties (for emergent combination)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct WordProperties {
    // Numeric properties that combine additively
    pub damage: i32,
    pub defense: i32,
    pub healing: i32,
    pub stability: i32,
    pub chaos: i32,
    pub slow: i32,
    pub lifesteal: i32,
    pub speed: i32,
    pub gold_find: i32,

    // Boolean flags
    pub is_projectile: bool,
    pub is_aoe: bool,
    pub is_passive: bool,
    pub reveals: bool,

    // Element tags
    pub elements: HashSet<Element>,
}

impl WordProperties {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder methods for cleaner spec definitions
    pub fn damage(mut self, value: i32) -> Self {
        self.damage = value;
        self
    }

    pub fn defense(mut self, value: i32) -> Self {
        self.defense = value;
        self
    }

    pub fn healing(mut self, value: i32) -> Self {
        self.healing = value;
        self
    }

    pub fn stability(mut self, value: i32) -> Self {
        self.stability = value;
        self
    }

    pub fn chaos(mut self, value: i32) -> Self {
        self.chaos = value;
        self
    }

    pub fn slow(mut self, value: i32) -> Self {
        self.slow = value;
        self
    }

    pub fn lifesteal(mut self, value: i32) -> Self {
        self.lifesteal = value;
        self
    }

    pub fn speed(mut self, value: i32) -> Self {
        self.speed = value;
        self
    }

    pub fn gold_find(mut self, value: i32) -> Self {
        self.gold_find = value;
        self
    }

    pub fn projectile(mut self) -> Self {
        self.is_projectile = true;
        self
    }

    pub fn aoe(mut self) -> Self {
        self.is_aoe = true;
        self
    }

    pub fn passive(mut self) -> Self {
        self.is_passive = true;
        self
    }

    pub fn reveals(mut self) -> Self {
        self.reveals = true;
        self
    }

    pub fn element(mut self, elem: Element) -> Self {
        self.elements.insert(elem);
        self
    }

    /// Combine properties from multiple words (additive)
    pub fn combine(words: &[&WordProperties]) -> Self {
        let mut combined = WordProperties::new();

        for props in words {
            combined.damage += props.damage;
            combined.defense += props.defense;
            combined.healing += props.healing;
            combined.stability += props.stability;
            combined.chaos += props.chaos;
            combined.slow += props.slow;
            combined.lifesteal += props.lifesteal;
            combined.speed += props.speed;
            combined.gold_find += props.gold_find;

            combined.is_projectile |= props.is_projectile;
            combined.is_aoe |= props.is_aoe;
            combined.is_passive |= props.is_passive;
            combined.reveals |= props.reveals;

            combined.elements.extend(&props.elements);
        }

        combined
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Word Specification
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WordSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub properties: WordProperties,
}
