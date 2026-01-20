//! Word definitions using the entity_macros system
//!
//! This file consolidates:
//! - WordSpec struct definition
//! - WordId enum
//! - WordProperties and Element types
//! - All word spec constants
//! - The spec() method on WordId

use std::collections::HashSet;

use crate::registry::RegistryDefaults;

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
// WordSpec and WordId via entity_macros
// ─────────────────────────────────────────────────────────────────────────────

entity_macros::define_data! {
    spec WordSpec {
        pub name: &'static str,
        pub description: &'static str,
        pub properties: WordProperties,
    }

    id WordId;

    variants {
        // ─────────────────────────────────────────────────────────────────────
        // Elements
        // ─────────────────────────────────────────────────────────────────────
        Fire {
            name: "Fire",
            description: "Adds fire damage to spells",
            properties: WordProperties::new()
                .damage(5)
                .element(Element::Fire),
        }
        Ice {
            name: "Ice",
            description: "Adds ice damage and slowing effect",
            properties: WordProperties::new()
                .damage(3)
                .slow(2)
                .element(Element::Ice),
        }
        Lightning {
            name: "Lightning",
            description: "High damage but less stable",
            properties: WordProperties::new()
                .damage(7)
                .chaos(2)
                .element(Element::Lightning),
        }
        Earth {
            name: "Earth",
            description: "Grounding element, stability",
            properties: WordProperties::new()
                .stability(2)
                .defense(2),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Actions
        // ─────────────────────────────────────────────────────────────────────
        Bolt {
            name: "Bolt",
            description: "Single target projectile attack",
            properties: WordProperties::new().damage(3).projectile(),
        }
        Shield {
            name: "Shield",
            description: "Defensive buff",
            properties: WordProperties::new().defense(5),
        }
        Burst {
            name: "Burst",
            description: "Area effect attack",
            properties: WordProperties::new().damage(2).aoe(),
        }
        Drain {
            name: "Drain",
            description: "Damage and heal self",
            properties: WordProperties::new().damage(2).lifesteal(3),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Modifiers
        // ─────────────────────────────────────────────────────────────────────
        Power {
            name: "Power",
            description: "Amplifies damage",
            properties: WordProperties::new().damage(4),
        }
        Swift {
            name: "Swift",
            description: "Faster but slightly weaker",
            properties: WordProperties::new().speed(2).damage(-1),
        }
        Stable {
            name: "Stable",
            description: "Reduces backfire chance",
            properties: WordProperties::new().stability(3),
        }
        Chaos {
            name: "Chaos",
            description: "High variance, high risk",
            properties: WordProperties::new().chaos(5).damage(3),
        }
        Luck {
            name: "Luck",
            description: "Fortune and chance",
            properties: WordProperties::new().passive(),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Utility
        // ─────────────────────────────────────────────────────────────────────
        Sight {
            name: "Sight",
            description: "Passive: reveals hidden things",
            properties: WordProperties::new().passive().reveals(),
        }
        Gold {
            name: "Gold",
            description: "Passive: bonus gold find",
            properties: WordProperties::new().passive().gold_find(10),
        }
        Mend {
            name: "Mend",
            description: "Healing effect",
            properties: WordProperties::new().healing(8),
        }
        Find {
            name: "Find",
            description: "Enhances discovery and finding",
            properties: WordProperties::new().passive().gold_find(5),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Environment/Location Words
        // ─────────────────────────────────────────────────────────────────────
        Rock {
            name: "Rock",
            description: "Stone and mineral essence",
            properties: WordProperties::new().stability(1),
        }
        Time {
            name: "Time",
            description: "Temporal manipulation",
            properties: WordProperties::new().passive(),
        }
        Heat {
            name: "Heat",
            description: "Thermal energy, warmth",
            properties: WordProperties::new()
                .element(Element::Fire),
        }
        Field {
            name: "Field",
            description: "Open grassland, hunting grounds",
            properties: WordProperties::new().passive(),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Concept Words
        // ─────────────────────────────────────────────────────────────────────
        Pass {
            name: "Pass",
            description: "Movement, bypassing obstacles",
            properties: WordProperties::new().passive(),
        }
        Safe {
            name: "Safe",
            description: "Protection, security",
            properties: WordProperties::new().passive().defense(1),
        }

        // ─────────────────────────────────────────────────────────────────────
        // Creature Words (for spawn weight modifiers)
        // ─────────────────────────────────────────────────────────────────────
        Cow {
            name: "Cow",
            description: "Bovine essence, grazing creature",
            properties: WordProperties::new().passive(),
        }
        Slime {
            name: "Slime",
            description: "Gelatinous creature essence",
            properties: WordProperties::new().passive(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RegistryDefaults
// ─────────────────────────────────────────────────────────────────────────────

impl RegistryDefaults<WordId> for WordSpec {
    fn defaults() -> impl IntoIterator<Item = (WordId, Self)> {
        WordId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional WordId Methods
// ─────────────────────────────────────────────────────────────────────────────

impl WordId {
    /// Returns the canonical string name for this word (used for text input matching)
    pub fn name(&self) -> &'static str {
        self.spec().name.to_lowercase().leak()
    }

    /// Parse a string into a WordId (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        let lower = s.to_lowercase();
        WordId::ALL.iter().find(|id| id.spec().name.to_lowercase() == lower).copied()
    }

    /// Returns all known word IDs (alias for ALL)
    pub fn all() -> &'static [WordId] {
        WordId::ALL
    }
}
