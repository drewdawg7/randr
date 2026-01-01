use crate::item::recipe::RecipeError;

#[derive(Debug)]
pub enum AlchemistError {
    RecipeError(RecipeError),
    InventoryFull,
}
