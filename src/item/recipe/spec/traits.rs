use crate::item::recipe::spec::specs::{BASIC_HP_POTION_RECIPE, BRONZE_SWORD_RECIPE, COPPER_SWORD_RECIPE, TIN_SWORD_RECIPE};
use crate::registry::RegistryDefaults;

use super::super::enums::RecipeId;
use super::definition::RecipeSpec;
use super::specs::{
    BRONZE_INGOT_RECIPE, COPPER_INGOT_RECIPE, TIN_INGOT_RECIPE,
    // Copper Armor Recipes
    COPPER_HELMET_RECIPE, COPPER_CHESTPLATE_RECIPE, COPPER_GAUNTLETS_RECIPE,
    COPPER_GREAVES_RECIPE, COPPER_LEGGINGS_RECIPE,
    // Tin Armor Recipes
    TIN_HELMET_RECIPE, TIN_CHESTPLATE_RECIPE, TIN_GAUNTLETS_RECIPE,
    TIN_GREAVES_RECIPE, TIN_LEGGINGS_RECIPE,
    // Bronze Armor Recipes
    BRONZE_HELMET_RECIPE, BRONZE_CHESTPLATE_RECIPE, BRONZE_GAUNTLETS_RECIPE,
    BRONZE_GREAVES_RECIPE, BRONZE_LEGGINGS_RECIPE,
};

impl RegistryDefaults<RecipeId> for RecipeSpec {
    fn defaults() -> impl IntoIterator<Item = (RecipeId, Self)> {
        [
            (RecipeId::TinIngot, TIN_INGOT_RECIPE.clone()),
            (RecipeId::CopperIngot, COPPER_INGOT_RECIPE.clone()),
            (RecipeId::BronzeIngot, BRONZE_INGOT_RECIPE.clone()),
            (RecipeId::TinSword, TIN_SWORD_RECIPE.clone()),
            (RecipeId::CopperSword, COPPER_SWORD_RECIPE.clone()),
            (RecipeId::BronzeSword, BRONZE_SWORD_RECIPE.clone()),
            (RecipeId::BasicHPPotion, BASIC_HP_POTION_RECIPE.clone()),
            // Copper Armor
            (RecipeId::CopperHelmet, COPPER_HELMET_RECIPE.clone()),
            (RecipeId::CopperChestplate, COPPER_CHESTPLATE_RECIPE.clone()),
            (RecipeId::CopperGauntlets, COPPER_GAUNTLETS_RECIPE.clone()),
            (RecipeId::CopperGreaves, COPPER_GREAVES_RECIPE.clone()),
            (RecipeId::CopperLeggings, COPPER_LEGGINGS_RECIPE.clone()),
            // Tin Armor
            (RecipeId::TinHelmet, TIN_HELMET_RECIPE.clone()),
            (RecipeId::TinChestplate, TIN_CHESTPLATE_RECIPE.clone()),
            (RecipeId::TinGauntlets, TIN_GAUNTLETS_RECIPE.clone()),
            (RecipeId::TinGreaves, TIN_GREAVES_RECIPE.clone()),
            (RecipeId::TinLeggings, TIN_LEGGINGS_RECIPE.clone()),
            // Bronze Armor
            (RecipeId::BronzeHelmet, BRONZE_HELMET_RECIPE.clone()),
            (RecipeId::BronzeChestplate, BRONZE_CHESTPLATE_RECIPE.clone()),
            (RecipeId::BronzeGauntlets, BRONZE_GAUNTLETS_RECIPE.clone()),
            (RecipeId::BronzeGreaves, BRONZE_GREAVES_RECIPE.clone()),
            (RecipeId::BronzeLeggings, BRONZE_LEGGINGS_RECIPE.clone()),
        ]
    }
}
