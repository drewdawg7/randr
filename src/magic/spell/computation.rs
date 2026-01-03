use std::collections::HashSet;

use crate::magic::effect::{ActiveEffect, PassiveEffect};
use crate::magic::word::{Element, WordId, WordProperties, WordSpec};
use crate::registry::Registry;

use super::definition::ComputedSpell;
use super::recipes::{find_invalid_combo, find_recipe};

// ─────────────────────────────────────────────────────────────────────────────
// Spell Computation
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the spell effect from a set of words
pub fn compute_spell(words: &[WordId], registry: &Registry<WordId, WordSpec>) -> ComputedSpell {
    if words.is_empty() {
        return ComputedSpell::Fizzle {
            reason: "No words inscribed".to_string(),
        };
    }

    if words.len() > 5 {
        return ComputedSpell::Fizzle {
            reason: "Too many words (max 5)".to_string(),
        };
    }

    let word_set: HashSet<WordId> = words.iter().copied().collect();

    // 1. Check for hardcoded recipes first (exact match)
    if let Some(recipe) = find_recipe(&word_set) {
        return recipe.effect.clone();
    }

    // 2. Check for invalid combinations (backfire)
    if let Some(invalid) = find_invalid_combo(&word_set) {
        return ComputedSpell::Backfire {
            reason: invalid.reason.to_string(),
            effect: invalid.backfire.clone(),
        };
    }

    // 3. Compute emergent effect from combined properties
    let props: Vec<&WordProperties> = words
        .iter()
        .filter_map(|w| registry.get(w).map(|spec| &spec.properties))
        .collect();

    if props.is_empty() {
        return ComputedSpell::Fizzle {
            reason: "Unknown words".to_string(),
        };
    }

    let combined = WordProperties::combine(&props);

    // 4. Determine spell type from combined properties
    compute_from_properties(&combined, &word_set)
}

/// Determine the spell effect based on combined properties
fn compute_from_properties(props: &WordProperties, words: &HashSet<WordId>) -> ComputedSpell {
    // Generate a name from the words
    let name = generate_spell_name(words);

    // Determine if this is primarily passive, active, or hybrid
    let has_passive_component = props.is_passive || props.gold_find > 0 || props.reveals;
    let has_active_component =
        props.damage > 0 || props.healing > 0 || props.defense > 0 || props.lifesteal > 0;

    // Pure passive spell
    if has_passive_component && !has_active_component {
        let effect = determine_passive_effect(props);
        return ComputedSpell::Passive {
            name,
            description: effect.describe(),
            effect,
        };
    }

    // Active spell (with or without passive component)
    if has_active_component {
        let active = determine_active_effect(props);

        if has_passive_component {
            let passive = determine_passive_effect(props);
            return ComputedSpell::Hybrid {
                name,
                description: format!("{}. Passive: {}", active.describe(), passive.describe()),
                active,
                passive,
            };
        }

        return ComputedSpell::Active {
            name,
            description: active.describe(),
            effect: active,
        };
    }

    // Not enough properties to form a spell
    ComputedSpell::Fizzle {
        reason: "Insufficient magical properties".to_string(),
    }
}

/// Generate a spell name from the words used
fn generate_spell_name(words: &HashSet<WordId>) -> String {
    // Simple naming: concatenate word names
    let mut names: Vec<&str> = words.iter().map(|w| w.name()).collect();
    names.sort(); // Consistent ordering

    if names.len() == 1 {
        capitalize(names[0])
    } else if names.len() == 2 {
        format!("{} {}", capitalize(names[0]), capitalize(names[1]))
    } else {
        // For 3+ words, use "X of Y and Z" style
        // Use pattern matching to safely handle edge cases
        if let (Some(last), Some(second_last)) = (names.pop(), names.pop()) {
            let prefix: Vec<&str> = names.iter().copied().collect();

            if prefix.is_empty() {
                format!("{} and {}", capitalize(second_last), capitalize(last))
            } else {
                format!(
                    "{} of {} and {}",
                    prefix.iter().map(|s| capitalize(s)).collect::<Vec<_>>().join(" "),
                    capitalize(second_last),
                    capitalize(last)
                )
            }
        } else {
            // Fallback if somehow we don't have enough elements
            "Unknown Spell".to_string()
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Determine the active effect from combined properties
fn determine_active_effect(props: &WordProperties) -> ActiveEffect {
    // Determine primary element
    let element = if props.elements.contains(&Element::Fire) {
        Element::Fire
    } else if props.elements.contains(&Element::Ice) {
        Element::Ice
    } else if props.elements.contains(&Element::Lightning) {
        Element::Lightning
    } else {
        Element::Physical
    };

    // Priority: Lifesteal > Healing > Damage
    if props.lifesteal > 0 && props.damage > 0 {
        return ActiveEffect::LifeDrain {
            damage: props.damage,
            heal_percent: props.lifesteal * 10, // lifesteal value * 10%
        };
    }

    if props.healing > 0 && props.damage == 0 {
        return ActiveEffect::Heal {
            amount: props.healing,
        };
    }

    if props.defense > 0 && props.damage == 0 {
        return ActiveEffect::DefenseBuff {
            amount: props.defense,
            duration: 3,
        };
    }

    // Damage spell - AoE or single target
    if props.is_aoe {
        let mut effect = ActiveEffect::AreaDamage {
            amount: props.damage.max(1),
            element,
        };

        // Add slow if present
        if props.slow > 0 {
            effect = ActiveEffect::DamageWithEffect {
                damage: props.damage.max(1),
                element,
                secondary: Box::new(ActiveEffect::Slow {
                    amount: props.slow,
                    duration: 2,
                }),
            };
        }

        return effect;
    }

    // Single target damage
    let base = ActiveEffect::Damage {
        amount: props.damage.max(1),
        element,
    };

    // Add slow if present
    if props.slow > 0 {
        return ActiveEffect::DamageWithEffect {
            damage: props.damage.max(1),
            element,
            secondary: Box::new(ActiveEffect::Slow {
                amount: props.slow,
                duration: 2,
            }),
        };
    }

    // Add defense buff if present
    if props.defense > 0 {
        return ActiveEffect::DamageWithEffect {
            damage: props.damage.max(1),
            element,
            secondary: Box::new(ActiveEffect::DefenseBuff {
                amount: props.defense,
                duration: 2,
            }),
        };
    }

    base
}

/// Determine the passive effect from combined properties
fn determine_passive_effect(props: &WordProperties) -> PassiveEffect {
    // Priority order for passive effects
    if props.reveals {
        return PassiveEffect::Reveal;
    }

    if props.gold_find > 0 {
        return PassiveEffect::BonusGoldFind(props.gold_find);
    }

    if props.defense > 0 {
        return PassiveEffect::BonusDefense(props.defense);
    }

    // Default passive
    PassiveEffect::BonusAttack(0)
}
