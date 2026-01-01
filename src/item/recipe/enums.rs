#[derive(Debug)]
pub enum RecipeError {
    NoMatchingRecipe,
    NotEnoughIngredients,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RecipeId {
    BronzeIngot,
    TinIngot,
    CopperIngot,
}
