//! Recipe definitions using the entity_macros system
//!
//! This file consolidates:
//! - RecipeSpec struct definition
//! - RecipeId enum
//! - All recipe spec constants
//! - The spec() method on RecipeId

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::item::ItemId;
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
        GoldIngot {
            name: "Gold Ingot",
            ingredients: HashMap::from([(ItemId::GoldOre, 1)]),
            output: ItemId::GoldIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }
        IronIngot {
            name: "Iron Ingot",
            ingredients: HashMap::from([(ItemId::IronOre, 1)]),
            output: ItemId::IronIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }
        CopperIngot {
            name: "Copper Ingot",
            ingredients: HashMap::from([(ItemId::IronOre, 1), (ItemId::GoldOre, 1)]),
            output: ItemId::CopperIngot,
            output_quantity: 1,
            recipe_type: RecipeType::Smelting,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Forging Recipes - Swords
        // ─────────────────────────────────────────────────────────────────────
        CopperSword {
            name: "Copper Sword",
            ingredients: HashMap::from([(ItemId::CopperIngot, 4)]),
            output: ItemId::CopperSword,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        IronSword {
            name: "Iron Sword",
            ingredients: HashMap::from([(ItemId::IronIngot, 4)]),
            output: ItemId::IronSword,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        GoldSword {
            name: "Gold Sword",
            ingredients: HashMap::from([(ItemId::GoldIngot, 4)]),
            output: ItemId::GoldSword,
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
        // Iron Armor Recipes
        // ─────────────────────────────────────────────────────────────────────
        IronHelmet {
            name: "Iron Helmet",
            ingredients: HashMap::from([(ItemId::IronIngot, 12)]),
            output: ItemId::IronHelmet,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        IronChestplate {
            name: "Iron Chestplate",
            ingredients: HashMap::from([(ItemId::IronIngot, 20)]),
            output: ItemId::IronChestplate,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        IronGauntlets {
            name: "Iron Gauntlets",
            ingredients: HashMap::from([(ItemId::IronIngot, 8)]),
            output: ItemId::IronGauntlets,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        IronGreaves {
            name: "Iron Greaves",
            ingredients: HashMap::from([(ItemId::IronIngot, 10)]),
            output: ItemId::IronGreaves,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        IronLeggings {
            name: "Iron Leggings",
            ingredients: HashMap::from([(ItemId::IronIngot, 18)]),
            output: ItemId::IronLeggings,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }

        // ─────────────────────────────────────────────────────────────────────
        // Gold Armor Recipes
        // ─────────────────────────────────────────────────────────────────────
        GoldHelmet {
            name: "Gold Helmet",
            ingredients: HashMap::from([(ItemId::GoldIngot, 12)]),
            output: ItemId::GoldHelmet,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        GoldChestplate {
            name: "Gold Chestplate",
            ingredients: HashMap::from([(ItemId::GoldIngot, 20)]),
            output: ItemId::GoldChestplate,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        GoldGauntlets {
            name: "Gold Gauntlets",
            ingredients: HashMap::from([(ItemId::GoldIngot, 8)]),
            output: ItemId::GoldGauntlets,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        GoldGreaves {
            name: "Gold Greaves",
            ingredients: HashMap::from([(ItemId::GoldIngot, 10)]),
            output: ItemId::GoldGreaves,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
        }
        GoldLeggings {
            name: "Gold Leggings",
            ingredients: HashMap::from([(ItemId::GoldIngot, 18)]),
            output: ItemId::GoldLeggings,
            output_quantity: 1,
            recipe_type: RecipeType::Forging,
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
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cached Recipe Lists (lazy-initialized once, returned as static slices)
// ─────────────────────────────────────────────────────────────────────────────

static FORGING_RECIPES: LazyLock<Vec<RecipeId>> = LazyLock::new(|| {
    RecipeId::ALL
        .iter()
        .filter(|id| id.spec().recipe_type == RecipeType::Forging)
        .copied()
        .collect()
});

static ALCHEMY_RECIPES: LazyLock<Vec<RecipeId>> = LazyLock::new(|| {
    RecipeId::ALL
        .iter()
        .filter(|id| id.spec().recipe_type == RecipeType::Alchemy)
        .copied()
        .collect()
});

static SMELTING_RECIPES: LazyLock<Vec<RecipeId>> = LazyLock::new(|| {
    RecipeId::ALL
        .iter()
        .filter(|id| id.spec().recipe_type == RecipeType::Smelting)
        .copied()
        .collect()
});

// ─────────────────────────────────────────────────────────────────────────────
// Additional RecipeId Methods
// ─────────────────────────────────────────────────────────────────────────────

impl RecipeId {
    pub fn all_forging_recipes() -> &'static [RecipeId] {
        &FORGING_RECIPES
    }

    pub fn all_alchemy_recipes() -> &'static [RecipeId] {
        &ALCHEMY_RECIPES
    }

    pub fn all_smelting_recipes() -> &'static [RecipeId] {
        &SMELTING_RECIPES
    }

    /// Get the material type for this recipe (for forge filtering)
    pub fn material(&self) -> ForgeMaterial {
        match self {
            RecipeId::IronIngot
            | RecipeId::IronSword
            | RecipeId::IronHelmet
            | RecipeId::IronChestplate
            | RecipeId::IronGauntlets
            | RecipeId::IronGreaves
            | RecipeId::IronLeggings => ForgeMaterial::Iron,

            RecipeId::GoldIngot
            | RecipeId::GoldSword
            | RecipeId::GoldHelmet
            | RecipeId::GoldChestplate
            | RecipeId::GoldGauntlets
            | RecipeId::GoldGreaves
            | RecipeId::GoldLeggings => ForgeMaterial::Gold,

            RecipeId::CopperIngot
            | RecipeId::CopperSword
            | RecipeId::CopperHelmet
            | RecipeId::CopperChestplate
            | RecipeId::CopperGauntlets
            | RecipeId::CopperGreaves
            | RecipeId::CopperLeggings => ForgeMaterial::Bronze,

            RecipeId::BasicHPPotion => ForgeMaterial::Other,
        }
    }
}
