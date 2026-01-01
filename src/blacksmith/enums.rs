use crate::item::{enums::ItemError, recipe::RecipeError};
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
