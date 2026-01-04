use crate::{
    player::Player,
    item::{Item, recipe::{Recipe, RecipeId}},
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
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, _data: &AlchemistData) -> Self {
        Alchemist {
            location_id,
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
    ) -> Result<Item, AlchemistError> {
        let recipe = Recipe::new(*recipe_id).map_err(AlchemistError::RecipeError)?;
        let item_id = recipe.craft(player).map_err(AlchemistError::RecipeError)?;
        let item = item_id.spawn();
        player
            .add_to_inv(item.clone())
            .map_err(|_| AlchemistError::InventoryFull)?;
        Ok(item)
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
