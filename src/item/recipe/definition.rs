use std::collections::HashMap;

use crate::{
    player::Player,
    item::ItemId,
    inventory::{FindsItems, ManagesItems},
};

use super::definitions::{RecipeId, RecipeSpec};
use super::enums::RecipeError;

pub struct Recipe {
    spec: RecipeSpec,
}

impl Recipe {
    pub fn new(recipe_id: RecipeId) -> Result<Self, RecipeError> {
        let spec = recipe_id.spec().clone();
        Ok(Self { spec })
    }

    pub fn name(&self) -> &'static str {
        self.spec.name
    }

    pub fn ingredients(&self) -> &HashMap<ItemId, u32> {
        &self.spec.ingredients
    }

    #[allow(dead_code)]
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

    /// Consumes ingredients from player inventory and returns the ItemId to spawn.
    /// The caller is responsible for spawning the item using an ItemRegistry.
    pub fn craft(&self, player: &mut Player) -> Result<ItemId, RecipeError> {
        if !self.can_craft(player) {
            return Err(RecipeError::NotEnoughIngredients);
        }

        for (&item_id, &qty) in &self.spec.ingredients {
            if let Some(inv_item) = player.find_item_by_id(item_id).cloned() {
                player.decrease_item_quantity(&inv_item, qty);
            }
        }

        Ok(self.spec.output)
    }
}
