mod computation;
mod definition;
mod recipes;

pub use computation::compute_spell;
pub use definition::{BackfireEffect, ComputedSpell, SpellResult};
pub use recipes::{InvalidCombo, Recipe, INVALID_COMBOS, RECIPES};
