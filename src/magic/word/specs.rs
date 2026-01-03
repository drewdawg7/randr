use crate::registry::RegistryDefaults;

use super::definition::{Element, WordId, WordProperties, WordSpec};

impl RegistryDefaults<WordId> for WordSpec {
    fn defaults() -> impl IntoIterator<Item = (WordId, Self)> {
        vec![
            // ─────────────────────────────────────────────────────────────────
            // Elements
            // ─────────────────────────────────────────────────────────────────
            (
                WordId::Fire,
                WordSpec {
                    name: "Fire",
                    description: "Adds fire damage to spells",
                    properties: WordProperties::new()
                        .damage(5)
                        .element(Element::Fire),
                },
            ),
            (
                WordId::Ice,
                WordSpec {
                    name: "Ice",
                    description: "Adds ice damage and slowing effect",
                    properties: WordProperties::new()
                        .damage(3)
                        .slow(2)
                        .element(Element::Ice),
                },
            ),
            (
                WordId::Lightning,
                WordSpec {
                    name: "Lightning",
                    description: "High damage but less stable",
                    properties: WordProperties::new()
                        .damage(7)
                        .chaos(2)
                        .element(Element::Lightning),
                },
            ),
            // ─────────────────────────────────────────────────────────────────
            // Actions
            // ─────────────────────────────────────────────────────────────────
            (
                WordId::Bolt,
                WordSpec {
                    name: "Bolt",
                    description: "Single target projectile attack",
                    properties: WordProperties::new().damage(3).projectile(),
                },
            ),
            (
                WordId::Shield,
                WordSpec {
                    name: "Shield",
                    description: "Defensive buff",
                    properties: WordProperties::new().defense(5),
                },
            ),
            (
                WordId::Burst,
                WordSpec {
                    name: "Burst",
                    description: "Area effect attack",
                    properties: WordProperties::new().damage(2).aoe(),
                },
            ),
            (
                WordId::Drain,
                WordSpec {
                    name: "Drain",
                    description: "Damage and heal self",
                    properties: WordProperties::new().damage(2).lifesteal(3),
                },
            ),
            // ─────────────────────────────────────────────────────────────────
            // Modifiers
            // ─────────────────────────────────────────────────────────────────
            (
                WordId::Power,
                WordSpec {
                    name: "Power",
                    description: "Amplifies damage",
                    properties: WordProperties::new().damage(4),
                },
            ),
            (
                WordId::Swift,
                WordSpec {
                    name: "Swift",
                    description: "Faster but slightly weaker",
                    properties: WordProperties::new().speed(2).damage(-1),
                },
            ),
            (
                WordId::Stable,
                WordSpec {
                    name: "Stable",
                    description: "Reduces backfire chance",
                    properties: WordProperties::new().stability(3),
                },
            ),
            (
                WordId::Chaos,
                WordSpec {
                    name: "Chaos",
                    description: "High variance, high risk",
                    properties: WordProperties::new().chaos(5).damage(3),
                },
            ),
            // ─────────────────────────────────────────────────────────────────
            // Utility
            // ─────────────────────────────────────────────────────────────────
            (
                WordId::Sight,
                WordSpec {
                    name: "Sight",
                    description: "Passive: reveals hidden things",
                    properties: WordProperties::new().passive().reveals(),
                },
            ),
            (
                WordId::Gold,
                WordSpec {
                    name: "Gold",
                    description: "Passive: bonus gold find",
                    properties: WordProperties::new().passive().gold_find(10),
                },
            ),
            (
                WordId::Mend,
                WordSpec {
                    name: "Mend",
                    description: "Healing effect",
                    properties: WordProperties::new().healing(8),
                },
            ),
        ]
    }
}
