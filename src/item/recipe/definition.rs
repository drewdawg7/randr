use std::collections::HashMap;

use crate::{
    entities::Player,
    game_state,
    item::{recipe::enums::{RecipeError, RecipeId}, Item},
    HasInventory,
    ItemId,
};

use super::spec::RecipeSpec;

pub struct Recipe {
    spec: RecipeSpec,
}

impl Recipe {
    pub fn new(recipe_id: RecipeId) -> Result<Self, RecipeError> {
        let spec = game_state()
            .recipe_registry()
            .get(&recipe_id)
            .cloned()
            .ok_or(RecipeError::NoMatchingRecipe)?;
        Ok(Self { spec })
    }

    pub fn name(&self) -> &'static str {
        self.spec.name
    }

    pub fn ingredients(&self) -> &HashMap<ItemId, u32> {
        &self.spec.ingredients
    }

    pub fn output_item_id(&self) -> ItemId {
        self.spec.output
    }

    pub fn can_craft(&self, player: &Player) -> bool {
        self.spec.ingredients.iter().all(|(&item_id, &qty)| {
            player
                .find_item_by_id(item_id)
                .map(|inv| inv.quantity >= qty)
                .unwrap_or(false)
        })
    }

    pub fn craft(&self, player: &mut Player) -> Result<Item, RecipeError> {
        if !self.can_craft(player) {
            return Err(RecipeError::NotEnoughIngredients);
        }

        for (&item_id, &qty) in &self.spec.ingredients {
            if let Some(inv_item) = player.find_item_by_id(item_id).cloned() {
                player.decrease_item_quantity(&inv_item, qty);
            }
        }

        Ok(game_state().spawn_item(self.spec.output))
    }
}
