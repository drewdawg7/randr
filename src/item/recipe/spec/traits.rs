use crate::registry::RegistryDefaults;

use super::super::enums::RecipeId;
use super::definition::RecipeSpec;
use super::specs::{BRONZE_INGOT_RECIPE, COPPER_INGOT_RECIPE, TIN_INGOT_RECIPE};

impl RegistryDefaults<RecipeId> for RecipeSpec {
    fn defaults() -> impl IntoIterator<Item = (RecipeId, Self)> {
        [
            (RecipeId::TinIngot, TIN_INGOT_RECIPE.clone()),
            (RecipeId::CopperIngot, COPPER_INGOT_RECIPE.clone()),
            (RecipeId::BronzeIngot, BRONZE_INGOT_RECIPE.clone()),
        ]
    }
}
