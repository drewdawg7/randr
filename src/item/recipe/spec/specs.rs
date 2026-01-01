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
