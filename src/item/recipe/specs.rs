//! Recipe definitions using the entity_macros system
//!
//! This file consolidates:
//! - RecipeSpec struct definition
//! - RecipeId enum
//! - All recipe spec constants
//! - The spec() method on RecipeId

use std::collections::HashMap;

use crate::item::ItemId;
use crate::registry::RegistryDefaults;

// RecipeType and other enums are kept separate
pub use super::enums::{ForgeMaterial, RecipeType};

entity_macros::define_data! {
    spec RecipeSpec {
        pub name: &'static str,
        pub ingredients: HashMap<ItemId, u32>,
        pub output: ItemId,
        pub output_quantity: u32,
        pub recipe_type: RecipeType,
    }

    id RecipeId;

    variants {
        // ─────────────────────────────────────────────────────────────────────
        // Smelting Recipes
        // ─────────────────────────────────────────────────────────────────────
        TinIngot {
            name: "Tin Ingot",
            ingredients: HashMap::from([(ItemId::TinOre, 1)]),
            output: ItemId::TinIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }
        CopperIngot {
            name: "Copper Ingot",
            ingredients: HashMap::from([(ItemId::CopperOre, 1)]),
            output: ItemId::CopperIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }
        BronzeIngot {
            name: "Bronze Ingot",
            ingredients: HashMap::from([(ItemId::CopperOre, 1), (ItemId::TinOre, 1)]),
            output: ItemId::BronzeIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Forging Recipes - Swords
        // ─────────────────────────────────────────────────────────────────────
        BronzeSword {
            name: "Bronze Sword",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 4)]),
            output: ItemId::BronzeSword,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        CopperSword {
            name: "Copper Sword",
            ingredients: HashMap::from([(ItemId::CopperIngot, 4)]),
            output: ItemId::CopperSword,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        TinSword {
            name: "Tin Sword",
            ingredients: HashMap::from([(ItemId::TinIngot, 4)]),
            output: ItemId::TinSword,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Alchemy Recipes
        // ─────────────────────────────────────────────────────────────────────
        BasicHPPotion {
            name: "Basic HP Potion",
            ingredients: HashMap::from([(ItemId::SlimeGel, 10)]),
            output: ItemId::BasicHPPotion,
            output_quantity: 1,
            recipe_type: RecipeType::Alchemy,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Copper Armor Recipes
        // ─────────────────────────────────────────────────────────────────────
        CopperHelmet {
            name: "Copper Helmet",
            ingredients: HashMap::from([(ItemId::CopperIngot, 12)]),
            output: ItemId::CopperHelmet,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        CopperChestplate {
            name: "Copper Chestplate",
            ingredients: HashMap::from([(ItemId::CopperIngot, 20)]),
            output: ItemId::CopperChestplate,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        CopperGauntlets {
            name: "Copper Gauntlets",
            ingredients: HashMap::from([(ItemId::CopperIngot, 8)]),
            output: ItemId::CopperGauntlets,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        CopperGreaves {
            name: "Copper Greaves",
            ingredients: HashMap::from([(ItemId::CopperIngot, 10)]),
            output: ItemId::CopperGreaves,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        CopperLeggings {
            name: "Copper Leggings",
            ingredients: HashMap::from([(ItemId::CopperIngot, 18)]),
            output: ItemId::CopperLeggings,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Tin Armor Recipes
        // ─────────────────────────────────────────────────────────────────────
        TinHelmet {
            name: "Tin Helmet",
            ingredients: HashMap::from([(ItemId::TinIngot, 12)]),
            output: ItemId::TinHelmet,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        TinChestplate {
            name: "Tin Chestplate",
            ingredients: HashMap::from([(ItemId::TinIngot, 20)]),
            output: ItemId::TinChestplate,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        TinGauntlets {
            name: "Tin Gauntlets",
            ingredients: HashMap::from([(ItemId::TinIngot, 8)]),
            output: ItemId::TinGauntlets,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        TinGreaves {
            name: "Tin Greaves",
            ingredients: HashMap::from([(ItemId::TinIngot, 10)]),
            output: ItemId::TinGreaves,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        TinLeggings {
            name: "Tin Leggings",
            ingredients: HashMap::from([(ItemId::TinIngot, 18)]),
            output: ItemId::TinLeggings,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Bronze Armor Recipes
        // ─────────────────────────────────────────────────────────────────────
        BronzeHelmet {
            name: "Bronze Helmet",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 12)]),
            output: ItemId::BronzeHelmet,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        BronzeChestplate {
            name: "Bronze Chestplate",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 20)]),
            output: ItemId::BronzeChestplate,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        BronzeGauntlets {
            name: "Bronze Gauntlets",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 8)]),
            output: ItemId::BronzeGauntlets,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        BronzeGreaves {
            name: "Bronze Greaves",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 10)]),
            output: ItemId::BronzeGreaves,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        BronzeLeggings {
            name: "Bronze Leggings",
            ingredients: HashMap::from([(ItemId::BronzeIngot, 18)]),
            output: ItemId::BronzeLeggings,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RegistryDefaults (no spawn for recipes, just lookup)
// ─────────────────────────────────────────────────────────────────────────────

impl RegistryDefaults<RecipeId> for RecipeSpec {
    fn defaults() -> impl IntoIterator<Item = (RecipeId, Self)> {
        RecipeId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional RecipeId Methods
// ─────────────────────────────────────────────────────────────────────────────

impl RecipeId {
    pub fn all_forging_recipes() -> Vec<RecipeId> {
        RecipeId::ALL
            .iter()
            .filter(|id| id.spec().recipe_type == RecipeType::Forging)
            .copied()
            .collect()
    }

    pub fn all_alchemy_recipes() -> Vec<RecipeId> {
        RecipeId::ALL
            .iter()
            .filter(|id| id.spec().recipe_type == RecipeType::Alchemy)
            .copied()
            .collect()
    }

    pub fn all_smelting_recipes() -> Vec<RecipeId> {
        RecipeId::ALL
            .iter()
            .filter(|id| id.spec().recipe_type == RecipeType::Smelting)
            .copied()
            .collect()
    }

    /// Get the material type for this recipe (for forge filtering)
    pub fn material(&self) -> ForgeMaterial {
        let name = format!("{:?}", self);
        if name.starts_with("Copper") {
            ForgeMaterial::Copper
        } else if name.starts_with("Tin") {
            ForgeMaterial::Tin
        } else if name.starts_with("Bronze") {
            ForgeMaterial::Bronze
        } else {
            ForgeMaterial::Other
        }
    }
}
