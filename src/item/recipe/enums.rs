#[derive(Debug)]
pub enum RecipeError {
    NoMatchingRecipe,
    NotEnoughIngredients,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RecipeType {
    Smelting,  // ore to ingot
    Forging,   // crafting items from materials
    Alchemy,   // brewing potions
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
    // Copper Armor Recipes
    CopperHelmet,
    CopperChestplate,
    CopperGauntlets,
    CopperGreaves,
    CopperLeggings,
    // Tin Armor Recipes
    TinHelmet,
    TinChestplate,
    TinGauntlets,
    TinGreaves,
    TinLeggings,
    // Bronze Armor Recipes
    BronzeHelmet,
    BronzeChestplate,
    BronzeGauntlets,
    BronzeGreaves,
    BronzeLeggings,
}

impl RecipeId {
    pub fn all_forging_recipes() -> Vec<RecipeId> {
        vec![
            // Swords
            RecipeId::BronzeSword,
            RecipeId::TinSword,
            RecipeId::CopperSword,
            // Copper Armor
            RecipeId::CopperHelmet,
            RecipeId::CopperChestplate,
            RecipeId::CopperGauntlets,
            RecipeId::CopperGreaves,
            RecipeId::CopperLeggings,
            // Tin Armor
            RecipeId::TinHelmet,
            RecipeId::TinChestplate,
            RecipeId::TinGauntlets,
            RecipeId::TinGreaves,
            RecipeId::TinLeggings,
            // Bronze Armor
            RecipeId::BronzeHelmet,
            RecipeId::BronzeChestplate,
            RecipeId::BronzeGauntlets,
            RecipeId::BronzeGreaves,
            RecipeId::BronzeLeggings,
        ]
    }

    pub fn all_alchemy_recipes() -> Vec<RecipeId> {
        vec![RecipeId::BasicHPPotion]
    }
}
