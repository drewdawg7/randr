#[derive(Debug)]
pub enum RecipeError {
    NoMatchingRecipe,
    NotEnoughIngredients,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RecipeType {
    Smelting,  // ore to ingot
    Forging,   // crafting items from materials
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RecipeId {
    BronzeIngot,
    TinIngot,
    CopperIngot,
    BronzeSword,
    TinSword,
    CopperSword,
    BasicHPPotion,
}

impl RecipeId {
    pub fn all_forging_recipes() -> Vec<RecipeId> {
        vec![
            RecipeId::BronzeSword,
            RecipeId::TinSword,
            RecipeId::CopperSword,
            RecipeId::BasicHPPotion,
        ]
    }
}
