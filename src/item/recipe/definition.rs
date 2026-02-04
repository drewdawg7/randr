use std::collections::HashMap;

use crate::{
    item::ItemId,
    inventory::{FindsItems, ManagesItems},
};

use super::specs::{RecipeId, RecipeSpec};
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

    pub fn output_item_id(&self) -> ItemId {
        self.spec.output
    }

    pub fn can_craft<T: FindsItems>(&self, inventory: &T) -> bool {
        self.spec.ingredients.iter().all(|(&item_id, &qty)| {
            inventory
                .find_item_by_id(item_id)
                .map(|inv| inv.quantity >= qty)
                .unwrap_or(false)
        })
    }

    /// Consumes ingredients from inventory and returns the ItemId to spawn.
    /// The caller is responsible for spawning the item using an ItemRegistry.
    pub fn craft<T: FindsItems + ManagesItems>(&self, inventory: &mut T) -> Result<ItemId, RecipeError> {
        if !self.can_craft(inventory) {
            return Err(RecipeError::NotEnoughIngredients);
        }

        for (&item_id, &qty) in &self.spec.ingredients {
            if inventory.find_item_by_id(item_id).is_some() {
                inventory.decrease_item_quantity(item_id, qty);
            }
        }

        Ok(self.spec.output)
    }
}
