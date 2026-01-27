#[derive(Debug)]
pub enum RecipeError {
    NoMatchingRecipe,
    NotEnoughIngredients,
}

/// Material type for forge filtering
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ForgeMaterial {
    Iron,
    Gold,
    Bronze,
    #[default]
    Other,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RecipeType {
    Smelting,  // ore to ingot
    Forging,   // crafting items from materials
    Alchemy,   // brewing potions
}
