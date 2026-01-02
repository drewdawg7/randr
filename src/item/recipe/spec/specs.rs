use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::item::ItemId;
use crate::item::recipe::enums::RecipeType;

use super::definition::RecipeSpec;

pub static TIN_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Ingot",
    ingredients: HashMap::from([(ItemId::TinOre, 1)]),
    output: ItemId::TinIngot,
    output_quantity: 1,
    recipe_type: RecipeType::Smelting,
});

pub static COPPER_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Ingot",
    ingredients: HashMap::from([(ItemId::CopperOre, 1)]),
    output: ItemId::CopperIngot,
    output_quantity: 1,
    recipe_type: RecipeType::Smelting,
});

pub static BRONZE_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Ingot",
    ingredients: HashMap::from([
        (ItemId::CopperOre, 1),
        (ItemId::TinOre, 1),
    ]),
    output: ItemId::BronzeIngot,
    output_quantity: 1,
    recipe_type: RecipeType::Smelting,
});


pub static BRONZE_SWORD_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Sword",
    ingredients: HashMap::from([
        (ItemId::BronzeIngot, 4),
    ]),
    output: ItemId::BronzeSword,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});


pub static COPPER_SWORD_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Sword",
    ingredients: HashMap::from([
        (ItemId::CopperIngot, 4),
    ]),
    output: ItemId::CopperSword,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static TIN_SWORD_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Sword",
    ingredients: HashMap::from([
        (ItemId::TinIngot, 4),
    ]),
    output: ItemId::TinSword,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static BASIC_HP_POTION_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Basic HP Potion",
    ingredients: HashMap::from([
        (ItemId::SlimeGel, 10),
    ]),
    output: ItemId::BasicHPPotion,
    output_quantity: 1,
    recipe_type: RecipeType::Alchemy,
});

// ─────────────────────────────────────────────────────────────────────────────
// Copper Armor Recipes
// ─────────────────────────────────────────────────────────────────────────────

pub static COPPER_HELMET_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Helmet",
    ingredients: HashMap::from([(ItemId::CopperIngot, 12)]),
    output: ItemId::CopperHelmet,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static COPPER_CHESTPLATE_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Chestplate",
    ingredients: HashMap::from([(ItemId::CopperIngot, 20)]),
    output: ItemId::CopperChestplate,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static COPPER_GAUNTLETS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Gauntlets",
    ingredients: HashMap::from([(ItemId::CopperIngot, 8)]),
    output: ItemId::CopperGauntlets,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static COPPER_GREAVES_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Greaves",
    ingredients: HashMap::from([(ItemId::CopperIngot, 10)]),
    output: ItemId::CopperGreaves,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static COPPER_LEGGINGS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Leggings",
    ingredients: HashMap::from([(ItemId::CopperIngot, 18)]),
    output: ItemId::CopperLeggings,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

// ─────────────────────────────────────────────────────────────────────────────
// Tin Armor Recipes
// ─────────────────────────────────────────────────────────────────────────────

pub static TIN_HELMET_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Helmet",
    ingredients: HashMap::from([(ItemId::TinIngot, 12)]),
    output: ItemId::TinHelmet,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static TIN_CHESTPLATE_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Chestplate",
    ingredients: HashMap::from([(ItemId::TinIngot, 20)]),
    output: ItemId::TinChestplate,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static TIN_GAUNTLETS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Gauntlets",
    ingredients: HashMap::from([(ItemId::TinIngot, 8)]),
    output: ItemId::TinGauntlets,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static TIN_GREAVES_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Greaves",
    ingredients: HashMap::from([(ItemId::TinIngot, 10)]),
    output: ItemId::TinGreaves,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static TIN_LEGGINGS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Leggings",
    ingredients: HashMap::from([(ItemId::TinIngot, 18)]),
    output: ItemId::TinLeggings,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

// ─────────────────────────────────────────────────────────────────────────────
// Bronze Armor Recipes
// ─────────────────────────────────────────────────────────────────────────────

pub static BRONZE_HELMET_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Helmet",
    ingredients: HashMap::from([(ItemId::BronzeIngot, 12)]),
    output: ItemId::BronzeHelmet,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static BRONZE_CHESTPLATE_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Chestplate",
    ingredients: HashMap::from([(ItemId::BronzeIngot, 20)]),
    output: ItemId::BronzeChestplate,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static BRONZE_GAUNTLETS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Gauntlets",
    ingredients: HashMap::from([(ItemId::BronzeIngot, 8)]),
    output: ItemId::BronzeGauntlets,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static BRONZE_GREAVES_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Greaves",
    ingredients: HashMap::from([(ItemId::BronzeIngot, 10)]),
    output: ItemId::BronzeGreaves,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});

pub static BRONZE_LEGGINGS_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Leggings",
    ingredients: HashMap::from([(ItemId::BronzeIngot, 18)]),
    output: ItemId::BronzeLeggings,
    output_quantity: 1,
    recipe_type: RecipeType::Forging,
});
