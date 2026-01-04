//! Alchemy-related game commands.
//!
//! Handles brewing potions and other alchemical items.

use crate::inventory::HasInventory;
use crate::item::recipe::{Recipe, RecipeId};
use crate::system::game_state;

use super::CommandResult;

/// Brew a recipe at the alchemist.
pub fn brew_recipe(recipe_id: RecipeId) -> CommandResult {
    let gs = game_state();

    match Recipe::new(recipe_id) {
        Ok(recipe) => match recipe.craft(&mut gs.player) {
            Ok(item_id) => {
                let item = item_id.spawn();
                let item_name = item.name;
                match gs.player.add_to_inv(item) {
                    Ok(_) => CommandResult::success(format!("Brewed {}!", item_name)),
                    Err(_) => CommandResult::error("Inventory is full"),
                }
            }
            Err(crate::item::recipe::RecipeError::NotEnoughIngredients) => {
                CommandResult::error("Missing ingredients")
            }
            Err(_) => CommandResult::error("Brewing failed"),
        },
        Err(_) => CommandResult::error("Invalid recipe"),
    }
}
