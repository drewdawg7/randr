use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::mob::MobId;
use crate::magic::effect::{ActiveEffect, PassiveEffect};
use crate::magic::word::{Element, WordId};

use super::definition::{BackfireEffect, ComputedSpell};

// ─────────────────────────────────────────────────────────────────────────────
// Hardcoded Recipes
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Recipe {
    /// The words required for this recipe (exact match)
    pub required_words: HashSet<WordId>,
    /// The resulting spell
    pub effect: ComputedSpell,
}

impl Recipe {
    /// Check if a set of words matches this recipe exactly
    pub fn matches(&self, words: &HashSet<WordId>) -> bool {
        self.required_words == *words
    }
}

/// Hardcoded spell recipes - specific word combinations that produce designed spells
pub static RECIPES: Lazy<Vec<Recipe>> = Lazy::new(|| {
    vec![
        // Fire + Bolt = Firebolt
        Recipe {
            required_words: HashSet::from([WordId::Fire, WordId::Bolt]),
            effect: ComputedSpell::Active {
                name: "Firebolt".to_string(),
                description: "A blazing projectile of fire".to_string(),
                effect: ActiveEffect::Damage {
                    amount: 15,
                    element: Element::Fire,
                },
            },
        },
        // Ice + Shield = Frost Ward
        Recipe {
            required_words: HashSet::from([WordId::Ice, WordId::Shield]),
            effect: ComputedSpell::Active {
                name: "Frost Ward".to_string(),
                description: "A protective barrier of ice that slows attackers".to_string(),
                effect: ActiveEffect::DamageWithEffect {
                    damage: 0,
                    element: Element::Ice,
                    secondary: Box::new(ActiveEffect::DefenseBuff {
                        amount: 8,
                        duration: 3,
                    }),
                },
            },
        },
        // Lightning + Burst = Thunder Nova
        Recipe {
            required_words: HashSet::from([WordId::Lightning, WordId::Burst]),
            effect: ComputedSpell::Active {
                name: "Thunder Nova".to_string(),
                description: "A devastating burst of lightning in all directions".to_string(),
                effect: ActiveEffect::AreaDamage {
                    amount: 12,
                    element: Element::Lightning,
                },
            },
        },
        // Drain + Power = Life Siphon
        Recipe {
            required_words: HashSet::from([WordId::Drain, WordId::Power]),
            effect: ComputedSpell::Active {
                name: "Life Siphon".to_string(),
                description: "Drains life force from the target".to_string(),
                effect: ActiveEffect::LifeDrain {
                    damage: 10,
                    heal_percent: 50,
                },
            },
        },
        // Mend + Power = Greater Heal
        Recipe {
            required_words: HashSet::from([WordId::Mend, WordId::Power]),
            effect: ComputedSpell::Active {
                name: "Greater Heal".to_string(),
                description: "A powerful healing spell".to_string(),
                effect: ActiveEffect::Heal { amount: 20 },
            },
        },
        // Fire + Burst = Inferno
        Recipe {
            required_words: HashSet::from([WordId::Fire, WordId::Burst]),
            effect: ComputedSpell::Active {
                name: "Inferno".to_string(),
                description: "Engulfs the area in flames".to_string(),
                effect: ActiveEffect::AreaDamage {
                    amount: 10,
                    element: Element::Fire,
                },
            },
        },
        // Ice + Bolt = Frostbolt
        Recipe {
            required_words: HashSet::from([WordId::Ice, WordId::Bolt]),
            effect: ComputedSpell::Active {
                name: "Frostbolt".to_string(),
                description: "A chilling projectile that slows the target".to_string(),
                effect: ActiveEffect::DamageWithEffect {
                    damage: 10,
                    element: Element::Ice,
                    secondary: Box::new(ActiveEffect::Slow {
                        amount: 3,
                        duration: 2,
                    }),
                },
            },
        },
        // ─────────────────────────────────────────────────────────────────────────
        // Passive Recipes (System-Tied Effects)
        // ─────────────────────────────────────────────────────────────────────────
        // Heat + Rock + Time = Furnace Fuel Regen (+1 per minute)
        Recipe {
            required_words: HashSet::from([WordId::Heat, WordId::Rock, WordId::Time]),
            effect: ComputedSpell::Passive {
                name: "Eternal Ember".to_string(),
                description: "The furnace slowly regenerates fuel over time".to_string(),
                effect: PassiveEffect::FurnaceFuelRegen(1),
            },
        },
        // Field + Cow + Luck = Increase Cow spawn weight
        Recipe {
            required_words: HashSet::from([WordId::Field, WordId::Cow, WordId::Luck]),
            effect: ComputedSpell::Passive {
                name: "Bovine Blessing".to_string(),
                description: "Cows appear more frequently in the field".to_string(),
                effect: PassiveEffect::MobSpawnWeight(MobId::Cow, 5),
            },
        },
        // Field + Slime + Luck = Increase Slime spawn weight
        Recipe {
            required_words: HashSet::from([WordId::Field, WordId::Slime, WordId::Luck]),
            effect: ComputedSpell::Passive {
                name: "Slime Magnet".to_string(),
                description: "Slimes appear more frequently in the field".to_string(),
                effect: PassiveEffect::MobSpawnWeight(MobId::Slime, 5),
            },
        },
        // Dungeon + Pass + Safe = Dungeon Bypass
        Recipe {
            required_words: HashSet::from([WordId::Dungeon, WordId::Pass, WordId::Safe]),
            effect: ComputedSpell::Passive {
                name: "Shadow Step".to_string(),
                description: "You can bypass dungeon rooms without combat".to_string(),
                effect: PassiveEffect::DungeonBypass,
            },
        },
        // Dungeon + Sight = Dungeon Reveal
        Recipe {
            required_words: HashSet::from([WordId::Dungeon, WordId::Sight]),
            effect: ComputedSpell::Passive {
                name: "Dungeon Sight".to_string(),
                description: "All rooms in dungeons are revealed".to_string(),
                effect: PassiveEffect::DungeonReveal,
            },
        },
        // Gold + Find + Power = Bonus Gold Find (+15%)
        Recipe {
            required_words: HashSet::from([WordId::Gold, WordId::Find, WordId::Power]),
            effect: ComputedSpell::Passive {
                name: "Midas Touch".to_string(),
                description: "Significantly increased gold find".to_string(),
                effect: PassiveEffect::BonusGoldFind(15),
            },
        },
        // Gold + Find = Bonus Gold Find (+5%)
        Recipe {
            required_words: HashSet::from([WordId::Gold, WordId::Find]),
            effect: ComputedSpell::Passive {
                name: "Lucky Coin".to_string(),
                description: "Increased gold find".to_string(),
                effect: PassiveEffect::BonusGoldFind(5),
            },
        },
        // Sight + Gold = Reveals gold and bonus gold find
        Recipe {
            required_words: HashSet::from([WordId::Sight, WordId::Gold]),
            effect: ComputedSpell::Passive {
                name: "Golden Eye".to_string(),
                description: "Reveals hidden treasures and increases gold find".to_string(),
                effect: PassiveEffect::BonusGoldFind(10),
            },
        },
        // Earth + Stable = Bonus Defense
        Recipe {
            required_words: HashSet::from([WordId::Earth, WordId::Stable]),
            effect: ComputedSpell::Passive {
                name: "Stone Skin".to_string(),
                description: "Your skin hardens like stone".to_string(),
                effect: PassiveEffect::BonusDefense(3),
            },
        },
        // Power + Swift = Bonus Attack
        Recipe {
            required_words: HashSet::from([WordId::Power, WordId::Swift]),
            effect: ComputedSpell::Passive {
                name: "Battle Fury".to_string(),
                description: "Your attacks are more powerful".to_string(),
                effect: PassiveEffect::BonusAttack(3),
            },
        },
    ]
});

