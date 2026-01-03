use crate::{
    entities::Player,
    game_state,
    item::recipe::{Recipe, RecipeId},
    location::{AlchemistData, LocationId, LocationSpec},
    HasInventory,
};

use super::enums::AlchemistError;

pub struct Alchemist {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
}

impl Alchemist {
    pub fn from_spec(spec: &LocationSpec, _data: &AlchemistData) -> Self {
        Alchemist {
            location_id: spec.location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
        }
    }

    pub fn new(name: String) -> Self {
        Self {
            location_id: LocationId::VillageAlchemist,
            name,
            description: String::new(),
        }
    }

    pub fn brew_potion(
        &self,
        player: &mut Player,
        recipe_id: &RecipeId,
    ) -> Result<(), AlchemistError> {
        let recipe = Recipe::new(*recipe_id).map_err(AlchemistError::RecipeError)?;
        let item_id = recipe.craft(player).map_err(AlchemistError::RecipeError)?;
        let item = game_state()
            .item_registry()
            .spawn(item_id)
            .ok_or(AlchemistError::RecipeError(
                crate::item::recipe::RecipeError::NoMatchingRecipe,
            ))?;
        player
            .add_to_inv(item)
            .map_err(|_| AlchemistError::InventoryFull)?;
        Ok(())
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
