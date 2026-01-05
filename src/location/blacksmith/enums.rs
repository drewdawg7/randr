use crate::item::{enums::ItemError, recipe::RecipeError, UpgradeResult};

#[derive(Debug)]
pub enum BlacksmithError {
    MaxUpgradesReached,
    NotEnoughGold,
    NoUpgradeStones,
    NotEquipment,
    InvalidBlacksmithRecipe,
    ItemError(ItemError),
    RecipeError(RecipeError),
    NotEnoughFuel,
    NoFuel,
    ItemNotFound,
    InventoryFull,
}

/// Result of a blacksmith upgrade operation, including the item upgrade details and gold spent
#[derive(Debug, Clone)]
pub struct BlacksmithUpgradeResult {
    /// The upgrade result from the item (new level and stat increases)
    pub upgrade: UpgradeResult,
    /// The amount of gold spent on this upgrade
    pub gold_spent: i32,
}
