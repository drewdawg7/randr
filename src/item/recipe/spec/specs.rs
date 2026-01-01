use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::item::ItemId;

use super::definition::RecipeSpec;

pub static TIN_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Tin Ingot",
    ingredients: HashMap::from([(ItemId::TinOre, 1)]),
    output: ItemId::TinIngot,
    output_quantity: 1,
});

pub static COPPER_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Copper Ingot",
    ingredients: HashMap::from([(ItemId::CopperOre, 1)]),
    output: ItemId::CopperIngot,
    output_quantity: 1,
});

pub static BRONZE_INGOT_RECIPE: Lazy<RecipeSpec> = Lazy::new(|| RecipeSpec {
    name: "Bronze Ingot",
    ingredients: HashMap::from([
        (ItemId::CopperOre, 1),
        (ItemId::TinOre, 1),
    ]),
    output: ItemId::BronzeIngot,
    output_quantity: 1,
});