// ─────────────────────────────────────────────────────────────────────────────
// Invalid Combinations (cause backfire)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct InvalidCombo {
    /// The words that form an invalid combination
    pub words: HashSet<WordId>,
    /// What happens when this combo is used
    pub backfire: BackfireEffect,
    /// Why this combination is unstable
    pub reason: &'static str,
}

impl InvalidCombo {
    /// Check if words contain this invalid combination
    pub fn is_present_in(&self, words: &HashSet<WordId>) -> bool {
        self.words.is_subset(words)
    }
}

/// Invalid word combinations that cause backfire
pub static INVALID_COMBOS: Lazy<Vec<InvalidCombo>> = Lazy::new(|| {
    vec![
        // Fire + Ice = Elemental conflict
        InvalidCombo {
            words: HashSet::from([WordId::Fire, WordId::Ice]),
            backfire: BackfireEffect::DamageSelf(10),
            reason: "Opposing elements destabilize the spell",
        },
        // Chaos + Chaos = Too unstable
        InvalidCombo {
            words: HashSet::from([WordId::Chaos]),
            backfire: BackfireEffect::RandomCurse,
            reason: "Double chaos is too unstable",
        },
        // Lightning + Drain = Dangerous feedback
        InvalidCombo {
            words: HashSet::from([WordId::Lightning, WordId::Drain]),
            backfire: BackfireEffect::Stun { turns: 2 },
            reason: "The energy feedback overwhelms you",
        },
    ]
});

/// Find a matching recipe for the given words
pub fn find_recipe(words: &HashSet<WordId>) -> Option<&'static Recipe> {
    RECIPES.iter().find(|r| r.matches(words))
}

/// Check if words contain any invalid combinations
pub fn find_invalid_combo(words: &HashSet<WordId>) -> Option<&'static InvalidCombo> {
    // Special case: Chaos + Chaos requires two chaos words, but we can't have duplicates
    // So we only check for single Chaos if there's nothing else stabilizing it
    INVALID_COMBOS.iter().find(|ic| {
        // For the Chaos case, only trigger if Chaos is the only word or stability is low
        if ic.words == HashSet::from([WordId::Chaos]) {
            // Only backfire if multiple sources of chaos without stability
            // For now, skip this - the design said specific pairs
            return false;
        }
        ic.is_present_in(words)
    })
}
